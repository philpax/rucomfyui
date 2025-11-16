//!`image` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod bfl;
pub mod byte_dance;
pub mod gemini;
pub mod ideogram;
pub mod kling;
pub mod luma;
pub mod open_ai;
pub mod recraft;
pub mod runway;
pub mod stability_ai;
pub mod wan;
