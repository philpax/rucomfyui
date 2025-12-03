//! ComfyUI client for Lua.

use mlua::prelude::*;

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

        methods.add_async_method("get_object_for_name", |lua, this, name: String| async move {
            this.check_allowed("get_object_for_name", this.config.get_object_for_name)?;
            let object = to_lua_result(this.inner.get_object_for_name(&name).await)?;
            lua.to_value(&object)
        });

        // ==================== Queue Operations ====================

        methods.add_async_method("queue", |lua, this, graph: LuaAnyUserData| async move {
            this.check_allowed("queue", this.config.queue)?;
            let graph_ref = graph.borrow::<Graph>()?;
            let workflow = graph_ref.to_workflow();
            drop(graph_ref);

            let result = to_lua_result(this.inner.queue(&workflow).await)?;
            lua.to_value(&result)
        });

        // easy_queue needs manual handling for images (bytes) and custom metatable
        methods.add_async_method("easy_queue", |lua, this, graph: LuaAnyUserData| async move {
            this.check_allowed("easy_queue", this.config.easy_queue)?;
            let graph_ref = graph.borrow::<Graph>()?;
            let workflow = graph_ref.to_workflow();
            drop(graph_ref);

            let result = to_lua_result(this.inner.easy_queue(&workflow).await)?;

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
                            if let Ok(outputs) =
                                ud.borrow::<crate::node_output::NodeOutputs>()
                            {
                                return table.raw_get(outputs.node_id.0);
                            }
                        }
                        table.raw_get(key)
                    },
                )?,
            )?;
            output_table.set_metatable(Some(metatable));

            Ok(output_table)
        });

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
                serde_json::from_value(serde_json::Value::String(category))
                    .map_err(mlua::Error::external)?;
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
            "upload",
            |lua, this, (filename, bytes): (String, LuaString)| async move {
                this.check_allowed("upload", this.config.upload)?;
                let bytes_vec = bytes.as_bytes().to_vec();
                let result = to_lua_result(this.inner.upload(&filename, bytes_vec).await)?;
                lua.to_value(&result)
            },
        );
    }
}
