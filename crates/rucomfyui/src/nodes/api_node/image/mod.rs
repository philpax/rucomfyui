//!`image` definitions/categories.
#![allow(unused_imports, clippy::too_many_arguments, clippy::new_without_default)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod bfl;
#[rustfmt::skip]
pub mod byte_dance;
#[rustfmt::skip]
pub mod gemini;
#[rustfmt::skip]
pub mod ideogram;
#[rustfmt::skip]
pub mod kling;
#[rustfmt::skip]
pub mod luma;
#[rustfmt::skip]
pub mod open_ai;
#[rustfmt::skip]
pub mod recraft;
#[rustfmt::skip]
pub mod runway;
#[rustfmt::skip]
pub mod stability_ai;
#[rustfmt::skip]
pub mod wan;
