//! WebSocket transport for [`Client::execute`] (the `websocket` feature).
//!
//! Connects to ComfyUI's `/ws` endpoint to receive live progress events and
//! previews. The socket is connected *before* the prompt is queued so that no
//! early events are missed, and final outputs are always reconciled from the
//! history endpoint (via [`super::final_events`]) so a dropped socket never
//! costs us the results.

use std::collections::HashMap;

use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use futures::Stream;
use serde::{Deserialize, Serialize};

use super::{final_events, Event, Execution, PreviewImage, PreviewImageFormat};
use crate::{workflow::WorkflowNodeId, Client, ClientError, Result, Workflow};

/// How long to wait for the socket to open before falling back to polling.
const OPEN_TIMEOUT: web_time::Duration = web_time::Duration::from_secs(5);
/// How often to poll the (non-blocking) receiver for new events.
const RECV_POLL_INTERVAL: web_time::Duration = web_time::Duration::from_millis(20);

impl Client {
    /// Attempt to run `workflow` over a WebSocket connection.
    ///
    /// Returns `Ok(None)` if the socket couldn't be established (so the caller
    /// falls back to polling); `Err` only for genuine errors like a rejected
    /// prompt.
    pub(super) async fn try_execute_websocket(
        &self,
        workflow: &Workflow,
    ) -> Result<Option<Execution>> {
        let url = self.websocket_url();
        let (mut sender, receiver) = match ewebsock::connect(url, ewebsock::Options::default()) {
            Ok(pair) => pair,
            Err(_) => return Ok(None),
        };

        // Wait for the connection to open before queueing, so the server has
        // registered our client ID and routes the prompt's events to us.
        let Some(receiver) = wait_for_open(receiver).await else {
            return Ok(None);
        };

        // Advertise our supported features as the very first message. Recent
        // ComfyUI streams the richer preview format to clients that opt into
        // `supports_preview_metadata`; without it, it uses the legacy format.
        #[derive(Serialize)]
        #[serde(tag = "type", content = "data", rename_all = "snake_case")]
        enum ClientMessage {
            FeatureFlags { supports_preview_metadata: bool },
        }
        if let Ok(handshake) = serde_json::to_string(&ClientMessage::FeatureFlags {
            supports_preview_metadata: true,
        }) {
            sender.send(WsMessage::Text(handshake));
        }

        // Request live sampler previews for this prompt. The server's preview
        // method defaults to "none", so without this no preview frames are
        // generated at all.
        #[derive(Serialize)]
        struct ExtraData {
            preview_method: &'static str,
        }
        let prompt_id = self
            .queue_prompt_with_extra_data(
                workflow,
                ExtraData {
                    preview_method: "auto",
                },
            )
            .await?
            .prompt_id;

        let stream = Box::pin(ws_execution(
            self.clone(),
            sender,
            receiver,
            prompt_id.clone(),
        ));
        Ok(Some(Execution { prompt_id, stream }))
    }

    /// The `ws(s)://.../ws?clientId=...` URL for this client.
    fn websocket_url(&self) -> String {
        let base = self
            .api_base
            .replacen("https://", "wss://", 1)
            .replacen("http://", "ws://", 1);
        format!("{base}/ws?clientId={}", self.client_id)
    }
}

/// Drains the receiver until the socket opens, returning it on success or `None`
/// if it errored, closed, or timed out (callers fall back to polling).
///
/// Takes ownership of (and returns) the receiver rather than borrowing it: an
/// `&WsReceiver` held across an `.await` would make the caller's future `!Send`
/// (the receiver is `!Sync`), but owning it across an await is fine.
async fn wait_for_open(receiver: WsReceiver) -> Option<WsReceiver> {
    let start = web_time::Instant::now();
    loop {
        match receiver.try_recv() {
            Some(WsEvent::Opened) => return Some(receiver),
            Some(WsEvent::Error(_)) | Some(WsEvent::Closed) => return None,
            _ => {}
        }
        if start.elapsed() > OPEN_TIMEOUT {
            return None;
        }
        futures_timer::Delay::new(RECV_POLL_INTERVAL).await;
    }
}

/// The WebSocket-backed [`Event`] stream.
///
/// `sender` is held only to keep the socket alive for the duration of the stream.
fn ws_execution(
    client: Client,
    sender: WsSender,
    receiver: WsReceiver,
    prompt_id: String,
) -> impl Stream<Item = Result<Event>> {
    async_stream::try_stream! {
        let _sender = sender;
        loop {
            match receiver.try_recv() {
                Some(WsEvent::Message(WsMessage::Text(text))) => {
                    match parse_text(&text, &prompt_id) {
                        Parsed::Event(event) => yield event,
                        Parsed::Done => {
                            for event in final_events(&client, &prompt_id).await? {
                                yield event;
                            }
                            break;
                        }
                        Parsed::Failed { node, message } => {
                            yield Event::Error { prompt_id: prompt_id.clone(), node, message };
                            break;
                        }
                        Parsed::Ignore => {}
                    }
                }
                Some(WsEvent::Message(WsMessage::Binary(bytes))) => {
                    if let Some(image) = parse_preview(&bytes) {
                        yield Event::Preview { prompt_id: prompt_id.clone(), image };
                    }
                }
                // The socket dropped before completion: reconcile via history.
                Some(WsEvent::Error(error)) => {
                    Err(ClientError::WebSocket(error))?;
                }
                Some(WsEvent::Closed) => {
                    for event in final_events(&client, &prompt_id).await? {
                        yield event;
                    }
                    break;
                }
                // Ping/pong/unknown messages and a re-`Opened` are not relevant.
                Some(_) => {}
                None => futures_timer::Delay::new(RECV_POLL_INTERVAL).await,
            }
        }
    }
}

/// The result of parsing a text frame.
#[derive(Debug)]
enum Parsed {
    /// Emit this event.
    Event(Event),
    /// The prompt finished successfully; reconcile outputs from history.
    Done,
    /// The prompt failed.
    Failed {
        node: Option<WorkflowNodeId>,
        message: String,
    },
    /// Nothing to emit (not for us, or not a relevant event type).
    Ignore,
}

/// Parses a ComfyUI WebSocket text frame into a [`Parsed`] outcome.
///
/// Frames are `{ "type": ..., "data": {...} }`; only the variants we act on are
/// modelled, and anything else deserialises to `Other` and is ignored. Events
/// carrying a `prompt_id` that isn't ours are ignored too.
fn parse_text(text: &str, our_prompt_id: &str) -> Parsed {
    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data", rename_all = "snake_case")]
    enum ServerMessage {
        Status {
            #[serde(default)]
            status: StatusInfo,
        },
        ExecutionStart {
            prompt_id: String,
        },
        Executing {
            #[serde(default)]
            node: Option<String>,
            prompt_id: String,
        },
        /// Legacy single-node progress (older ComfyUI).
        Progress {
            #[serde(default)]
            value: f64,
            #[serde(default)]
            max: f64,
            #[serde(default)]
            node: Option<String>,
            prompt_id: String,
        },
        /// Newer per-node progress.
        ProgressState {
            prompt_id: String,
            #[serde(default)]
            nodes: HashMap<String, NodeProgress>,
        },
        ExecutionSuccess {
            prompt_id: String,
        },
        ExecutionError {
            prompt_id: String,
            #[serde(default)]
            node_id: Option<String>,
            #[serde(default)]
            exception_message: Option<String>,
        },
        ExecutionInterrupted {
            #[serde(default)]
            prompt_id: Option<String>,
            #[serde(default)]
            node_id: Option<String>,
        },
        #[serde(other)]
        Other,
    }
    #[derive(Deserialize, Default)]
    struct StatusInfo {
        #[serde(default)]
        exec_info: ExecInfo,
    }
    #[derive(Deserialize, Default)]
    struct ExecInfo {
        #[serde(default)]
        queue_remaining: u32,
    }
    #[derive(Deserialize)]
    struct NodeProgress {
        #[serde(default)]
        value: f64,
        #[serde(default)]
        max: f64,
        #[serde(default)]
        state: String,
        #[serde(default)]
        node_id: Option<String>,
    }

    let Ok(message) = serde_json::from_str::<ServerMessage>(text) else {
        return Parsed::Ignore;
    };
    let ours = |prompt_id: &str| prompt_id == our_prompt_id;

    match message {
        // `status` has no prompt id and is always relevant.
        ServerMessage::Status { status } => Parsed::Event(Event::Status {
            queue_remaining: status.exec_info.queue_remaining,
        }),
        ServerMessage::ExecutionStart { prompt_id } if ours(&prompt_id) => {
            Parsed::Event(Event::ExecutionStart { prompt_id })
        }
        ServerMessage::Executing { node, prompt_id } if ours(&prompt_id) => match node {
            // A null/absent node signals the prompt has finished executing.
            None => Parsed::Done,
            Some(node) => Parsed::Event(Event::Executing {
                node: node.parse().ok(),
                prompt_id,
            }),
        },
        ServerMessage::Progress {
            value,
            max,
            node,
            prompt_id,
        } if ours(&prompt_id) => Parsed::Event(Event::Progress {
            node: parse_node_id(node.as_deref()),
            value: value as u32,
            max: max as u32,
            prompt_id,
        }),
        // Surface the actively-running node, which is the one reporting
        // fine-grained progress (e.g. the sampler).
        ServerMessage::ProgressState { prompt_id, nodes } if ours(&prompt_id) => {
            match nodes.values().find(|n| n.state == "running" && n.max > 0.0) {
                Some(node) => Parsed::Event(Event::Progress {
                    node: parse_node_id(node.node_id.as_deref()),
                    value: node.value as u32,
                    max: node.max as u32,
                    prompt_id,
                }),
                None => Parsed::Ignore,
            }
        }
        ServerMessage::ExecutionSuccess { prompt_id } if ours(&prompt_id) => Parsed::Done,
        ServerMessage::ExecutionError {
            prompt_id,
            node_id,
            exception_message,
        } if ours(&prompt_id) => Parsed::Failed {
            node: parse_node_id(node_id.as_deref()),
            message: exception_message.unwrap_or_else(|| "execution error".to_string()),
        },
        ServerMessage::ExecutionInterrupted { prompt_id, node_id }
            if prompt_id.as_deref() == Some(our_prompt_id) =>
        {
            Parsed::Failed {
                node: parse_node_id(node_id.as_deref()),
                message: "execution interrupted".to_string(),
            }
        }
        _ => Parsed::Ignore,
    }
}

/// Parses a node ID string into a [`WorkflowNodeId`].
fn parse_node_id(node: Option<&str>) -> Option<WorkflowNodeId> {
    node.and_then(|s| s.parse().ok())
}

/// Parses a binary preview frame.
///
/// Both ComfyUI preview framings are supported. Each starts with a big-endian
/// `u32` event type:
/// - `1` (`PREVIEW_IMAGE`): `[u32 event][u32 image format][image bytes]`.
/// - `4` (`PREVIEW_IMAGE_WITH_METADATA`): `[u32 event][u32 metadata len][JSON
///   metadata][image bytes]`, where the metadata's `image_type` gives the MIME
///   type. Recent ComfyUI only sends this variant (and only to clients that
///   advertised `supports_preview_metadata`).
fn parse_preview(bytes: &[u8]) -> Option<PreviewImage> {
    if bytes.len() < 8 {
        return None;
    }
    let event_type = u32::from_be_bytes(bytes[0..4].try_into().ok()?);
    match event_type {
        1 => {
            let format = match u32::from_be_bytes(bytes[4..8].try_into().ok()?) {
                1 => PreviewImageFormat::Jpeg,
                2 => PreviewImageFormat::Png,
                _ => PreviewImageFormat::Unknown,
            };
            Some(PreviewImage {
                format,
                data: bytes[8..].to_vec(),
            })
        }
        4 => {
            #[derive(Deserialize)]
            struct PreviewMetadata {
                #[serde(default)]
                image_type: Option<String>,
            }

            let metadata_len = u32::from_be_bytes(bytes[4..8].try_into().ok()?) as usize;
            let image_start = 8usize.checked_add(metadata_len)?;
            let metadata = bytes.get(8..image_start)?;
            let image = bytes.get(image_start..)?;
            let format = serde_json::from_slice::<PreviewMetadata>(metadata)
                .ok()
                .and_then(|m| m.image_type)
                .map(|mime| format_from_mime(&mime))
                .unwrap_or(PreviewImageFormat::Unknown);
            Some(PreviewImage {
                format,
                data: image.to_vec(),
            })
        }
        _ => None,
    }
}

/// Maps a MIME type (from preview metadata) to a [`PreviewImageFormat`].
fn format_from_mime(mime: &str) -> PreviewImageFormat {
    match mime {
        "image/jpeg" => PreviewImageFormat::Jpeg,
        "image/png" => PreviewImageFormat::Png,
        _ => PreviewImageFormat::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Legacy `PREVIEW_IMAGE` framing: `[u32 event=1][u32 format][image]`.
    #[test]
    fn parse_preview_legacy() {
        let mut frame = Vec::new();
        frame.extend_from_slice(&1u32.to_be_bytes()); // event
        frame.extend_from_slice(&1u32.to_be_bytes()); // JPEG
        frame.extend_from_slice(b"jpegbytes");
        let preview = parse_preview(&frame).unwrap();
        assert_eq!(preview.format, PreviewImageFormat::Jpeg);
        assert_eq!(preview.data, b"jpegbytes");
    }

    /// `PREVIEW_IMAGE_WITH_METADATA` framing:
    /// `[u32 event=4][u32 metadata_len][JSON][image]`.
    #[test]
    fn parse_preview_with_metadata() {
        let metadata = br#"{"image_type":"image/png","node_id":"7"}"#;
        let mut frame = Vec::new();
        frame.extend_from_slice(&4u32.to_be_bytes());
        frame.extend_from_slice(&(metadata.len() as u32).to_be_bytes());
        frame.extend_from_slice(metadata);
        frame.extend_from_slice(b"pngbytes");
        let preview = parse_preview(&frame).unwrap();
        assert_eq!(preview.format, PreviewImageFormat::Png);
        assert_eq!(preview.data, b"pngbytes");
    }

    /// A truncated metadata length must not panic.
    #[test]
    fn parse_preview_truncated_metadata() {
        let mut frame = Vec::new();
        frame.extend_from_slice(&4u32.to_be_bytes());
        frame.extend_from_slice(&9999u32.to_be_bytes()); // longer than the frame
        frame.extend_from_slice(b"short");
        assert!(parse_preview(&frame).is_none());
    }

    /// `progress_state` should surface the running node's progress.
    #[test]
    fn parse_progress_state() {
        let text = r#"{
            "type": "progress_state",
            "data": {
                "prompt_id": "abc",
                "nodes": {
                    "3": {"state": "finished", "value": 1.0, "max": 1.0, "node_id": "3"},
                    "7": {"state": "running", "value": 5.0, "max": 20.0, "node_id": "7"}
                }
            }
        }"#;
        match parse_text(text, "abc") {
            Parsed::Event(Event::Progress {
                node, value, max, ..
            }) => {
                assert_eq!(node, Some(WorkflowNodeId(7)));
                assert_eq!(value, 5);
                assert_eq!(max, 20);
            }
            other => panic!("expected Progress, got {other:?}"),
        }
    }

    /// Events tagged with a different prompt must be ignored.
    #[test]
    fn parse_text_filters_other_prompts() {
        let text = r#"{"type":"executing","data":{"node":"3","prompt_id":"other"}}"#;
        assert!(matches!(parse_text(text, "ours"), Parsed::Ignore));
    }

    /// A null `executing` node signals completion.
    #[test]
    fn parse_executing_null_is_done() {
        let text = r#"{"type":"executing","data":{"node":null,"prompt_id":"ours"}}"#;
        assert!(matches!(parse_text(text, "ours"), Parsed::Done));
    }

    /// Unmodelled message types deserialise to `Other` and are ignored.
    #[test]
    fn parse_text_ignores_unknown_types() {
        let text = r#"{"type":"executed","data":{"node":"3","output":{"images":[]}}}"#;
        assert!(matches!(parse_text(text, "ours"), Parsed::Ignore));
    }
}
