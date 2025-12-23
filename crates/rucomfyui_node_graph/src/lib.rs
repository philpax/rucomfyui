#![deny(missing_docs)]
//! A recreation of the ComfyUI node graph in Rust using [`egui`] and [`rucomfyui`].
//!
//! Wraps around [`egui-snarl`] to provide a node graph for [`rucomfyui`].

use std::collections::HashMap;

use egui_snarl::{
    ui::{SnarlStyle, SnarlWidget},
    InPinId, NodeId, OutPinId, Snarl,
};

use rucomfyui::{
    object_info::ObjectInfo,
    workflow::{WorkflowInput, WorkflowMeta, WorkflowNode, WorkflowNodeId},
    Workflow, WorkflowGraph,
};

pub mod internal;

use internal::{FlowNodeData, FlowUserState, FlowValueType, FlowViewer};

/// A mapping from graph node IDs to workflow node IDs.
///
/// Produced by [`ComfyUiNodeGraph::as_workflow_graph_with_mapping`].
pub type NodeToWorkflowNodeMapping = HashMap<NodeId, WorkflowNodeId>;

/// The main struct for the node graph.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComfyUiNodeGraph {
    /// The snarl graph.
    pub snarl: Snarl<FlowNodeData>,
    /// The user state of the node graph.
    pub user_state: FlowUserState,
    /// The object info for the node graph.
    pub object_info: ObjectInfo,
}

impl ComfyUiNodeGraph {
    /// Create a new node graph.
    pub fn new(object_info: ObjectInfo) -> Self {
        Self {
            snarl: Snarl::new(),
            user_state: FlowUserState::default(),
            object_info,
        }
    }

    /// Clear the graph and reset the state.
    pub fn clear(&mut self) {
        self.snarl = Snarl::new();
        self.user_state.output_images.clear();
    }

    /// Converts this graph to a [`rucomfyui::WorkflowGraph`], complete with a mapping from graph node IDs to workflow node IDs.
    pub fn as_workflow_graph_with_mapping(&self) -> (WorkflowGraph, NodeToWorkflowNodeMapping) {
        let g = WorkflowGraph::new();
        let mut mapping: HashMap<NodeId, WorkflowNodeId> = HashMap::new();
        let mut input_mapping: HashMap<(NodeId, usize), (WorkflowNodeId, String)> = HashMap::new();
        let mut output_mapping: HashMap<(NodeId, usize), (WorkflowNodeId, usize)> = HashMap::new();

        // First pass: create all nodes
        for (node_id, node) in self.snarl.node_ids() {
            let object = &node.object;
            let mut g_node = WorkflowNode::new(object.name.clone())
                .with_meta(WorkflowMeta::new(object.display_name()));

            // Add constant inputs (non-connected inputs)
            for (input_idx, input) in node.inputs.iter().enumerate() {
                // Check if this input is connected
                let in_pin = self.snarl.in_pin(InPinId {
                    node: node_id,
                    input: input_idx,
                });

                if in_pin.remotes.is_empty() {
                    // Not connected - add as constant value
                    let workflow_input = match &input.value {
                        FlowValueType::Array { selected, .. } => {
                            WorkflowInput::String(selected.clone())
                        }
                        FlowValueType::String { value, .. } => WorkflowInput::String(value.clone()),
                        FlowValueType::Float { value, .. } => WorkflowInput::F64(*value),
                        FlowValueType::SignedInt { value, .. } => WorkflowInput::I64(*value),
                        FlowValueType::UnsignedInt { value, .. } => WorkflowInput::U64(*value),
                        FlowValueType::Boolean(value) => WorkflowInput::Boolean(*value),
                        FlowValueType::Other(_) | FlowValueType::Unknown => continue,
                    };
                    g_node.add_input(input.name.clone(), workflow_input);
                } else {
                    // Connected - store for second pass
                    // We'll process connections after all nodes are created
                }
            }

            let g_node_id = g.add_dynamic(g_node);
            mapping.insert(node_id, g_node_id);

            // Store input and output mappings for connection processing
            for (input_idx, input) in node.inputs.iter().enumerate() {
                input_mapping.insert((node_id, input_idx), (g_node_id, input.name.clone()));
            }
            for (output_idx, _output) in node.outputs.iter().enumerate() {
                output_mapping.insert((node_id, output_idx), (g_node_id, output_idx));
            }
        }

        // Second pass: process connections
        for (node_id, node) in self.snarl.node_ids() {
            for (input_idx, _input) in node.inputs.iter().enumerate() {
                let in_pin = self.snarl.in_pin(InPinId {
                    node: node_id,
                    input: input_idx,
                });

                // If there's a connection, add it as a slot input
                if let Some(&remote) = in_pin.remotes.first() {
                    let Some((g_input_node_id, input_name)) =
                        input_mapping.get(&(node_id, input_idx))
                    else {
                        continue;
                    };
                    let Some((g_output_node_id, output_slot)) =
                        output_mapping.get(&(remote.node, remote.output))
                    else {
                        continue;
                    };
                    let Some(mut g_input_node) = g.get_node_mut(*g_input_node_id) else {
                        continue;
                    };
                    g_input_node.add_input(
                        input_name.clone(),
                        WorkflowInput::slot(*g_output_node_id, *output_slot as u32),
                    );
                }
            }
        }

        (g, mapping)
    }

    /// Extracts the [`Workflow`] from this graph.
    ///
    /// Wrapper around [`Self::as_workflow_graph_with_mapping`] that takes the [`WorkflowGraph`] and
    /// extracts the [`Workflow`]. If you need either the original graph or the mapping, use that instead.
    pub fn save_api_workflow(&self) -> Workflow {
        self.as_workflow_graph_with_mapping().0.into_workflow()
    }

    /// Load a [`Workflow`] into this graph.
    pub fn load_api_workflow(&mut self, workflow: &Workflow) -> Result<(), UnknownClassTypeError> {
        let sorted_node_ids = workflow.topological_sort_with_depth();

        let mut mapping = HashMap::<WorkflowNodeId, NodeId>::new();
        let mut node_position = egui::Pos2::ZERO;

        self.clear();

        for node_ids in sorted_node_ids {
            node_position.x += 600.0;
            node_position.y = 0.0;

            for node_id in node_ids {
                let workflow_node = workflow.0.get(&node_id).unwrap();
                let object = self
                    .object_info
                    .get(&workflow_node.class_type)
                    .ok_or_else(|| UnknownClassTypeError {
                        node_id,
                        class_type: workflow_node.class_type.clone(),
                    })?;

                // Create node data with values from workflow
                let mut node_data = FlowNodeData::new(object.clone());

                // Update input values from workflow
                for input in &mut node_data.inputs {
                    if let Some(workflow_input) = workflow_node.inputs.get(&input.name) {
                        // Don't override if it's a slot (connection)
                        if !matches!(workflow_input, WorkflowInput::Slot(_, _)) {
                            // If the input is an Array (dropdown), just update the selected value
                            // rather than replacing the whole input (which would lose the options)
                            if let FlowValueType::Array { selected, .. } = &mut input.value {
                                if let WorkflowInput::String(s) = workflow_input {
                                    *selected = s.clone();
                                    continue;
                                }
                            }

                            let meta_typed = object
                                .all_inputs()
                                .find(|(name, _, _)| name == &input.name.as_str())
                                .and_then(|(_, inp, _)| inp.as_meta_typed());

                            input.value = FlowValueType::from_object_type_and_input(
                                &input.typ,
                                workflow_input,
                                meta_typed,
                            );
                        }
                    }
                }

                // Insert the node into the snarl
                let snarl_node_id = self.snarl.insert_node(node_position, node_data);
                mapping.insert(node_id, snarl_node_id);

                node_position.y += 400.0;
            }
        }

        // Second pass: create connections
        // Collect all connections first to avoid borrow checker issues
        let mut connections = Vec::new();

        for (workflow_node_id, workflow_node) in &workflow.0 {
            let Some(&snarl_node_id) = mapping.get(workflow_node_id) else {
                continue;
            };

            let node = &self.snarl[snarl_node_id];

            for (input_name, workflow_input) in &workflow_node.inputs {
                if let WorkflowInput::Slot(output_node_id_str, output_slot) = workflow_input {
                    // Parse the node ID from string
                    let Ok(output_node_id) = output_node_id_str.parse::<WorkflowNodeId>() else {
                        continue;
                    };

                    // Find the input index for this input name
                    let Some(input_idx) =
                        node.inputs.iter().position(|inp| inp.name == *input_name)
                    else {
                        continue;
                    };

                    // Find the source node
                    let Some(&source_snarl_node_id) = mapping.get(&output_node_id) else {
                        continue;
                    };

                    // Store the connection for later
                    let from = OutPinId {
                        node: source_snarl_node_id,
                        output: *output_slot as usize,
                    };
                    let to = InPinId {
                        node: snarl_node_id,
                        input: input_idx,
                    };

                    connections.push((from, to));
                }
            }
        }

        // Apply all connections
        for (from, to) in connections {
            self.snarl.connect(from, to);
        }

        Ok(())
    }

    /// Render the node graph in the given [`egui::Ui`].
    pub fn show(&mut self, ui: &mut egui::Ui) {
        let mut viewer = FlowViewer {
            user_state: &mut self.user_state,
            object_info: &self.object_info,
        };

        SnarlWidget::new()
            .style(SnarlStyle::default())
            .show(&mut self.snarl, &mut viewer, ui);
    }

    /// Populate the output images for rendering in the graph.
    /// Any node can have images attached to it for display.
    ///
    /// Note that:
    /// - `egui_extras::install_image_loaders` must have been called to register the image loaders
    ///   used to display the images.
    /// - this will clear all existing output images.
    ///
    /// The `unique_prefix` is used to ensure that there are no conflicts with the image URIs
    /// used by `egui`. We recommend using a timestamp for this.
    pub fn populate_output_images(
        &mut self,
        unique_prefix: &str,
        outputs: impl Iterator<Item = (NodeId, Vec<Vec<u8>>)>,
    ) {
        self.user_state.output_images = outputs
            .filter(|(_, images)| !images.is_empty())
            .flat_map(|(node_id, images)| {
                let images = images
                    .iter()
                    .enumerate()
                    .map(|(i, v)| egui::ImageSource::Bytes {
                        uri: format!("bytes://{unique_prefix}_{node_id:?}_{i}.png").into(),
                        bytes: v.clone().into(),
                    })
                    .collect();
                Some((node_id, (images, 0)))
            })
            .collect();
    }
}

#[derive(Debug)]
/// Error for when a node has an unknown class type during workflow loading.
pub struct UnknownClassTypeError {
    /// The node ID of the node with the unknown class type.
    pub node_id: WorkflowNodeId,
    /// The class type of the node.
    pub class_type: String,
}

impl std::fmt::Display for UnknownClassTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "node {} has unknown class type {}",
            self.node_id, self.class_type
        )
    }
}

impl std::error::Error for UnknownClassTypeError {}
