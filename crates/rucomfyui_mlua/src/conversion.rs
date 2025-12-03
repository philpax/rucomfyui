//! Conversion utilities between Lua values and rucomfyui types.

use crate::node_output::{NodeOutput, NodeOutputs};
use mlua::prelude::*;
use rucomfyui::workflow::WorkflowInput;

/// Convert a Lua value to a WorkflowInput.
///
/// Handles:
/// - Strings -> WorkflowInput::String
/// - Integers -> WorkflowInput::I64
/// - Numbers -> WorkflowInput::F64
/// - Booleans -> WorkflowInput::Boolean
/// - NodeOutput -> WorkflowInput::Slot
/// - NodeOutputs -> WorkflowInput::Slot (uses first output, slot 0)
pub fn lua_to_workflow_input(value: LuaValue) -> LuaResult<WorkflowInput> {
    match value {
        LuaValue::String(s) => Ok(WorkflowInput::String(s.to_str()?.to_string())),
        LuaValue::Integer(i) => Ok(WorkflowInput::I64(i)),
        LuaValue::Number(n) => {
            // Check if it's actually an integer
            if n.fract() == 0.0 && n >= i64::MIN as f64 && n <= i64::MAX as f64 {
                Ok(WorkflowInput::I64(n as i64))
            } else {
                Ok(WorkflowInput::F64(n))
            }
        }
        LuaValue::Boolean(b) => Ok(WorkflowInput::Boolean(b)),
        LuaValue::UserData(ud) => {
            // Try NodeOutput first
            if let Ok(output) = ud.borrow::<NodeOutput>() {
                return Ok(WorkflowInput::Slot(
                    output.node_id.to_string(),
                    output.slot,
                ));
            }
            // Try NodeOutputs (use slot 0 by default)
            if let Ok(outputs) = ud.borrow::<NodeOutputs>() {
                return Ok(WorkflowInput::Slot(outputs.node_id.to_string(), 0));
            }
            Err(LuaError::external(
                "Cannot convert userdata to workflow input",
            ))
        }
        LuaValue::Nil => Err(LuaError::external("Cannot use nil as workflow input")),
        LuaValue::Table(_) => Err(LuaError::external(
            "Cannot use table directly as workflow input (did you mean to pass a node output?)",
        )),
        _ => Err(LuaError::external(format!(
            "Cannot convert {:?} to workflow input",
            value.type_name()
        ))),
    }
}

/// Check if a Lua value represents a node output.
#[allow(dead_code)]
pub fn is_node_output(value: &LuaValue) -> bool {
    match value {
        LuaValue::UserData(ud) => {
            ud.borrow::<NodeOutput>().is_ok() || ud.borrow::<NodeOutputs>().is_ok()
        }
        _ => false,
    }
}

/// Get the node ID from a Lua value if it's a node output.
#[allow(dead_code)]
pub fn get_node_id(value: &LuaValue) -> Option<u32> {
    match value {
        LuaValue::UserData(ud) => {
            if let Ok(output) = ud.borrow::<NodeOutput>() {
                return Some(output.node_id.0);
            }
            if let Ok(outputs) = ud.borrow::<NodeOutputs>() {
                return Some(outputs.node_id.0);
            }
            None
        }
        _ => None,
    }
}
