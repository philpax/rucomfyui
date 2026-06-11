//! Executing workflows and observing their progress.
//!
//! [`Client::execute`] queues a workflow and returns an [`Execution`], a stream
//! of [`Event`]s describing the workflow's progress. The stream is the universal
//! interface: whether the events are sourced from a live WebSocket connection
//! (with the `websocket` feature) or synthesised from HTTP polling is an
//! implementation detail.
//!
//! The simplest usage drives the stream to completion and collects the outputs:
//! ```no_run
//! # async fn example(client: &rucomfyui::Client, workflow: &rucomfyui::Workflow) -> rucomfyui::Result<()> {
//! let outputs = client.execute(workflow).await?.outputs().await?;
//! # Ok(()) }
//! ```
//! To observe progress, consume the [`Execution`] as a [`Stream`] instead.

use std::{collections::HashMap, pin::Pin, task::Poll};

use futures::{Stream, StreamExt};

use crate::{
    history::HistoryNodeOutput, workflow::WorkflowNodeId, Client, ClientError, OwnedBytes, Result,
    Workflow,
};

/// The outputs produced by a single node during execution.
#[derive(Debug, Clone, Default)]
pub struct NodeOutput {
    /// Output images, downloaded as raw bytes.
    pub images: Vec<OwnedBytes>,
    /// Output texts.
    pub texts: Vec<String>,
}

/// The encoding of a [`PreviewImage`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewImageFormat {
    /// JPEG.
    Jpeg,
    /// PNG.
    Png,
    /// An unrecognised format.
    Unknown,
}

/// A preview image streamed during execution.
///
/// Only emitted by the WebSocket transport (the `websocket` feature); the
/// polling transport cannot produce previews.
#[derive(Debug, Clone)]
pub struct PreviewImage {
    /// The image encoding.
    pub format: PreviewImageFormat,
    /// The raw image bytes.
    pub data: OwnedBytes,
}

/// An event describing the progress of an [`Execution`].
///
/// `Progress` and `Preview` are only produced by the WebSocket transport; the
/// polling transport produces `ExecutionStart`, `Executed`, and `Completed`.
#[derive(Debug, Clone)]
pub enum Event {
    /// The number of prompts remaining in the queue changed.
    Status {
        /// Number of prompts still queued (including the running one).
        queue_remaining: u32,
    },
    /// Execution of the prompt has started.
    ExecutionStart {
        /// The prompt being executed.
        prompt_id: String,
    },
    /// A node has started executing, or (when `node` is `None`) the prompt has
    /// finished executing.
    Executing {
        /// The prompt being executed.
        prompt_id: String,
        /// The node now executing, or `None` if the prompt is done.
        node: Option<WorkflowNodeId>,
    },
    /// Progress within a node (e.g. sampler steps). WebSocket transport only.
    Progress {
        /// The prompt being executed.
        prompt_id: String,
        /// The node reporting progress, if known.
        node: Option<WorkflowNodeId>,
        /// The current progress value.
        value: u32,
        /// The maximum progress value.
        max: u32,
    },
    /// A node finished and produced output.
    Executed {
        /// The prompt being executed.
        prompt_id: String,
        /// The node that produced the output.
        node: WorkflowNodeId,
        /// The node's output.
        output: NodeOutput,
    },
    /// A preview image was streamed. WebSocket transport only.
    Preview {
        /// The prompt being executed.
        prompt_id: String,
        /// The preview image.
        image: PreviewImage,
    },
    /// The prompt failed during execution.
    Error {
        /// The prompt that failed.
        prompt_id: String,
        /// The node that failed, if known.
        node: Option<WorkflowNodeId>,
        /// The error message.
        message: String,
    },
    /// The prompt completed successfully. This is the terminal event.
    Completed {
        /// The prompt that completed.
        prompt_id: String,
    },
}

// The boxed event stream backing an `Execution`. We require `Send` on native so
// the stream can be driven by multi-threaded executors, but not on WASM where
// the underlying transports (reqwest, ewebsock) produce `!Send` futures.
#[cfg(not(target_arch = "wasm32"))]
type BoxEventStream = Pin<Box<dyn Stream<Item = Result<Event>> + Send>>;
#[cfg(target_arch = "wasm32")]
type BoxEventStream = Pin<Box<dyn Stream<Item = Result<Event>>>>;

/// A running workflow execution.
///
/// This is a [`Stream`] of [`Event`]s. Drive it to completion with
/// [`Execution::outputs`] to collect the final outputs, or consume it event by
/// event to observe progress.
pub struct Execution {
    prompt_id: String,
    stream: BoxEventStream,
}
impl Execution {
    /// The ID of the prompt being executed.
    pub fn prompt_id(&self) -> &str {
        &self.prompt_id
    }

    /// Drive the execution to completion, returning each node's output.
    ///
    /// Returns [`ClientError::Execution`] if the workflow reports an error.
    pub async fn outputs(mut self) -> Result<HashMap<WorkflowNodeId, NodeOutput>> {
        let mut outputs = HashMap::new();
        while let Some(event) = self.stream.next().await {
            match event? {
                Event::Executed { node, output, .. } => {
                    outputs.insert(node, output);
                }
                Event::Error {
                    prompt_id,
                    node,
                    message,
                } => {
                    return Err(ClientError::Execution {
                        prompt_id,
                        node,
                        message,
                    });
                }
                _ => {}
            }
        }
        Ok(outputs)
    }
}
impl Stream for Execution {
    type Item = Result<Event>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.stream.as_mut().poll_next(cx)
    }
}

/// Functions for executing workflows and observing their progress.
impl Client {
    /// Queue a workflow and return an [`Execution`] tracking its progress.
    ///
    /// The returned [`Execution`] is a [`Stream`] of [`Event`]s. Call
    /// [`Execution::outputs`] for the common "run and collect outputs" case, or
    /// consume the stream to observe progress.
    pub async fn execute(&self, workflow: &Workflow) -> Result<Execution> {
        let prompt_id = self.queue_prompt(workflow).await?.prompt_id;
        let stream = Box::pin(poll_execution(self.clone(), prompt_id.clone()));
        Ok(Execution { prompt_id, stream })
    }
}

/// Synthesises an [`Event`] stream by polling the history endpoint.
///
/// This is the transport used without the `websocket` feature (or as a fallback
/// when the socket is unavailable). It can only emit `ExecutionStart`, the final
/// `Executed` events, and `Completed`; fine-grained `Progress`/`Preview`/`Status`
/// events require the WebSocket transport.
fn poll_execution(client: Client, prompt_id: String) -> impl Stream<Item = Result<Event>> {
    async_stream::try_stream! {
        yield Event::ExecutionStart { prompt_id: prompt_id.clone() };
        loop {
            let history = client.get_history_for_prompt(&prompt_id).await?;
            if let Some(data) = history.data.get(&prompt_id) {
                if data.status.completed {
                    for (node_name, node_output) in &data.outputs.nodes {
                        let node = node_name.parse::<WorkflowNodeId>()?;
                        let output = download_node_output(&client, node_output).await?;
                        yield Event::Executed {
                            prompt_id: prompt_id.clone(),
                            node,
                            output,
                        };
                    }
                    yield Event::Completed { prompt_id: prompt_id.clone() };
                    break;
                }
            }
            futures_timer::Delay::new(web_time::Duration::from_millis(100)).await;
        }
    }
}

/// Downloads the images referenced by a history node output into a [`NodeOutput`].
async fn download_node_output(
    client: &Client,
    node_output: &HistoryNodeOutput,
) -> Result<NodeOutput> {
    let images =
        futures::future::try_join_all(node_output.images.iter().map(|i| i.download(client)))
            .await?;
    Ok(NodeOutput {
        images,
        texts: node_output.text.clone(),
    })
}
