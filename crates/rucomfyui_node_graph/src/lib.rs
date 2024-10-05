#![deny(missing_docs)]
//! A recreation of the ComfyUI node graph in Rust using [`egui`] and [`rucomfyui`].
//!
//! Wraps around [`egui_node_graph2`] to provide a node graph for [`rucomfyui`].

use std::collections::HashMap;

use egui_node_graph2::*;

use rucomfyui::{
    object_info::ObjectInfo,
    workflow::{WorkflowInput, WorkflowMeta, WorkflowNode, WorkflowNodeId},
    Workflow, WorkflowGraph,
};

pub mod internal;

/// A mapping from graph node IDs to workflow node IDs.
///
/// Produced by [`ComfyUiNodeGraph::as_workflow_graph_with_mapping`].
pub type NodeToWorkflowNodeMapping = HashMap<NodeId, WorkflowNodeId>;

/// The main struct for the node graph.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComfyUiNodeGraph {
    /// The state of the node graph.
    pub state: internal::FlowEditorState,
    /// The user state of the node graph.
    pub user_state: internal::FlowUserState,
    /// The object info for the node graph.
    pub object_info: ObjectInfo,
}
impl ComfyUiNodeGraph {
    /// Create a new node graph.
    pub fn new(object_info: ObjectInfo) -> Self {
        Self {
            state: internal::FlowEditorState::default(),
            user_state: internal::FlowUserState::default(),
            object_info,
        }
    }

    /// Clear the graph and reset the state.
    pub fn clear(&mut self) {
        let state = &mut self.state;
        state.node_finder = None;
        state.node_order.clear();
        state.node_positions.clear();
        state.selected_nodes.clear();
        state.graph.connections.clear();
        state.graph.inputs.clear();
        state.graph.outputs.clear();
        state.graph.nodes.clear();
    }

    /// Converts this graph to a [`rucomfyui::WorkflowGraph`], complete with a mapping from graph node IDs to workflow node IDs.
    pub fn as_workflow_graph_with_mapping(&self) -> (WorkflowGraph, NodeToWorkflowNodeMapping) {
        let g = WorkflowGraph::new();
        let mut mapping: HashMap<NodeId, WorkflowNodeId> = HashMap::new();
        let mut input_mapping: HashMap<InputId, (WorkflowNodeId, String)> = HashMap::new();
        let mut output_mapping: HashMap<OutputId, (WorkflowNodeId, usize)> = HashMap::new();

        let graph = &self.state.graph;
        for (node_id, node) in &graph.nodes {
            let object = &node.user_data.template.0;
            let mut g_node = WorkflowNode::new(object.name.clone())
                .with_meta(WorkflowMeta::new(object.display_name.clone()));

            let mut connections = vec![];
            for (input_name, input_id) in &node.inputs {
                let input = graph.inputs.get(*input_id).unwrap();
                use internal::FlowValueType as FVT;
                let workflow_input = match &input.value {
                    FVT::Array { selected, .. } => WorkflowInput::String(selected.clone()),
                    FVT::String { value, .. } => WorkflowInput::String(value.clone()),
                    FVT::Float { value, .. } => WorkflowInput::F64(*value),
                    FVT::SignedInt { value, .. } => WorkflowInput::I64(*value),
                    FVT::UnsignedInt { value, .. } => WorkflowInput::U64(*value),
                    FVT::Boolean(value) => WorkflowInput::Boolean(*value),
                    FVT::Other(_) => {
                        connections.push((*input_id, input_name.clone()));
                        continue;
                    }
                    FVT::Unknown => continue,
                };
                g_node.add_input(input_name.clone(), workflow_input);
            }

            let g_node_id = g.add_dynamic(g_node);
            mapping.insert(node_id, g_node_id);
            for (input_id, input_name) in connections {
                input_mapping.insert(input_id, (g_node_id, input_name));
            }
            for (output_slot, (_output_name, output_id)) in node.outputs.iter().enumerate() {
                output_mapping.insert(*output_id, (g_node_id, output_slot));
            }
        }

        for (input_id, output_ids) in &graph.connections {
            let Some(output_id) = output_ids.first().copied() else {
                continue;
            };

            let Some((g_input_node_id, input_name)) = input_mapping.get(&input_id) else {
                continue;
            };
            let Some((g_output_node_id, output_slot)) = output_mapping.get(&output_id) else {
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

        let mut mapping = HashMap::<WorkflowNodeId, internal::BuildNodeOutput>::new();
        let mut node_position = egui::Pos2::ZERO;

        self.clear();
        let state = &mut self.state;
        for node_ids in sorted_node_ids {
            node_position.x += 300.0;
            node_position.y = 0.0;

            for node_id in node_ids {
                let node = workflow.0.get(&node_id).unwrap();
                let object = self.object_info.get(&node.class_type).ok_or_else(|| {
                    UnknownClassTypeError {
                        node_id,
                        class_type: node.class_type.clone(),
                    }
                })?;
                let template = internal::FlowNodeTemplate(object.clone());
                let g_node_id = state.graph.add_node(
                    object.display_name.clone(),
                    template.user_data(&mut self.user_state),
                    |g, g_node_id| {
                        let bno = internal::build_node(&template, g, g_node_id, Some(node));
                        for (name, input) in node.inputs.iter() {
                            let Some(&input_id) = bno.input_ids.get(name) else {
                                continue;
                            };
                            let Some((output_node_id, slot)) = input.as_slot() else {
                                continue;
                            };
                            let Some(output_bno) = mapping.get(&output_node_id) else {
                                continue;
                            };
                            let Some(&output_id) = output_bno.output_ids.get(slot as usize) else {
                                continue;
                            };
                            g.add_connection(output_id, input_id, 0);
                        }
                        mapping.insert(node_id, bno);
                    },
                );

                state.node_positions.insert(g_node_id, node_position);
                node_position.y += 200.0;
                state.node_order.push(g_node_id);
            }
        }

        Ok(())
    }

    /// Render the node graph in the given [`egui::Ui`].
    pub fn show(&mut self, ui: &mut egui::Ui) {
        let _ = self.state.draw_graph_editor(
            ui,
            internal::NodeTemplates(&self.object_info),
            &mut self.user_state,
            Vec::default(),
        );
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
