use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A workflow is a graph of nodes that are executed in order.
/// Each node is a step in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Workflow(
    /// The nodes in the workflow.
    pub HashMap<u32, WorkflowNode>,
);
impl Workflow {
    /// Create a new workflow.
    pub fn new(nodes: impl IntoIterator<Item = (u32, WorkflowNode)>) -> Self {
        Self(HashMap::from_iter(nodes))
    }
}
impl FromIterator<(u32, WorkflowNode)> for Workflow {
    fn from_iter<T: IntoIterator<Item = (u32, WorkflowNode)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

/// A workflow graph constructs a [`Workflow`] by adding nodes to it.
///
/// The [`Workflow`] can be retrieved using the [`Into`] implementation or through the [`AsRef`] implementation.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct WorkflowGraph {
    workflow: Workflow,
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
        let id = self.last_node + 1;
        self.workflow.0.insert(id, node.into());
        self.last_node = id;
        id
    }
}

/// A workflow node ID.
pub type WorkflowNodeId = u32;

/// A node in the workflow.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WorkflowNode {
    /// The inputs to the node.
    pub inputs: HashMap<String, WorkflowInput>,
    /// The type of node, e.g. "CLIPTextEncode".
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
    /// A slot input. First value is the node ID, second is the slot index.
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
