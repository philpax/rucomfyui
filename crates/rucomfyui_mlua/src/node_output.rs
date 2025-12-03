//! Node output types for Lua.
//!
//! These types represent the outputs of ComfyUI nodes and can be used
//! as inputs to other nodes.

use mlua::prelude::*;
use rucomfyui::workflow::WorkflowNodeId;
use std::collections::HashMap;

/// A single output from a node.
///
/// This represents a specific output slot from a node and can be used
/// as an input to another node.
#[derive(Debug, Clone, Copy)]
pub struct NodeOutput {
    /// The node ID that produced this output.
    pub node_id: WorkflowNodeId,
    /// The output slot index.
    pub slot: u32,
}

impl NodeOutput {
    /// Create a new node output.
    pub fn new(node_id: WorkflowNodeId, slot: u32) -> Self {
        Self { node_id, slot }
    }
}

impl LuaUserData for NodeOutput {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("NodeOutput({}, slot={})", this.node_id, this.slot))
        });
    }
}

/// Multiple outputs from a node, accessible by name.
///
/// For nodes with multiple outputs (like CheckpointLoaderSimple which outputs
/// model, clip, and vae), this provides named access to each output.
#[derive(Debug, Clone)]
pub struct NodeOutputs {
    /// The node ID that produced these outputs.
    pub node_id: WorkflowNodeId,
    /// Map from output name to slot index.
    pub outputs: HashMap<String, u32>,
}

impl NodeOutputs {
    /// Create new node outputs.
    pub fn new(node_id: WorkflowNodeId, outputs: HashMap<String, u32>) -> Self {
        Self { node_id, outputs }
    }

    /// Get a specific output by name.
    pub fn get(&self, name: &str) -> Option<NodeOutput> {
        self.outputs
            .get(name)
            .map(|&slot| NodeOutput::new(self.node_id, slot))
    }
}

impl LuaUserData for NodeOutputs {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // Expose node_id for result lookup
        fields.add_field_method_get("_node_id", |_, this| Ok(this.node_id.0));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            let outputs: Vec<_> = this.outputs.keys().collect();
            Ok(format!(
                "NodeOutputs({}, outputs={:?})",
                this.node_id, outputs
            ))
        });

        // Allow indexing by output name: node_outputs.model, node_outputs.clip, etc.
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: String| {
            if key == "_node_id" {
                return Ok(LuaValue::Integer(this.node_id.0 as i64));
            }

            match this.get(&key) {
                Some(output) => Ok(LuaValue::UserData(lua.create_userdata(output)?)),
                None => Ok(LuaValue::Nil),
            }
        });
    }
}

/// Either a single output or multiple outputs.
///
/// This is returned by node constructors - single-output nodes return
/// a NodeOutput directly, while multi-output nodes return NodeOutputs.
#[derive(Debug, Clone)]
pub enum NodeOutputValue {
    /// A single output.
    Single(NodeOutput),
    /// Multiple named outputs.
    Multiple(NodeOutputs),
}

impl IntoLua for NodeOutputValue {
    fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
        match self {
            Self::Single(output) => Ok(LuaValue::UserData(lua.create_userdata(output)?)),
            Self::Multiple(outputs) => Ok(LuaValue::UserData(lua.create_userdata(outputs)?)),
        }
    }
}
