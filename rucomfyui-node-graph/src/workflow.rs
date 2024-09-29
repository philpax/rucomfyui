use std::collections::HashMap;

use egui_node_graph2::{Graph, InputId, NodeId, OutputId};
use rucomfyui::{
    object_info::ObjectType,
    workflow::{WorkflowInput, WorkflowMeta, WorkflowNode, WorkflowNodeId},
    WorkflowGraph,
};

use crate::{FlowNodeData, FlowValueType};

pub type NodeToWorkflowNodeMapping = HashMap<NodeId, WorkflowNodeId>;

pub fn node_graph_to_workflow_graph(
    graph: &Graph<FlowNodeData, ObjectType, FlowValueType>,
) -> (WorkflowGraph, NodeToWorkflowNodeMapping) {
    let g = WorkflowGraph::new();
    let mut mapping: HashMap<NodeId, WorkflowNodeId> = HashMap::new();
    let mut input_mapping: HashMap<InputId, (WorkflowNodeId, String)> = HashMap::new();
    let mut output_mapping: HashMap<OutputId, (WorkflowNodeId, usize)> = HashMap::new();

    for (node_id, node) in &graph.nodes {
        let object = &node.user_data.template.0;
        let mut g_node = WorkflowNode::new(object.name.clone())
            .with_meta(WorkflowMeta::new(object.display_name.clone()));

        let mut connections = vec![];
        for (input_name, input_id) in &node.inputs {
            let input = graph.inputs.get(*input_id).unwrap();
            let workflow_input = match &input.value {
                FlowValueType::Array { selected, .. } => WorkflowInput::String(selected.clone()),
                FlowValueType::String { value, .. } => WorkflowInput::String(value.clone()),
                FlowValueType::Float { value, .. } => WorkflowInput::F64(*value),
                FlowValueType::SignedInt { value, .. } => WorkflowInput::I64(*value),
                FlowValueType::UnsignedInt { value, .. } => WorkflowInput::U64(*value),
                FlowValueType::Boolean(value) => WorkflowInput::Boolean(*value),
                FlowValueType::Other(_) => {
                    connections.push((*input_id, input_name.clone()));
                    continue;
                }
                FlowValueType::Unknown => continue,
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
