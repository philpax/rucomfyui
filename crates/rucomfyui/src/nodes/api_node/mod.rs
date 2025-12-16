//!`api node` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod n_3_d;
#[rustfmt::skip]
pub mod audio;
#[rustfmt::skip]
pub mod image;
#[rustfmt::skip]
pub mod text;
#[rustfmt::skip]
pub mod video;
