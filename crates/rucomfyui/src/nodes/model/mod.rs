//!`model` definitions/categories.
#![allow(
    unused_imports,
    clippy::too_many_arguments,
    clippy::new_without_default,
    clippy::doc_lazy_continuation
)]
use std::collections::HashMap;
use crate::{
    workflow::{WorkflowNodeId, WorkflowInput},
    nodes::types::Out,
};
#[rustfmt::skip]
pub mod conditioning;
#[rustfmt::skip]
pub mod latent;
#[rustfmt::skip]
pub mod loaders;
#[rustfmt::skip]
pub mod patch;
#[rustfmt::skip]
pub mod sampling;
#[rustfmt::skip]
pub mod training;
