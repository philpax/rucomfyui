//! Graph builder for ComfyUI workflows.

use std::{cell::RefCell, collections::HashMap};

use mlua::prelude::*;
use rucomfyui::{
    object_info::{Object, ObjectInfo},
    workflow::{Workflow, WorkflowInput, WorkflowMeta, WorkflowNode, WorkflowNodeId},
};

use crate::{
    conversion::lua_to_workflow_input,
    error::Error,
    node_output::{NodeOutput, NodeOutputValue, NodeOutputs},
};

/// A workflow graph builder.
///
/// This is the main type for building ComfyUI workflows in Lua.
/// Node types are exposed as methods via the `__index` metamethod.
pub struct Graph {
    /// The workflow being built.
    workflow: RefCell<Workflow>,
    /// The next node ID to assign.
    next_id: RefCell<u32>,
    /// Object info for node definitions.
    object_info: ObjectInfo,
}

impl Graph {
    /// Create a new graph from object info.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(lua: &Lua, object_info_table: LuaTable) -> LuaResult<LuaAnyUserData> {
        // Use mlua's serde feature to deserialize the Lua table directly
        let object_info: ObjectInfo = lua.from_value(LuaValue::Table(object_info_table))?;

        lua.create_userdata(Self {
            workflow: RefCell::new(Workflow::default()),
            next_id: RefCell::new(0),
            object_info,
        })
    }

    /// Add a node to the graph.
    fn add_node(&self, node: WorkflowNode) -> WorkflowNodeId {
        let mut next_id = self.next_id.borrow_mut();
        *next_id += 1;
        let id = WorkflowNodeId(*next_id);
        self.workflow.borrow_mut().0.insert(id, node);
        id
    }

    /// Get the object definition for a node type.
    fn get_object(&self, name: &str) -> Option<&Object> {
        self.object_info.get(name)
    }

    /// Convert the graph to a workflow.
    pub fn to_workflow(&self) -> Workflow {
        self.workflow.borrow().clone()
    }
}

impl LuaUserData for Graph {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Get the underlying workflow as a Lua-compatible table
        methods.add_method("_get_workflow", |lua, this, ()| {
            let workflow = this.to_workflow();
            lua.to_value(&workflow)
        });

        // The main magic: __index returns a node constructor for any node type
        methods.add_meta_method(LuaMetaMethod::Index, |lua, this, key: String| {
            // Check if it's a known node type
            if let Some(object) = this.get_object(&key) {
                // Return a constructor function for this node type
                let object = object.clone();
                let constructor = lua.create_function(move |lua, args: LuaMultiValue| {
                    create_node(lua, &object, args)
                })?;
                Ok(LuaValue::Function(constructor))
            } else {
                // Unknown node type - return nil (let Lua handle the error)
                Ok(LuaValue::Nil)
            }
        });
    }
}

/// Create a node from Lua arguments.
fn create_node(lua: &Lua, object: &Object, args: LuaMultiValue) -> LuaResult<LuaValue> {
    // Get the graph from the first argument (self)
    let mut args_iter = args.into_iter();
    let graph_ud = args_iter
        .next()
        .ok_or_else(|| LuaError::external("Expected graph as first argument"))?;

    let graph = match &graph_ud {
        LuaValue::UserData(ud) => ud.borrow::<Graph>()?,
        _ => return Err(LuaError::external("Expected graph as first argument")),
    };

    // Parse the remaining arguments into inputs
    let inputs = parse_node_inputs(object, args_iter.collect())?;

    // Create the workflow node
    let node = WorkflowNode {
        inputs,
        class_type: object.name.clone(),
        meta: Some(WorkflowMeta::new(object.display_name())),
    };

    // Add to graph and create output
    let node_id = graph.add_node(node);
    let output = create_node_output(object, node_id);

    output.into_lua(lua)
}

/// Parse Lua arguments into workflow inputs.
fn parse_node_inputs(
    object: &Object,
    args: Vec<LuaValue>,
) -> LuaResult<HashMap<String, WorkflowInput>> {
    let mut inputs = HashMap::new();

    // Collect input names in order
    let input_names: Vec<&str> = object.all_inputs().map(|(name, _, _)| name).collect();

    if args.is_empty() {
        // No arguments - check if all inputs are optional
        for (name, _input, required) in object.all_inputs() {
            if required {
                return Err(Error::MissingInput {
                    node: object.name.clone(),
                    input: name.to_string(),
                }
                .into());
            }
        }
        return Ok(inputs);
    }

    // Check if first argument is a table (named arguments)
    if let Some(LuaValue::Table(table)) = args.first() {
        // Named arguments mode
        for (name, _input, required) in object.all_inputs() {
            let value: LuaValue = table.get(name)?;
            if value.is_nil() {
                if required {
                    return Err(Error::MissingInput {
                        node: object.name.clone(),
                        input: name.to_string(),
                    }
                    .into());
                }
            } else {
                inputs.insert(name.to_string(), lua_to_workflow_input(value)?);
            }
        }
    } else {
        // Positional arguments mode
        for (i, value) in args.into_iter().enumerate() {
            if i >= input_names.len() {
                break; // Extra arguments ignored
            }
            let name = input_names[i];
            inputs.insert(name.to_string(), lua_to_workflow_input(value)?);
        }

        // Check for missing required inputs
        for (name, _input, required) in object.all_inputs() {
            if required && !inputs.contains_key(name) {
                return Err(Error::MissingInput {
                    node: object.name.clone(),
                    input: name.to_string(),
                }
                .into());
            }
        }
    }

    Ok(inputs)
}

/// Create the appropriate output type for a node.
fn create_node_output(object: &Object, node_id: WorkflowNodeId) -> NodeOutputValue {
    if object.output.len() <= 1 {
        // Single output or output node
        NodeOutputValue::Single(NodeOutput::new(node_id, 0))
    } else {
        // Multiple outputs - create named outputs
        let mut outputs = HashMap::new();
        for (i, name) in object.output_name.iter().enumerate() {
            // Convert output name to snake_case for Lua field access
            let lua_name = to_snake_case(name);
            outputs.insert(lua_name, i as u32);
        }
        NodeOutputValue::Multiple(NodeOutputs::new(node_id, outputs))
    }
}

/// Convert a string to snake_case.
/// Handles ALL_CAPS words correctly (e.g., "CLIP" -> "clip", "VAE" -> "vae").
fn to_snake_case(s: &str) -> String {
    // If the string is all uppercase, just lowercase it
    if s.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
        return s.to_lowercase();
    }

    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            // Check if we're in the middle of an acronym (current and next are uppercase)
            let next_is_upper = chars.get(i + 1).is_some_and(|&n| n.is_uppercase());
            let prev_is_upper = i > 0 && chars.get(i - 1).is_some_and(|&p| p.is_uppercase());

            // Add underscore before this char if:
            // - We're not at the start
            // - Previous char was lowercase, OR
            // - We're at the end of an acronym (prev is upper, next is lower or end)
            if i > 0 && (!prev_is_upper || (!next_is_upper && i + 1 < chars.len())) {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
