//!`advanced` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
pub mod conditioning;
pub mod debug;
pub mod guidance;
pub mod hooks;
pub mod loaders;
pub mod model;
pub mod model_merging;
