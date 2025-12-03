//!`advanced` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod conditioning;
#[rustfmt::skip]
pub mod debug;
#[rustfmt::skip]
pub mod guidance;
#[rustfmt::skip]
pub mod hooks;
#[rustfmt::skip]
pub mod loaders;
#[rustfmt::skip]
pub mod model;
#[rustfmt::skip]
pub mod model_merging;
#[rustfmt::skip]
pub mod model_patches;
