//! Internal implementation details of this crate, including the types used
//! to represent the graph and its nodes in terms of [`egui-snarl`].
//!
//! Provided for serialization and deserialization purposes.

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use egui_snarl::{
    ui::{PinInfo, SnarlViewer},
    InPin, NodeId, OutPin, Snarl,
};

use rucomfyui::{
    object_info::{
        Object, ObjectInfo, ObjectInputMetaTyped, ObjectInputMetaTypedRoundValue, ObjectInputType,
        ObjectType,
    },
    workflow::WorkflowInput,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Default, PartialEq)]
/// The value type for an input in the graph.
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

    pub(crate) fn from_object_type(
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

    pub(crate) fn from_object_type_and_input(
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

    /// Render the value widget for an input
    fn render_input_widget(
        &mut self,
        ui: &mut egui::Ui,
        param_name: &str,
        tooltip: Option<&String>,
    ) {
        let r = ui.label(param_name);
        if let Some(tooltip) = tooltip {
            r.on_hover_text(tooltip);
        }

        match self {
            FlowValueType::Array { options, selected } => {
                egui::ComboBox::new(format!("{param_name}_combobox"), "")
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
                ui.label("Unknown");
            }
        }
    }
}

/// Input data for a node
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct FlowInput {
    /// The name of the input
    pub name: String,
    /// The type of the input
    pub typ: ObjectType,
    /// The value of the input (if not connected)
    pub value: FlowValueType,
    /// Tooltip for the input
    pub tooltip: Option<String>,
}

/// Output data for a node
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct FlowOutput {
    /// The name of the output
    pub name: String,
    /// The type of the output
    pub typ: ObjectType,
    /// Tooltip for the output
    pub tooltip: Option<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
/// The data for a node in the graph.
pub struct FlowNodeData {
    /// The Object template for the node.
    pub object: Object,
    /// The inputs for the node
    pub inputs: Vec<FlowInput>,
    /// The outputs for the node
    pub outputs: Vec<FlowOutput>,
}

impl FlowNodeData {
    /// Create a new node from an Object template
    pub fn new(object: Object) -> Self {
        let mut inputs = Vec::new();

        // Collect all inputs with their metadata
        for (name, input, _required) in object.all_inputs() {
            let meta_typed = input.as_meta_typed();
            let tooltip = input.tooltip().map(|s| s.to_owned());

            let (typ, value) = match input.as_input_type() {
                ObjectInputType::Array(vec) => (
                    ObjectType::String,
                    FlowValueType::Array {
                        options: vec.iter().map(|v| v.as_str().to_string()).collect(),
                        selected: vec.first().cloned().map(String::from).unwrap_or_default(),
                    },
                ),
                ObjectInputType::Typed(object_type) => (
                    object_type.clone(),
                    FlowValueType::from_object_type(object_type, meta_typed),
                ),
            };

            inputs.push(FlowInput {
                name: name.to_string(),
                typ,
                value,
                tooltip,
            });
        }

        // Sort inputs: connection-only first, then others
        inputs.sort_by_key(|input| {
            if input.value.is_connection_only() {
                0
            } else {
                1
            }
        });

        let outputs = object
            .output_name
            .iter()
            .zip(object.output.iter())
            .enumerate()
            .map(|(idx, (name, typ))| FlowOutput {
                name: name.clone(),
                typ: typ.clone(),
                tooltip: object
                    .processed_output()
                    .nth(idx)
                    .and_then(|o| o.tooltip.as_ref().map(|s| s.to_string())),
            })
            .collect();

        Self {
            object,
            inputs,
            outputs,
        }
    }
}

/// User state for the graph viewer
#[derive(Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlowUserState {
    #[cfg_attr(feature = "serde", serde(skip))]
    /// A mapping from node IDs to output images and the selected image.
    pub output_images: HashMap<NodeId, (Vec<egui::ImageSource<'static>>, usize)>,
    #[cfg_attr(feature = "serde", serde(skip))]
    /// The search string for the node finder menu.
    pub node_finder_search: String,
}

/// The viewer for the ComfyUI flow graph
pub struct FlowViewer<'a> {
    /// The user state
    pub user_state: &'a mut FlowUserState,
    /// The object info containing available node types
    pub object_info: &'a ObjectInfo,
}

impl SnarlViewer<FlowNodeData> for FlowViewer<'_> {
    fn title(&mut self, node: &FlowNodeData) -> String {
        node.object.display_name().to_string()
    }

    fn inputs(&mut self, node: &FlowNodeData) -> usize {
        node.inputs.len()
    }

    fn outputs(&mut self, node: &FlowNodeData) -> usize {
        node.outputs.len()
    }

    #[allow(refining_impl_trait)]
    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut egui::Ui,
        snarl: &mut Snarl<FlowNodeData>,
    ) -> PinInfo {
        let node = &mut snarl[pin.id.node];
        let input = &mut node.inputs[pin.id.input];

        // If there's a connection, show the connected value (read-only)
        if let Some(&_remote) = pin.remotes.first() {
            // Connected - show label only
            let r = ui.label(&input.name);
            if let Some(tooltip) = &input.tooltip {
                r.on_hover_text(tooltip);
            }
        } else if input.value.is_connection_only() {
            // Connection-only input with no connection - show label
            let r = ui.label(&input.name);
            if let Some(tooltip) = &input.tooltip {
                r.on_hover_text(tooltip);
            }
        } else {
            // No connection and not connection-only - show editable widget
            input
                .value
                .render_input_widget(ui, &input.name, input.tooltip.as_ref());
        }

        // Determine pin color based on type
        let color = data_type_color(&input.typ);
        PinInfo::circle().with_fill(color)
    }

    #[allow(refining_impl_trait)]
    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut egui::Ui,
        snarl: &mut Snarl<FlowNodeData>,
    ) -> PinInfo {
        let node = &snarl[pin.id.node];
        let output = &node.outputs[pin.id.output];

        let r = ui.label(&output.name);
        if let Some(tooltip) = &output.tooltip {
            r.on_hover_text(tooltip);
        }

        let color = data_type_color(&output.typ);
        PinInfo::circle().with_fill(color)
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<FlowNodeData>) {
        // Validate that the types are compatible
        let from_node = &snarl[from.id.node];
        let to_node = &snarl[to.id.node];

        let from_output = &from_node.outputs[from.id.output];
        let to_input = &to_node.inputs[to.id.input];

        // Check type compatibility
        // In ComfyUI, any type can connect to any type (it's dynamically typed)
        // but we'll at least check if they're the same or if one is a wildcard
        let compatible = from_output.typ == to_input.typ
            || matches!(from_output.typ, ObjectType::Wildcard)
            || matches!(to_input.typ, ObjectType::Wildcard);

        if !compatible {
            // Types don't match - still allow but could add validation
        }

        // Disconnect any existing connections to this input (single connection per input)
        for &remote in &to.remotes {
            snarl.disconnect(remote, to.id);
        }

        // Make the connection
        snarl.connect(from.id, to.id);
    }

    fn has_body(&mut self, _node: &FlowNodeData) -> bool {
        false
    }

    fn has_footer(&mut self, _node: &FlowNodeData) -> bool {
        // Always return true since any node could potentially have output images.
        // The actual rendering in show_footer will handle cases where there are no images.
        true
    }

    fn show_footer(
        &mut self,
        node_id: NodeId,
        _inputs: &[InPin],
        _outputs: &[OutPin],
        ui: &mut egui::Ui,
        _snarl: &mut Snarl<FlowNodeData>,
    ) {
        if let Some((images, selected)) = self.user_state.output_images.get_mut(&node_id) {
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

            if let Some(image_source) = images.get(*selected) {
                let image = egui::Image::new(image_source.clone());

                // Try to get the natural image size for the default resize dimensions
                let default_size = image
                    .load_for_size(ui.ctx(), egui::Vec2::splat(f32::INFINITY))
                    .ok()
                    .and_then(|poll| poll.size())
                    .unwrap_or(egui::Vec2::new(256.0, 256.0));

                egui::Resize::default()
                    .id_salt(format!("{node_id:?}_image_resize"))
                    .default_size(default_size)
                    .show(ui, |ui| {
                        ui.add(image.shrink_to_fit());
                    });
            }
        }
    }

    fn has_graph_menu(&mut self, _pos: egui::Pos2, _snarl: &mut Snarl<FlowNodeData>) -> bool {
        true
    }

    fn show_graph_menu(
        &mut self,
        pos: egui::Pos2,
        ui: &mut egui::Ui,
        snarl: &mut Snarl<FlowNodeData>,
    ) {
        ui.label("Add Node");
        ui.separator();

        // Search box
        let response = ui.text_edit_singleline(&mut self.user_state.node_finder_search);
        if response.changed() {
            response.request_focus();
        }

        let search = self.user_state.node_finder_search.to_lowercase();

        // Collect and sort nodes by category
        let mut nodes_by_category: HashMap<&str, Vec<&Object>> = HashMap::new();
        for object in self.object_info.values() {
            // Filter by search string
            if !search.is_empty() {
                let name_matches = object.display_name().to_lowercase().contains(&search);
                let category_matches = object.category.to_lowercase().contains(&search);
                if !name_matches && !category_matches {
                    continue;
                }
            }
            nodes_by_category
                .entry(&object.category)
                .or_default()
                .push(object);
        }

        // Sort categories
        let mut categories: Vec<_> = nodes_by_category.keys().copied().collect();
        categories.sort();

        // Show nodes in a scrollable area
        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for category in categories {
                    let nodes = nodes_by_category.get(category).unwrap();
                    ui.collapsing(category, |ui| {
                        let mut sorted_nodes = nodes.clone();
                        sorted_nodes.sort_by_key(|n| n.display_name());
                        for object in sorted_nodes {
                            if ui.button(object.display_name()).clicked() {
                                let node_data = FlowNodeData::new(object.clone());
                                snarl.insert_node(pos, node_data);
                                self.user_state.node_finder_search.clear();
                                ui.close();
                            }
                        }
                    });
                }
            });
    }
}

/// Get the color for a data type
fn data_type_color(typ: &ObjectType) -> egui::Color32 {
    let mut hasher = std::hash::DefaultHasher::new();
    format!("{typ:?}").hash(&mut hasher);
    let hash = hasher.finish();
    let hash = (hash % 3600) as f32 / 3600.0;
    egui::ecolor::Hsva::new(hash, 0.5, 0.5, 1.0).into()
}
