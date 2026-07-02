//! ComfyUI client for Lua.

use std::collections::HashMap;

use futures::StreamExt;
use mlua::LuaSerdeExt;
use mlua::prelude::*;
use rucomfyui::{Event, WorkflowNodeId};
use serde::Serialize;

use crate::{config::ClientConfig, error::to_lua_result, graph::Graph, node_output::NodeOutput};

/// A ComfyUI client.
///
/// This wraps the rucomfyui Client and exposes its functionality to Lua.
pub struct Client {
    inner: rucomfyui::Client,
    config: ClientConfig,
}

impl Client {
    /// Create a new client with a URL and config.
    pub fn new(_lua: &Lua, url: String, config: ClientConfig) -> LuaResult<Self> {
        Ok(Self {
            inner: rucomfyui::Client::new(url),
            config,
        })
    }

    /// Create a client from an existing rucomfyui::Client.
    ///
    /// This allows Rust integrators to pass in a pre-configured client.
    pub fn from_existing(client: rucomfyui::Client, config: ClientConfig) -> Self {
        Self {
            inner: client,
            config,
        }
    }

    /// Check if a method is allowed.
    fn check_allowed(&self, method_name: &str, allowed: bool) -> LuaResult<()> {
        if allowed {
            Ok(())
        } else {
            Err(LuaError::external(format!(
                "Method '{}' is not enabled in ClientConfig",
                method_name
            )))
        }
    }
}

impl LuaUserData for Client {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // ==================== Object Info ====================

        methods.add_async_method("get_object_info", |lua, this, ()| async move {
            this.check_allowed("get_object_info", this.config.get_object_info)?;
            let object_info = to_lua_result(this.inner.get_object_info().await)?;
            lua.to_value(&object_info)
        });

        methods.add_async_method(
            "get_object_for_name",
            |lua, this, name: String| async move {
                this.check_allowed("get_object_for_name", this.config.get_object_for_name)?;
                let object = to_lua_result(this.inner.get_object_for_name(&name).await)?;
                lua.to_value(&object)
            },
        );

        // ==================== Queue Operations ====================

        methods.add_async_method(
            "queue_prompt",
            |lua, this, graph: LuaAnyUserData| async move {
                this.check_allowed("queue_prompt", this.config.queue_prompt)?;
                let graph_ref = graph.borrow::<Graph>()?;
                let workflow = graph_ref.to_workflow();
                drop(graph_ref);

                let result = to_lua_result(this.inner.queue_prompt(&workflow).await)?;
                lua.to_value(&result)
            },
        );

        // execute needs manual handling for images (bytes) and custom metatable
        methods.add_async_method(
            "execute",
            |lua, this, (graph, opts): (LuaAnyUserData, Option<LuaTable>)| async move {
                this.check_allowed("execute", this.config.execute)?;
                let graph_ref = graph.borrow::<Graph>()?;
                let workflow = graph_ref.to_workflow();
                drop(graph_ref);

                // Extract the optional on_event callback.
                let on_event = opts.and_then(|t| t.get::<LuaFunction>("on_event").ok());

                let result = if let Some(on_event) = on_event {
                    // With a callback: consume the stream directly, invoking the
                    // callback for each event and collecting Executed outputs.
                    let mut execution = to_lua_result(this.inner.execute(&workflow).await)?;
                    let mut outputs: HashMap<WorkflowNodeId, rucomfyui::execute::NodeOutput> =
                        HashMap::new();
                    while let Some(event) = execution.next().await {
                        match to_lua_result(event)? {
                            Event::Error { node, message, .. } => {
                                return Err(LuaError::external(format!(
                                    "Execution error{}: {}",
                                    node.map(|n| format!(" at node {}", n)).unwrap_or_default(),
                                    message
                                )));
                            }
                            Event::Completed { .. } => break,
                            Event::Executed {
                                prompt_id,
                                node,
                                output,
                            } => {
                                outputs.insert(node, output.clone());
                                let event_table = event_to_lua_table(
                                    &lua,
                                    &Event::Executed {
                                        prompt_id,
                                        node,
                                        output,
                                    },
                                )?;
                                on_event.call_async::<()>(event_table).await?;
                            }
                            other => {
                                let event_table = event_to_lua_table(&lua, &other)?;
                                on_event.call_async::<()>(event_table).await?;
                            }
                        }
                    }
                    outputs
                } else {
                    // Backward-compatible: drain via outputs().
                    to_lua_result(
                        to_lua_result(this.inner.execute(&workflow).await)?
                            .outputs()
                            .await,
                    )?
                };

                build_outputs_table(&lua, result)
            },
        );

        methods.add_async_method("get_queue", |lua, this, ()| async move {
            this.check_allowed("get_queue", this.config.get_queue)?;
            let queue = to_lua_result(this.inner.get_queue().await)?;
            lua.to_value(&queue)
        });

        methods.add_async_method("interrupt", |_lua, this, ()| async move {
            this.check_allowed("interrupt", this.config.interrupt)?;
            to_lua_result(this.inner.interrupt().await)?;
            Ok(())
        });

        methods.add_async_method(
            "delete_from_queue",
            |_lua, this, prompt_ids: Vec<String>| async move {
                this.check_allowed("delete_from_queue", this.config.delete_from_queue)?;
                to_lua_result(this.inner.delete_from_queue(prompt_ids).await)?;
                Ok(())
            },
        );

        methods.add_async_method("clear_queue", |_lua, this, ()| async move {
            this.check_allowed("clear_queue", this.config.clear_queue)?;
            to_lua_result(this.inner.clear_queue().await)?;
            Ok(())
        });

        // ==================== History Operations ====================

        methods.add_async_method("get_history", |lua, this, max_items: u32| async move {
            this.check_allowed("get_history", this.config.get_history)?;
            let history = to_lua_result(this.inner.get_history(max_items).await)?;
            lua.to_value(&history)
        });

        methods.add_async_method(
            "get_history_for_prompt",
            |lua, this, prompt_id: String| async move {
                this.check_allowed("get_history_for_prompt", this.config.get_history_for_prompt)?;
                let history = to_lua_result(this.inner.get_history_for_prompt(&prompt_id).await)?;
                lua.to_value(&history)
            },
        );

        methods.add_async_method(
            "delete_from_history",
            |_lua, this, prompt_ids: Vec<String>| async move {
                this.check_allowed("delete_from_history", this.config.delete_from_history)?;
                to_lua_result(this.inner.delete_from_history(prompt_ids).await)?;
                Ok(())
            },
        );

        methods.add_async_method("clear_history", |_lua, this, ()| async move {
            this.check_allowed("clear_history", this.config.clear_history)?;
            to_lua_result(this.inner.clear_history().await)?;
            Ok(())
        });

        // ==================== Model Operations ====================

        methods.add_async_method("get_model_categories", |lua, this, ()| async move {
            this.check_allowed("get_model_categories", this.config.get_model_categories)?;
            let categories = to_lua_result(this.inner.get_model_categories().await)?;
            lua.to_value(&categories)
        });

        methods.add_async_method("get_models", |lua, this, category: String| async move {
            this.check_allowed("get_models", this.config.get_models)?;
            let cat: rucomfyui::models::ModelCategory =
                lua.from_value(LuaValue::String(lua.create_string(&category)?))?;
            let models = to_lua_result(this.inner.get_models(cat).await)?;
            lua.to_value(&models)
        });

        // ==================== System Operations ====================

        methods.add_async_method("system_stats", |lua, this, ()| async move {
            this.check_allowed("system_stats", this.config.system_stats)?;
            let stats = to_lua_result(this.inner.system_stats().await)?;
            lua.to_value(&stats)
        });

        methods.add_async_method(
            "free",
            |_lua, this, (unload_models, free_memory): (bool, bool)| async move {
                this.check_allowed("free", this.config.free)?;
                to_lua_result(this.inner.free(unload_models, free_memory).await)?;
                Ok(())
            },
        );

        // ==================== Upload Operations ====================

        methods.add_async_method(
            "upload_image",
            |lua,
             this,
             (filename, bytes, upload_type, overwrite): (
                String,
                LuaString,
                Option<String>,
                Option<bool>,
            )| async move {
                this.check_allowed("upload_image", this.config.upload)?;
                let bytes_vec = bytes.as_bytes().to_vec();
                let upload_type = upload_type
                    .as_deref()
                    .unwrap_or("input")
                    .parse::<rucomfyui::upload::UploadType>()
                    .map_err(LuaError::runtime)?;
                let overwrite = overwrite.unwrap_or(true);
                let result = to_lua_result(
                    this.inner
                        .upload_image(&filename, bytes_vec, upload_type, overwrite)
                        .await,
                )?;
                lua.to_value(&result)
            },
        );
    }
}

/// Builds the outputs table returned by `client:execute`.
///
/// The table maps node IDs (as Lua integers) to `{ images = {LuaString,...}, texts = {...} }`.
/// A metatable allows indexing by the `NodeOutput`/`NodeOutputs` userdata types.
fn build_outputs_table(
    lua: &Lua,
    result: HashMap<WorkflowNodeId, rucomfyui::execute::NodeOutput>,
) -> LuaResult<LuaTable> {
    let output_table = lua.create_table()?;

    for (node_id, node_output) in result {
        let node_table = lua.create_table()?;

        // Images as byte strings
        let images_table = lua.create_table()?;
        for (i, image) in node_output.images.into_iter().enumerate() {
            images_table.set(i + 1, lua.create_string(&image)?)?;
        }
        node_table.set("images", images_table)?;

        // Texts
        let texts_table = lua.create_table()?;
        for (i, text) in node_output.texts.into_iter().enumerate() {
            texts_table.set(i + 1, text)?;
        }
        node_table.set("texts", texts_table)?;

        output_table.set(node_id.0, node_table)?;
    }

    // Add a metatable to allow indexing by NodeOutput
    let metatable = lua.create_table()?;
    metatable.set(
        "__index",
        lua.create_function(
            |_lua, (table, key): (LuaTable, LuaValue)| -> LuaResult<LuaValue> {
                if let LuaValue::UserData(ud) = &key {
                    if let Ok(output) = ud.borrow::<NodeOutput>() {
                        return table.raw_get(output.node_id.0);
                    }
                    if let Ok(outputs) = ud.borrow::<crate::node_output::NodeOutputs>() {
                        return table.raw_get(outputs.node_id.0);
                    }
                }
                table.raw_get(key)
            },
        )?,
    )?;
    output_table.set_metatable(Some(metatable))?;

    Ok(output_table)
}

/// Converts an [`Event`] into a Lua table for the `on_event` callback.
///
/// Each event table has a `type` field identifying the variant, plus payload
/// fields specific to that variant. Uses mlua's serde serializer so that byte
/// fields (`images`, `data`) become Lua strings and `Option<T>` becomes `nil`.
fn event_to_lua_table(lua: &Lua, event: &Event) -> LuaResult<LuaTable> {
    use LuaSerializeOptions;

    /// A single image as raw bytes (serialized as a Lua string, not a number
    /// sequence, via `serde_bytes`).
    #[derive(Serialize)]
    struct LuaImage<'a>(#[serde(with = "serde_bytes")] &'a [u8]);

    /// The `PreviewImage` payload with byte data serialized as a Lua string.
    #[derive(Serialize)]
    struct LuaPreviewImage<'a> {
        format: &'a rucomfyui::execute::PreviewImageFormat,
        #[serde(with = "serde_bytes")]
        data: &'a [u8],
    }

    /// A serializable payload mirroring [`Event`] with an internally-tagged
    /// `type` field. Byte fields use `serde_bytes` so they become Lua strings
    /// rather than sequences of numbers.
    #[derive(Serialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    enum LuaEvent<'a> {
        Status {
            queue_remaining: u32,
        },
        ExecutionStart {
            prompt_id: &'a str,
        },
        Executing {
            prompt_id: &'a str,
            node: Option<u32>,
        },
        Progress {
            prompt_id: &'a str,
            node: Option<u32>,
            value: u32,
            max: u32,
        },
        Executed {
            prompt_id: &'a str,
            node: u32,
            images: Vec<LuaImage<'a>>,
            texts: &'a [String],
        },
        Preview {
            prompt_id: &'a str,
            #[serde(flatten)]
            image: LuaPreviewImage<'a>,
        },
        Error {
            prompt_id: &'a str,
            node: Option<u32>,
            message: &'a str,
        },
        Completed {
            prompt_id: &'a str,
        },
    }

    let payload = match event {
        Event::Status { queue_remaining } => LuaEvent::Status {
            queue_remaining: *queue_remaining,
        },
        Event::ExecutionStart { prompt_id } => LuaEvent::ExecutionStart { prompt_id },
        Event::Executing { prompt_id, node } => LuaEvent::Executing {
            prompt_id,
            node: node.map(|n| n.0),
        },
        Event::Progress {
            prompt_id,
            node,
            value,
            max,
        } => LuaEvent::Progress {
            prompt_id,
            node: node.map(|n| n.0),
            value: *value,
            max: *max,
        },
        Event::Executed {
            prompt_id,
            node,
            output,
        } => LuaEvent::Executed {
            prompt_id,
            node: node.0,
            images: output
                .images
                .iter()
                .map(|v| LuaImage(v.as_slice()))
                .collect(),
            texts: &output.texts,
        },
        Event::Preview { prompt_id, image } => LuaEvent::Preview {
            prompt_id,
            image: LuaPreviewImage {
                format: &image.format,
                data: &image.data,
            },
        },
        Event::Error {
            prompt_id,
            node,
            message,
        } => LuaEvent::Error {
            prompt_id,
            node: node.map(|n| n.0),
            message,
        },
        Event::Completed { prompt_id } => LuaEvent::Completed { prompt_id },
    };

    // `serialize_none_to_null(false)` makes `None` → `nil` (not `null`).
    let opts = LuaSerializeOptions::default().serialize_none_to_null(false);
    let value = lua.to_value_with(&payload, opts)?;
    match value {
        LuaValue::Table(t) => Ok(t),
        _ => Err(LuaError::runtime("expected event to serialize to a table")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rucomfyui::execute::{NodeOutput, PreviewImage, PreviewImageFormat};

    fn lua() -> Lua {
        Lua::new()
    }

    #[test]
    fn status_event() {
        let lua = lua();
        let event = Event::Status { queue_remaining: 3 };
        let t = event_to_lua_table(&lua, &event).unwrap();
        assert_eq!(t.get::<String>("type").unwrap(), "status");
        assert_eq!(t.get::<u32>("queue_remaining").unwrap(), 3);
    }

    #[test]
    fn execution_start_event() {
        let lua = lua();
        let event = Event::ExecutionStart {
            prompt_id: "abc-123".to_string(),
        };
        let t = event_to_lua_table(&lua, &event).unwrap();
        assert_eq!(t.get::<String>("type").unwrap(), "execution_start");
        assert_eq!(t.get::<String>("prompt_id").unwrap(), "abc-123");
    }

    #[test]
    fn executing_event_with_node() {
        let lua = lua();
        let event = Event::Executing {
            prompt_id: "p1".to_string(),
            node: Some(WorkflowNodeId(7)),
        };
        let t = event_to_lua_table(&lua, &event).unwrap();
        assert_eq!(t.get::<String>("type").unwrap(), "executing");
        assert_eq!(t.get::<String>("prompt_id").unwrap(), "p1");
        assert_eq!(t.get::<u32>("node").unwrap(), 7);
    }

    #[test]
    fn executing_event_node_nil() {
        let lua = lua();
        let event = Event::Executing {
            prompt_id: "p1".to_string(),
            node: None,
        };
        let t = event_to_lua_table(&lua, &event).unwrap();
        assert_eq!(t.get::<String>("type").unwrap(), "executing");
        assert!(matches!(t.get::<LuaValue>("node").unwrap(), LuaValue::Nil));
    }

    #[test]
    fn progress_event() {
        let lua = lua();
        let event = Event::Progress {
            prompt_id: "p1".to_string(),
            node: Some(WorkflowNodeId(5)),
            value: 10,
            max: 20,
        };
        let t = event_to_lua_table(&lua, &event).unwrap();
        assert_eq!(t.get::<String>("type").unwrap(), "progress");
        assert_eq!(t.get::<String>("prompt_id").unwrap(), "p1");
        assert_eq!(t.get::<u32>("node").unwrap(), 5);
        assert_eq!(t.get::<u32>("value").unwrap(), 10);
        assert_eq!(t.get::<u32>("max").unwrap(), 20);
    }

    #[test]
    fn executed_event() {
        let lua = lua();
        let event = Event::Executed {
            prompt_id: "p1".to_string(),
            node: WorkflowNodeId(9),
            output: NodeOutput {
                images: vec![vec![1, 2, 3], vec![4, 5]],
                texts: vec!["hello".to_string(), "world".to_string()],
            },
        };
        let t = event_to_lua_table(&lua, &event).unwrap();
        assert_eq!(t.get::<String>("type").unwrap(), "executed");
        assert_eq!(t.get::<String>("prompt_id").unwrap(), "p1");
        assert_eq!(t.get::<u32>("node").unwrap(), 9);

        let images: LuaTable = t.get("images").unwrap();
        assert_eq!(images.len().unwrap(), 2);
        let first: mlua::String = images.get(1).unwrap();
        assert_eq!(first.as_bytes(), &[1, 2, 3]);

        let texts: LuaTable = t.get("texts").unwrap();
        assert_eq!(texts.len().unwrap(), 2);
        let first_text: String = texts.get(1).unwrap();
        assert_eq!(first_text, "hello");
    }

    #[test]
    fn preview_event_jpeg() {
        let lua = lua();
        let event = Event::Preview {
            prompt_id: "p1".to_string(),
            image: PreviewImage {
                format: PreviewImageFormat::Jpeg,
                data: vec![0xFF, 0xD8, 0xFF],
            },
        };
        let t = event_to_lua_table(&lua, &event).unwrap();
        assert_eq!(t.get::<String>("type").unwrap(), "preview");
        assert_eq!(t.get::<String>("prompt_id").unwrap(), "p1");
        assert_eq!(t.get::<String>("format").unwrap(), "jpeg");
        let data: mlua::String = t.get("data").unwrap();
        assert_eq!(data.as_bytes(), &[0xFF, 0xD8, 0xFF]);
    }

    #[test]
    fn preview_event_png() {
        let lua = lua();
        let event = Event::Preview {
            prompt_id: "p1".to_string(),
            image: PreviewImage {
                format: PreviewImageFormat::Png,
                data: vec![0x89, 0x50, 0x4E, 0x47],
            },
        };
        let t = event_to_lua_table(&lua, &event).unwrap();
        assert_eq!(t.get::<String>("format").unwrap(), "png");
    }

    #[test]
    fn preview_event_unknown_format() {
        let lua = lua();
        let event = Event::Preview {
            prompt_id: "p1".to_string(),
            image: PreviewImage {
                format: PreviewImageFormat::Unknown,
                data: vec![],
            },
        };
        let t = event_to_lua_table(&lua, &event).unwrap();
        assert_eq!(t.get::<String>("format").unwrap(), "unknown");
    }
}
