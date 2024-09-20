//! Workflow graphs for ComfyUI.

use std::{collections::HashMap, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[cfg(feature = "typed_nodes")]
use crate::nodes::TypedNode;

/// A workflow is a graph of nodes that are executed in order.
/// Each node is a step in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Workflow(
    /// The nodes in the workflow.
    pub HashMap<WorkflowNodeId, WorkflowNode>,
);
impl Workflow {
    /// Create a new workflow.
    pub fn new(nodes: impl IntoIterator<Item = (WorkflowNodeId, WorkflowNode)>) -> Self {
        Self::from_iter(nodes)
    }
}
impl FromIterator<(WorkflowNodeId, WorkflowNode)> for Workflow {
    fn from_iter<T: IntoIterator<Item = (WorkflowNodeId, WorkflowNode)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

/// A workflow graph constructs a [`Workflow`] by adding nodes to it.
///
/// The [`Workflow`] can be retrieved using the [`Into`] implementation or through the [`AsRef`] implementation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct WorkflowGraph {
    /// The workflow being constructed.
    pub workflow: Workflow,
    /// The last node ID used.
    last_node: WorkflowNodeId,
}
impl From<WorkflowGraph> for Workflow {
    fn from(value: WorkflowGraph) -> Self {
        value.workflow
    }
}
impl AsRef<Workflow> for WorkflowGraph {
    fn as_ref(&self) -> &Workflow {
        &self.workflow
    }
}
impl WorkflowGraph {
    /// Add a node to the workflow.
    pub fn add(&mut self, node: impl Into<WorkflowNode>) -> WorkflowNodeId {
        let id = WorkflowNodeId(self.last_node.0 + 1);
        self.workflow.0.insert(id, node.into());
        self.last_node = id;
        id
    }

    #[cfg(feature = "typed_nodes")]
    /// Add a typed node to the workflow.
    pub fn add_typed<T: TypedNode>(&mut self, node: T) -> T::Output {
        let node_id = self.add(WorkflowNode {
            inputs: node.inputs(),
            class_type: T::NAME.to_string(),
            meta: Some(WorkflowMeta::new(T::DISPLAY_NAME)),
        });
        node.output(node_id)
    }
}

/// A workflow node ID.
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
#[serde(transparent)]
pub struct WorkflowNodeId(pub u32);
impl WorkflowNodeId {
    /// Convert to a [`WorkflowInput`] with a slot.
    pub fn to_input_with_slot(self, slot: u32) -> WorkflowInput {
        WorkflowInput::slot(self, slot)
    }
}
impl Display for WorkflowNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl FromStr for WorkflowNodeId {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

/// A node in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WorkflowNode {
    /// The inputs to the node.
    pub inputs: HashMap<String, WorkflowInput>,
    /// The type of node, e.g. `CLIPTextEncode`.
    pub class_type: String,
    #[serde(rename = "_meta")]
    /// The metadata for the node.
    pub meta: Option<WorkflowMeta>,
}
impl WorkflowNode {
    /// Create a new workflow node.
    pub fn new(class_type: impl Into<String>) -> Self {
        Self {
            inputs: HashMap::default(),
            class_type: class_type.into(),
            meta: None,
        }
    }
    /// Set the inputs for the node.
    pub fn with_input(mut self, key: impl Into<String>, value: impl Into<WorkflowInput>) -> Self {
        self.inputs.insert(key.into(), value.into());
        self
    }
    /// Set the metadata for the node.
    pub fn with_meta(mut self, meta: WorkflowMeta) -> Self {
        self.meta = Some(meta);
        self
    }
}

/// A workflow input.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum WorkflowInput {
    /// A string input.
    String(String),
    /// A f32 input.
    F32(f32),
    /// A i32 input.
    I32(i32),
    /// A boolean input.
    Boolean(bool),
    /// A slot input. First value is the node ID (integer-as-string), second is the slot index.
    Slot(String, u32),
}
impl WorkflowInput {
    /// Create a new slot input.
    pub fn slot(node_id: WorkflowNodeId, slot_index: u32) -> Self {
        WorkflowInput::Slot(node_id.to_string(), slot_index)
    }
}
impl From<String> for WorkflowInput {
    fn from(value: String) -> Self {
        WorkflowInput::String(value)
    }
}
impl From<&str> for WorkflowInput {
    fn from(value: &str) -> Self {
        WorkflowInput::String(value.to_string())
    }
}
impl From<f32> for WorkflowInput {
    fn from(value: f32) -> Self {
        WorkflowInput::F32(value)
    }
}
impl From<i32> for WorkflowInput {
    fn from(value: i32) -> Self {
        WorkflowInput::I32(value)
    }
}
impl From<bool> for WorkflowInput {
    fn from(value: bool) -> Self {
        WorkflowInput::Boolean(value)
    }
}

/// Metadata for a node.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WorkflowMeta {
    /// The title of the node, shown in the UI.
    title: String,
}
impl WorkflowMeta {
    /// Create a new workflow meta.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}
