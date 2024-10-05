//! Internal implementation details of this crate, including the types used
//! to represent the graph and its nodes in terms of [`egui_node_graph2`].
//!
//! Provided for serialization and deserialization purposes, as well as
//! keeping to the general ethos of `egui_node_graph2`.

use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{Hash, Hasher},
};

use egui_node_graph2::*;

use rucomfyui::{
    object_info::{
        Object, ObjectInfo, ObjectInputMetaTyped, ObjectInputMetaTypedRoundValue, ObjectInputType,
        ObjectType,
    },
    workflow::{WorkflowInput, WorkflowNode},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The data for a node in the graph.
pub struct FlowNodeData {
    /// The template for the node.
    pub template: FlowNodeTemplate,
    /// The input tooltips for the node where the key is the input name.
    pub input_tooltips: HashMap<String, String>,
    /// The output tooltips for the node where the key is the output name.
    pub output_tooltips: HashMap<String, String>,
}
impl NodeDataTrait for FlowNodeData {
    type Response = EmptyResponse;
    type UserState = FlowUserState;
    type DataType = ObjectType;
    type ValueType = FlowValueType;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        _graph: &Graph<Self, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<EmptyResponse, FlowNodeData>>
    where
        EmptyResponse: UserResponseTrait,
    {
        if let Some((images, selected)) = user_state.output_images.get_mut(&node_id) {
            ui.horizontal(|ui| {
                for idx in 0..images.len() {
                    if ui
                        .add(
                            egui::Button::new(format!("{}", idx + 1))
                                .small()
                                .selected(*selected == idx),
                        )
                        .clicked()
                    {
                        *selected = idx;
                    }
                }
            });

            if let Some(image) = images.get(*selected) {
                ui.image(image.clone());
            }
        }
        vec![]
    }
    fn output_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: NodeId,
        graph: &Graph<Self, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        param_name: &str,
    ) -> Vec<NodeResponse<Self::Response, Self>>
    where
        Self::Response: UserResponseTrait,
    {
        let r = ui.label(param_name);
        if let Some(tooltip) = graph
            .nodes
            .get(node_id)
            .and_then(|n| n.user_data.output_tooltips.get(param_name))
        {
            r.on_hover_text(tooltip);
        }

        Default::default()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The value type for a node in the graph.
pub enum FlowValueType {
    /// An array of options.
    Array {
        /// The options.
        options: Vec<String>,
        /// The selected option.
        selected: String,
    },
    /// A string.
    String {
        /// The value.
        value: String,
        /// Whether the string should be rendered as a multiline text box.
        multiline: bool,
    },
    /// A floating point number ([`f64`]).
    Float {
        /// The value.
        value: f64,
        /// The minimum value.
        min: f64,
        /// The maximum value.
        max: f64,
        /// The value to round to increments of.
        round: Option<f64>,
        /// The amount to increment by.
        step: f64,
    },
    /// A signed integer ([`i64`]).
    SignedInt {
        /// The value.
        value: i64,
        /// The minimum value.
        min: i64,
        /// The maximum value.
        max: i64,
        /// The amount to increment by.
        step: i64,
    },
    /// An unsigned integer ([`u64`]).
    UnsignedInt {
        /// The value.
        value: u64,
        /// The minimum value.
        min: u64,
        /// The maximum value.
        max: u64,
        /// The amount to increment by.
        step: u64,
    },
    /// A boolean ([`bool`]).
    Boolean(bool),
    /// A non-primitive type (i.e. a connection).
    Other(ObjectType),
    #[default]
    /// An unknown type. Should not occur in practice.
    Unknown,
}
impl FlowValueType {
    fn convert_float(value: Option<f64>, typed_meta: Option<&ObjectInputMetaTyped>) -> Self {
        let typed_meta = typed_meta.and_then(|m| m.as_number());
        Self::Float {
            value: value
                .or(typed_meta.map(|m| f64::from(m.default)))
                .unwrap_or(0.0),
            min: typed_meta.map(|m| m.min.into()).unwrap_or(f64::MIN),
            max: typed_meta.map(|m| m.max.into()).unwrap_or(f64::MAX),
            round: typed_meta.and_then(|m| m.round).and_then(|r| match r {
                ObjectInputMetaTypedRoundValue::Bool(b) => b.then_some(1.0),
                ObjectInputMetaTypedRoundValue::Number(v) => Some(v),
            }),
            step: typed_meta
                .and_then(|m| m.step)
                .map(|s| s.into())
                .unwrap_or(1.0),
        }
    }
    fn convert_string(value: &str, typed_meta: Option<&ObjectInputMetaTyped>) -> Self {
        Self::String {
            value: value.into(),
            multiline: typed_meta
                .and_then(|m| m.as_string())
                .and_then(|m| m.multiline)
                .unwrap_or(false),
        }
    }
    fn convert_i64(value: Option<i64>, typed_meta_orig: Option<&ObjectInputMetaTyped>) -> Self {
        let typed_meta = typed_meta_orig.and_then(|m| m.as_number());
        if typed_meta.is_some_and(|m| u64::from(m.min) == 0)
            || typed_meta.is_some_and(|m| u64::from(m.max) >= i64::MAX as u64)
        {
            // HACK: If the min is 0 or the max is greater than i64::MAX, then it's probably a u64
            return Self::convert_u64(value.map(|v| v as u64), typed_meta_orig);
        }
        Self::SignedInt {
            value: value
                .or(typed_meta.map(|m| i64::from(m.default)))
                .unwrap_or(0),
            min: typed_meta.map(|m| i64::from(m.min)).unwrap_or(i64::MIN),
            max: typed_meta.map(|m| i64::from(m.max)).unwrap_or(i64::MAX),
            step: typed_meta.and_then(|m| m.step).map(i64::from).unwrap_or(1),
        }
    }
    fn convert_u64(value: Option<u64>, typed_meta: Option<&ObjectInputMetaTyped>) -> Self {
        let typed_meta = typed_meta.and_then(|m| m.as_number());
        Self::UnsignedInt {
            value: value
                .or(typed_meta.map(|m| u64::from(m.default)))
                .unwrap_or(0),
            min: typed_meta.map(|m| u64::from(m.min)).unwrap_or(0),
            max: typed_meta.map(|m| u64::from(m.max)).unwrap_or(u64::MAX),
            step: typed_meta.and_then(|m| m.step).map(u64::from).unwrap_or(1),
        }
    }
    fn convert_bool(value: Option<bool>, typed_meta: Option<&ObjectInputMetaTyped>) -> Self {
        Self::Boolean(
            value
                .or(typed_meta.and_then(|m| m.as_boolean()).map(|m| m.default))
                .unwrap_or_default(),
        )
    }

    fn from_object_type(
        object_type: &ObjectType,
        typed_meta: Option<&ObjectInputMetaTyped>,
    ) -> Self {
        match object_type {
            ObjectType::Boolean => Self::convert_bool(None, typed_meta),
            ObjectType::Float => Self::convert_float(None, typed_meta),
            ObjectType::Int => Self::convert_i64(None, typed_meta),
            ObjectType::String => Self::convert_string("", typed_meta),
            _ => Self::Other(object_type.clone()),
        }
    }
    fn from_object_type_and_input(
        object_type: &ObjectType,
        input: &WorkflowInput,
        typed_meta: Option<&ObjectInputMetaTyped>,
    ) -> Self {
        match input {
            WorkflowInput::String(s) => Self::convert_string(s, typed_meta),
            WorkflowInput::F64(v) => Self::convert_float(Some(*v), typed_meta),
            WorkflowInput::I64(v) => Self::convert_i64(Some(*v), typed_meta),
            WorkflowInput::U64(v) => Self::convert_u64(Some(*v), typed_meta),
            WorkflowInput::Boolean(b) => Self::convert_bool(Some(*b), typed_meta),
            WorkflowInput::Slot(_, _) => Self::Other(object_type.clone()),
        }
    }

    #[must_use]
    /// Returns whether this value type is connection-only.
    pub fn is_connection_only(&self) -> bool {
        matches!(self, Self::Other(..)) || matches!(self, Self::Unknown)
    }
    #[must_use]
    /// Returns whether this value type is constant-only.
    pub fn is_constant_only(&self) -> bool {
        matches!(self, Self::Array { .. })
            | matches!(self, Self::String { .. })
            | matches!(self, Self::Float { .. })
            | matches!(self, Self::SignedInt { .. })
            | matches!(self, Self::UnsignedInt { .. })
            | matches!(self, Self::Boolean(..))
    }
}
impl WidgetValueTrait for FlowValueType {
    type Response = EmptyResponse;
    type UserState = FlowUserState;
    type NodeData = FlowNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut FlowUserState,
        node_data: &FlowNodeData,
    ) -> Vec<EmptyResponse> {
        let r = ui.label(param_name);
        if let Some(tooltip) = node_data.input_tooltips.get(param_name) {
            r.on_hover_text(tooltip);
        }
        match self {
            FlowValueType::Array { options, selected } => {
                egui::ComboBox::new(format!("{param_name}_checkbox"), "")
                    .selected_text(selected.clone())
                    .show_ui(ui, |ui| {
                        for option in options {
                            ui.selectable_value(selected, option.clone(), option.clone());
                        }
                    });
            }
            FlowValueType::String { value, multiline } => {
                if *multiline {
                    ui.text_edit_multiline(value);
                } else {
                    ui.text_edit_singleline(value);
                }
            }
            FlowValueType::Float {
                value,
                min,
                max,
                round,
                step,
            } => {
                ui.add(egui::DragValue::new(value).range(*min..=*max).speed(*step));
                *value = round.map(|r| (*value / r).round() * r).unwrap_or(*value);
            }
            FlowValueType::SignedInt {
                value,
                min,
                max,
                step,
            } => {
                ui.add(
                    egui::DragValue::new(value)
                        .range(*min..=*max)
                        .speed(*step as f64),
                );
            }
            FlowValueType::UnsignedInt {
                value,
                min,
                max,
                step,
            } => {
                ui.add(
                    egui::DragValue::new(value)
                        .range(*min..=*max)
                        .speed(*step as f64),
                );
            }
            FlowValueType::Boolean(b) => {
                ui.checkbox(b, "");
            }
            FlowValueType::Other(_) => {}
            FlowValueType::Unknown => {
                ui.label("Unknown, this should not happen");
            }
        }

        Vec::new()
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The template for a node in the graph.
pub struct FlowNodeTemplate(pub Object);
impl NodeTemplateTrait for FlowNodeTemplate {
    type NodeData = FlowNodeData;
    type DataType = ObjectType;
    type ValueType = FlowValueType;
    type UserState = FlowUserState;
    type CategoryType = String;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> Cow<'_, str> {
        self.0.display_name.as_str().into()
    }
    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<String> {
        vec![self.0.category.clone()]
    }
    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into()
    }
    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        let input_tooltips = self
            .0
            .all_inputs()
            .flat_map(|(name, input, _)| Some((name.to_string(), input.tooltip()?.to_owned())))
            .collect();
        let output_tooltips = self
            .0
            .processed_output()
            .flat_map(|o| Some((o.name.to_owned(), o.tooltip?.to_owned())))
            .collect();
        FlowNodeData {
            template: self.clone(),
            input_tooltips,
            output_tooltips,
        }
    }
    fn build_node(
        &self,
        graph: &mut Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: NodeId,
    ) {
        build_node(self, graph, node_id, None);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// The response for a node in the graph. Currently empty.
pub struct EmptyResponse;
impl UserResponseTrait for EmptyResponse {}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The user state of the node graph.
///
/// Currently used to store output images for nodes to display.
pub struct FlowUserState {
    #[cfg_attr(feature = "serde", serde(skip))]
    /// A mapping from node IDs to output images and the selected image.
    pub output_images: HashMap<NodeId, (Vec<egui::ImageSource<'static>>, usize)>,
}

impl DataTypeTrait<FlowUserState> for ObjectType {
    fn data_type_color(&self, _user_state: &mut FlowUserState) -> egui::Color32 {
        let mut hasher = std::hash::DefaultHasher::new();
        format!("{self:?}").hash(&mut hasher);
        let hash = hasher.finish();
        let hash = (hash % 3600) as f32 / 3600.0;
        egui::ecolor::Hsva::new(hash, 0.5, 0.5, 1.0).into()
    }

    fn name(&self) -> Cow<'_, str> {
        format!("{self:?}").into()
    }
}

/// The output of [`build_node`].
pub struct BuildNodeOutput {
    /// A mapping from input names to input IDs.
    pub input_ids: HashMap<String, InputId>,
    /// A list of output IDs in the order associated with the template.
    pub output_ids: Vec<OutputId>,
}
/// Build a node in the graph given a template, graph, and node ID.
///
/// A workflow node can be provided to set the initial values of the node,
/// which can be useful when loading a workflow in.
pub fn build_node(
    template: &FlowNodeTemplate,
    graph: &mut Graph<FlowNodeData, ObjectType, FlowValueType>,
    node_id: NodeId,
    workflow_node: Option<&WorkflowNode>,
) -> BuildNodeOutput {
    let mut input_ids = HashMap::new();
    let mut output_ids = vec![];

    let mut sorted_inputs = vec![];
    for (name, input, _required) in template.0.all_inputs() {
        let workflow_input = workflow_node.and_then(|n| n.inputs.get(name));

        let meta_typed = input.as_meta_typed();
        let (type_, value_type) = match input.as_input_type() {
            ObjectInputType::Array(vec) => (
                ObjectType::String,
                FlowValueType::Array {
                    options: vec.clone(),
                    selected: workflow_input
                        .and_then(|i| i.as_str())
                        .map(|s| s.to_string())
                        .or_else(|| vec.first().cloned())
                        .unwrap_or_default(),
                },
            ),
            ObjectInputType::Typed(object_type) => (
                object_type.clone(),
                workflow_input
                    .map(|input| {
                        FlowValueType::from_object_type_and_input(object_type, input, meta_typed)
                    })
                    .unwrap_or_else(|| FlowValueType::from_object_type(object_type, meta_typed)),
            ),
        };

        let input_param_kind = if value_type.is_connection_only() {
            InputParamKind::ConnectionOnly
        } else if value_type.is_constant_only() {
            InputParamKind::ConstantOnly
        } else {
            InputParamKind::ConnectionOrConstant
        };

        sorted_inputs.push((
            name.to_string(),
            type_.clone(),
            value_type,
            input_param_kind,
        ));
    }
    // Sort inputs by input param kind
    sorted_inputs.sort_by_key(|(_, _, _, input_param_kind)| match input_param_kind {
        InputParamKind::ConnectionOnly => 0,
        InputParamKind::ConnectionOrConstant | InputParamKind::ConstantOnly => 1,
    });
    for (name, type_, value_type, input_param_kind) in sorted_inputs {
        input_ids.insert(
            name.clone(),
            graph.add_input_param(node_id, name, type_, value_type, input_param_kind, true),
        );
    }

    for (name, output) in template.0.output_name.iter().zip(template.0.output.iter()) {
        output_ids.push(graph.add_output_param(node_id, name.clone(), output.clone()));
    }

    BuildNodeOutput {
        input_ids,
        output_ids,
    }
}

/// The iterator for node templates.
pub struct NodeTemplates<'a>(pub &'a ObjectInfo);
impl<'a> NodeTemplateIter for NodeTemplates<'a> {
    type Item = FlowNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        self.0.values().cloned().map(FlowNodeTemplate).collect()
    }
}

/// The state of the node graph.
pub type FlowEditorState =
    GraphEditorState<FlowNodeData, ObjectType, FlowValueType, FlowNodeTemplate, FlowUserState>;
