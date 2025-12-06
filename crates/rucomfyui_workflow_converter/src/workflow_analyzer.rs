//! Workflow analysis and topological sorting.

use rucomfyui::workflow::{Workflow, WorkflowInput, WorkflowNodeId};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use crate::{ConversionError, Result};

/// An analyzed workflow with nodes in topological order.
#[derive(Debug, Clone)]
pub struct AnalyzedWorkflow {
    /// Nodes in topological order (dependencies first, outputs last).
    pub nodes: Vec<AnalyzedNode>,
    /// Map from original node ID to generated variable name.
    pub node_var_names: HashMap<WorkflowNodeId, String>,
}

/// An analyzed node with its dependencies resolved.
#[derive(Debug, Clone)]
pub struct AnalyzedNode {
    /// The original node ID.
    pub id: WorkflowNodeId,
    /// The class type of the node (e.g., "CheckpointLoaderSimple").
    pub class_type: String,
    /// Display name from metadata, if available.
    pub display_name: Option<String>,
    /// Inputs with their values or references to other nodes (sorted by key for deterministic output).
    pub inputs: BTreeMap<String, AnalyzedInput>,
    /// The variable name to use for this node in generated code.
    pub var_name: String,
}

/// An analyzed input value.
#[derive(Debug, Clone)]
pub enum AnalyzedInput {
    /// A literal string value.
    String(String),
    /// A literal integer value.
    Integer(i64),
    /// A literal float value.
    Float(f64),
    /// A literal boolean value.
    Boolean(bool),
    /// A reference to another node's output.
    NodeRef {
        /// The variable name of the referenced node.
        var_name: String,
        /// The output slot index.
        slot: u32,
    },
}

impl AnalyzedWorkflow {
    /// Analyze a workflow from JSON string.
    pub fn from_json(json: &str) -> Result<Self> {
        let workflow: Workflow = serde_json::from_str(json)?;
        Self::from_workflow(&workflow)
    }

    /// Analyze a workflow.
    pub fn from_workflow(workflow: &Workflow) -> Result<Self> {
        let sorted_ids = topological_sort(workflow)?;

        // Generate variable names for each node
        let mut node_var_names = HashMap::new();
        let mut name_counts: HashMap<String, usize> = HashMap::new();

        for &id in &sorted_ids {
            let node = workflow
                .0
                .get(&id)
                .ok_or_else(|| ConversionError::NodeNotFound(id.to_string()))?;

            let base_name = class_type_to_var_name(&node.class_type);
            let count = name_counts.entry(base_name.clone()).or_insert(0);
            let var_name = if *count == 0 {
                base_name.clone()
            } else {
                format!("{}_{}", base_name, count)
            };
            *count += 1;
            node_var_names.insert(id, var_name);
        }

        // Build analyzed nodes
        let mut nodes = Vec::new();
        for id in sorted_ids {
            let node = workflow
                .0
                .get(&id)
                .ok_or_else(|| ConversionError::NodeNotFound(id.to_string()))?;

            let var_name = node_var_names.get(&id).unwrap().clone();
            let display_name = node.meta.as_ref().map(|m| m.title().to_string());

            let mut inputs = BTreeMap::new();
            for (name, input) in &node.inputs {
                let analyzed = match input {
                    WorkflowInput::String(s) => AnalyzedInput::String(s.clone()),
                    WorkflowInput::F64(f) => AnalyzedInput::Float(*f),
                    WorkflowInput::I64(i) => AnalyzedInput::Integer(*i),
                    WorkflowInput::U64(u) => AnalyzedInput::Integer(*u as i64),
                    WorkflowInput::Boolean(b) => AnalyzedInput::Boolean(*b),
                    WorkflowInput::Slot(node_id_str, slot) => {
                        let ref_id: WorkflowNodeId = node_id_str.parse().map_err(|_| {
                            ConversionError::InvalidNodeReference(node_id_str.clone())
                        })?;
                        let ref_var_name = node_var_names.get(&ref_id).ok_or_else(|| {
                            ConversionError::InvalidNodeReference(node_id_str.clone())
                        })?;
                        AnalyzedInput::NodeRef {
                            var_name: ref_var_name.clone(),
                            slot: *slot,
                        }
                    }
                };
                inputs.insert(name.clone(), analyzed);
            }

            nodes.push(AnalyzedNode {
                id,
                class_type: node.class_type.clone(),
                display_name,
                inputs,
                var_name,
            });
        }

        Ok(AnalyzedWorkflow {
            nodes,
            node_var_names,
        })
    }
}

/// Convert a class type like "CheckpointLoaderSimple" to a variable name like "checkpoint_loader_simple".
fn class_type_to_var_name(class_type: &str) -> String {
    use convert_case::{Case, Casing};

    // Handle special characters
    let cleaned = class_type
        .replace(".", "_")
        .replace("|", "_")
        .replace(" ", "_");

    // Convert to snake_case
    cleaned.to_case(Case::Snake)
}

/// Perform topological sort on the workflow graph.
/// Returns nodes in dependency order (nodes with no dependencies first).
fn topological_sort(workflow: &Workflow) -> Result<Vec<WorkflowNodeId>> {
    let mut in_degree: HashMap<WorkflowNodeId, usize> = HashMap::new();
    let mut dependents: HashMap<WorkflowNodeId, Vec<WorkflowNodeId>> = HashMap::new();

    // Initialize in_degree for all nodes
    for &node_id in workflow.0.keys() {
        in_degree.entry(node_id).or_insert(0);
        dependents.entry(node_id).or_default();
    }

    // Calculate in-degrees and dependencies
    for (&node_id, node) in &workflow.0 {
        for input in node.inputs.values() {
            if let WorkflowInput::Slot(dep_id_str, _) = input {
                if let Ok(dep_id) = dep_id_str.parse::<u32>() {
                    let dep_node_id = WorkflowNodeId(dep_id);
                    if workflow.0.contains_key(&dep_node_id) {
                        *in_degree.entry(node_id).or_insert(0) += 1;
                        dependents.entry(dep_node_id).or_default().push(node_id);
                    }
                }
            }
        }
    }

    // Kahn's algorithm for topological sort
    let mut queue: VecDeque<WorkflowNodeId> = in_degree
        .iter()
        .filter(|(_, &degree)| degree == 0)
        .map(|(&id, _)| id)
        .collect();

    // Sort queue for deterministic output
    let mut queue_vec: Vec<_> = queue.drain(..).collect();
    queue_vec.sort_by_key(|id| id.0);
    queue = queue_vec.into_iter().collect();

    let mut result = Vec::new();
    let mut visited = HashSet::new();

    while let Some(node_id) = queue.pop_front() {
        if visited.contains(&node_id) {
            continue;
        }
        visited.insert(node_id);
        result.push(node_id);

        // Get dependents and sort them for deterministic output
        if let Some(deps) = dependents.get(&node_id) {
            let mut sorted_deps = deps.to_vec();
            sorted_deps.sort_by_key(|id| id.0);

            for dependent in sorted_deps {
                if let Some(degree) = in_degree.get_mut(&dependent) {
                    *degree = degree.saturating_sub(1);
                    if *degree == 0 && !visited.contains(&dependent) {
                        queue.push_back(dependent);
                    }
                }
            }
        }
    }

    // Check if all nodes were visited (no cycles)
    if result.len() != workflow.0.len() {
        return Err(ConversionError::CyclicWorkflow);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_type_to_var_name() {
        assert_eq!(
            class_type_to_var_name("CheckpointLoaderSimple"),
            "checkpoint_loader_simple"
        );
        assert_eq!(class_type_to_var_name("KSampler"), "k_sampler");
        assert_eq!(class_type_to_var_name("VAEDecode"), "vae_decode");
        assert_eq!(class_type_to_var_name("CLIPTextEncode"), "clip_text_encode");
    }

    #[test]
    fn test_simple_workflow_analysis() {
        let json = r#"{
            "1": {
                "inputs": { "ckpt_name": "model.safetensors" },
                "class_type": "CheckpointLoaderSimple"
            },
            "2": {
                "inputs": {
                    "seed": 0,
                    "steps": 20,
                    "model": ["1", 0]
                },
                "class_type": "KSampler"
            }
        }"#;

        let analyzed = AnalyzedWorkflow::from_json(json).unwrap();
        assert_eq!(analyzed.nodes.len(), 2);

        // First node should be CheckpointLoaderSimple (no dependencies)
        assert_eq!(analyzed.nodes[0].class_type, "CheckpointLoaderSimple");
        assert_eq!(analyzed.nodes[0].var_name, "checkpoint_loader_simple");

        // Second node should be KSampler (depends on first)
        assert_eq!(analyzed.nodes[1].class_type, "KSampler");
        assert_eq!(analyzed.nodes[1].var_name, "k_sampler");

        // Check that KSampler references the checkpoint loader
        if let Some(AnalyzedInput::NodeRef { var_name, slot }) =
            analyzed.nodes[1].inputs.get("model")
        {
            assert_eq!(var_name, "checkpoint_loader_simple");
            assert_eq!(*slot, 0);
        } else {
            panic!("Expected NodeRef for model input");
        }
    }
}
