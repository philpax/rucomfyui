//! WebSocket transport for [`Client::execute`] (the `websocket` feature).
//!
//! Connects to ComfyUI's `/ws` endpoint to receive live progress events and
//! previews. The socket is connected *before* the prompt is queued so that no
//! early events are missed, and final outputs are always reconciled from the
//! history endpoint (via [`super::final_events`]) so a dropped socket never
//! costs us the results.

use ewebsock::{WsEvent, WsMessage, WsReceiver, WsSender};
use futures::Stream;
use serde_json::Value;

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
        let (sender, receiver) = match ewebsock::connect(url, ewebsock::Options::default()) {
            Ok(pair) => pair,
            Err(_) => return Ok(None),
        };

        // Wait for the connection to open before queueing, so the server has
        // registered our client ID and routes the prompt's events to us.
        let Some(receiver) = wait_for_open(receiver).await else {
            return Ok(None);
        };

        let prompt_id = self.queue_prompt(workflow).await?.prompt_id;
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
/// Frames are `{ "type": ..., "data": {...} }`. Events carrying a `prompt_id`
/// that isn't ours are ignored.
fn parse_text(text: &str, our_prompt_id: &str) -> Parsed {
    let Ok(value) = serde_json::from_str::<Value>(text) else {
        return Parsed::Ignore;
    };
    let Some(message_type) = value.get("type").and_then(Value::as_str) else {
        return Parsed::Ignore;
    };
    let data = value.get("data").unwrap_or(&Value::Null);

    // Ignore events tagged with a different prompt. `status` has no prompt_id.
    if let Some(prompt_id) = data.get("prompt_id").and_then(Value::as_str) {
        if prompt_id != our_prompt_id {
            return Parsed::Ignore;
        }
    }
    let prompt_id = || our_prompt_id.to_string();

    match message_type {
        "status" => {
            let queue_remaining = data
                .get("status")
                .and_then(|s| s.get("exec_info"))
                .and_then(|e| e.get("queue_remaining"))
                .and_then(Value::as_u64)
                .unwrap_or(0) as u32;
            Parsed::Event(Event::Status { queue_remaining })
        }
        "execution_start" => Parsed::Event(Event::ExecutionStart {
            prompt_id: prompt_id(),
        }),
        "executing" => {
            let node = parse_node(data.get("node"));
            // A null node signals the prompt has finished executing.
            if node.is_none() {
                Parsed::Done
            } else {
                Parsed::Event(Event::Executing {
                    prompt_id: prompt_id(),
                    node,
                })
            }
        }
        "progress" => Parsed::Event(Event::Progress {
            prompt_id: prompt_id(),
            node: parse_node(data.get("node")),
            value: data.get("value").and_then(Value::as_u64).unwrap_or(0) as u32,
            max: data.get("max").and_then(Value::as_u64).unwrap_or(0) as u32,
        }),
        "execution_success" => Parsed::Done,
        "execution_error" => Parsed::Failed {
            node: parse_node(data.get("node_id")),
            message: data
                .get("exception_message")
                .and_then(Value::as_str)
                .unwrap_or("execution error")
                .to_string(),
        },
        "execution_interrupted" => Parsed::Failed {
            node: parse_node(data.get("node_id")),
            message: "execution interrupted".to_string(),
        },
        // `executed`/`execution_cached` are informational; outputs come from
        // history so we don't surface them here.
        _ => Parsed::Ignore,
    }
}

/// Parses a node ID from a JSON value that may be a string or null.
fn parse_node(value: Option<&Value>) -> Option<WorkflowNodeId> {
    value
        .and_then(Value::as_str)
        .and_then(|s| s.parse::<WorkflowNodeId>().ok())
}

/// Parses a binary preview frame.
///
/// The framing is an 8-byte header (a big-endian `u32` event type, then a
/// big-endian `u32` image format) followed by the raw image bytes. Only event
/// type `1` (preview image) is recognised.
fn parse_preview(bytes: &[u8]) -> Option<PreviewImage> {
    if bytes.len() < 8 {
        return None;
    }
    let event_type = u32::from_be_bytes(bytes[0..4].try_into().ok()?);
    if event_type != 1 {
        return None;
    }
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
