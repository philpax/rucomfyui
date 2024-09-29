//! Workflow graphs for ComfyUI.

use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[cfg(feature = "typed_nodes")]
use crate::nodes::{types::Out, TypedNode};

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
    /// Load a workflow from a string. Convenience wrapper for [`serde_json::from_str`].
    pub fn from_json(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }
    /// Return a topological sort of the nodes in the workflow, with outputs at the end.
    pub fn topological_sort(&self) -> Vec<WorkflowNodeId> {
        self.topological_sort_with_depth()
            .into_iter()
            .flatten()
            .collect()
    }
    /// Return a topological sort of the nodes in the workflow, with nodes of the same depth grouped together, with outputs at the end.
    pub fn topological_sort_with_depth(&self) -> Vec<Vec<WorkflowNodeId>> {
        let mut result = Vec::new();
        let mut in_degree: HashMap<WorkflowNodeId, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        // Calculate in-degree for each node
        for (&node_id, node) in &self.0 {
            in_degree.entry(node_id).or_insert(0);
            for input in node.inputs.values() {
                if let WorkflowInput::Slot(dep_id, _) = input {
                    if let Ok(dep_id) = dep_id.parse::<u32>() {
                        let dep_node_id = WorkflowNodeId(dep_id);
                        *in_degree.entry(dep_node_id).or_insert(0) += 1;
                    }
                }
            }
        }

        // Add nodes with in-degree 0 to the queue
        for (&node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_id);
            }
        }

        // Process the queue
        while !queue.is_empty() {
            let mut current_depth = Vec::new();

            for _ in 0..queue.len() {
                if let Some(node_id) = queue.pop_front() {
                    if visited.contains(&node_id) {
                        continue;
                    }
                    visited.insert(node_id);
                    current_depth.push(node_id);

                    if let Some(node) = self.0.get(&node_id) {
                        for input in node.inputs.values() {
                            if let WorkflowInput::Slot(dep_id, _) = input {
                                if let Ok(dep_id) = dep_id.parse::<u32>() {
                                    let dep_node_id = WorkflowNodeId(dep_id);
                                    if let Some(degree) = in_degree.get_mut(&dep_node_id) {
                                        *degree -= 1;
                                        if *degree == 0 {
                                            queue.push_back(dep_node_id);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if !current_depth.is_empty() {
                result.push(current_depth);
            }
        }

        // Check for cycles
        let total_nodes: usize = result.iter().map(|v| v.len()).sum();
        if total_nodes != self.0.len() {
            panic!("Cyclic dependency detected in the workflow");
        }

        result.reverse();

        result
    }
}
impl FromIterator<(WorkflowNodeId, WorkflowNode)> for Workflow {
    fn from_iter<T: IntoIterator<Item = (WorkflowNodeId, WorkflowNode)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}
#[cfg(feature = "typed_nodes")]
/// Trait that allows using multiple [`Out`] types as outputs for [`WorkflowGraph::add_typed_dynamic`].
pub trait TypedOut: Sized {
    /// Provide the node ID for the output.
    fn provide_node_id(node_id: WorkflowNodeId) -> Self;
}

#[cfg(feature = "typed_nodes")]
impl<T: Out> TypedOut for T {
    fn provide_node_id(node_id: WorkflowNodeId) -> Self {
        T::from_dynamic(node_id, 0)
    }
}
#[cfg(feature = "typed_nodes")]
impl<T: Out> TypedOut for (T,) {
    fn provide_node_id(node_id: WorkflowNodeId) -> Self {
        (T::from_dynamic(node_id, 0),)
    }
}
#[cfg(feature = "typed_nodes")]
macro_rules! impl_typed_out_tuples {
    ($(($($name:ident),+)),+) => {
        $(
            impl<$($name: Out),+> TypedOut for ($($name),+) {
                fn provide_node_id(node_id: WorkflowNodeId) -> Self {
                    let mut i = 0;
                    ($(
                        $name::from_dynamic(node_id, { i += 1; i - 1 })
                    ),+)
                }
            }
        )+
    };
}
#[cfg(feature = "typed_nodes")]
impl_typed_out_tuples!(
    (A, B),
    (A, B, C),
    (A, B, C, D),
    (A, B, C, D, E),
    (A, B, C, D, E, F),
    (A, B, C, D, E, F, G),
    (A, B, C, D, E, F, G, H)
);

/// A workflow graph constructs a [`Workflow`] by adding nodes to it.
///
/// The [`Workflow`] can be retrieved using the [`Into`] implementation or through [`Self::borrow`].
///
/// This type uses interior mutability to make it easier to add multiple nodes to the graph
/// in a nested fashion. Ensure that you only add nodes when the graph is not being borrowed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct WorkflowGraph {
    workflow: RefCell<Workflow>,
    last_node: RefCell<WorkflowNodeId>,
}
impl From<WorkflowGraph> for Workflow {
    fn from(value: WorkflowGraph) -> Self {
        value.into_workflow()
    }
}
impl WorkflowGraph {
    /// Add a dynamic node to the workflow.
    pub fn add_dynamic(&self, node: impl Into<WorkflowNode>) -> WorkflowNodeId {
        let id = WorkflowNodeId(self.last_node.borrow().0 + 1);
        self.workflow.borrow_mut().0.insert(id, node.into());
        self.last_node.replace(id);
        id
    }

    #[cfg(feature = "typed_nodes")]
    /// Add a typed node to the workflow.
    pub fn add<T: TypedNode>(&self, node: T) -> T::Output {
        let node_id = self.add_dynamic(WorkflowNode {
            inputs: node.inputs(),
            class_type: T::NAME.to_string(),
            meta: Some(WorkflowMeta::new(T::DISPLAY_NAME)),
        });
        node.output(node_id)
    }

    #[cfg(feature = "typed_nodes")]
    /// Add a dynamic node to the workflow with the given typed output.
    ///
    /// Ensure that you verify the types of the output - the library will not check for you!
    pub fn add_typed_dynamic<Output: TypedOut>(&self, node: impl Into<WorkflowNode>) -> Output {
        Output::provide_node_id(self.add_dynamic(node))
    }

    /// Borrow the workflow.
    pub fn borrow(&self) -> Ref<'_, Workflow> {
        self.workflow.borrow()
    }

    /// Consume this type, returning the inner workflow.
    pub fn into_workflow(self) -> Workflow {
        self.workflow.into_inner()
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
    /// A F64 input.
    F64(f64),
    /// A i64 input.
    I64(i64),
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
    /// Get the string value of this input, if it is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(v) => Some(v.as_str()),
            _ => None,
        }
    }
    /// Get the `f64` value of this input, if it is a `f64`.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::F64(v) => Some(*v),
            _ => None,
        }
    }
    /// Get the `i64` value of this input, if it is a `i64`.
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::I64(v) => Some(*v),
            _ => None,
        }
    }
    /// Get the `bool` value of this input, if it is a `bool`.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(v) => Some(*v),
            _ => None,
        }
    }
    /// Get the slot value of this input, if it is a slot.
    pub fn as_slot(&self) -> Option<(WorkflowNodeId, u32)> {
        match self {
            Self::Slot(node_id, slot_index) => {
                Some((WorkflowNodeId(node_id.parse::<u32>().ok()?), *slot_index))
            }
            _ => None,
        }
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
        WorkflowInput::F64(value as f64)
    }
}
impl From<f64> for WorkflowInput {
    fn from(value: f64) -> Self {
        WorkflowInput::F64(value)
    }
}
impl From<i32> for WorkflowInput {
    fn from(value: i32) -> Self {
        WorkflowInput::I64(value as i64)
    }
}
impl From<u32> for WorkflowInput {
    fn from(value: u32) -> Self {
        WorkflowInput::I64(value as i64)
    }
}
impl From<i64> for WorkflowInput {
    fn from(value: i64) -> Self {
        WorkflowInput::I64(value)
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
