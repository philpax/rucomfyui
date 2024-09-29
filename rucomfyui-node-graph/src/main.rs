use std::{
    borrow::Cow,
    collections::HashMap,
    hash::{Hash, Hasher},
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};

use anyhow::Context;
use eframe::egui::{self, DragValue, TextStyle, Visuals};
use egui_node_graph2::*;
use serde::{Deserialize, Serialize};

use rucomfyui::{
    object_info::{
        Object, ObjectInfo, ObjectInputMetaTyped, ObjectInputMetaTypedRoundValue, ObjectInputType,
        ObjectType,
    },
    workflow::{WorkflowInput, WorkflowNode, WorkflowNodeId},
};

fn main() {
    eframe::run_native(
        "rucomfyui-node-graph",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Ok(Box::new(Application::new(cc)))
        }),
    )
    .unwrap();
}

pub struct Application {
    persisted: PersistedState,
    user_state: FlowUserState,

    object_info: Option<ObjectInfo>,

    tokio_input_tx: Sender<TokioInputEvent>,
    tokio_output_rx: Receiver<TokioOutputEvent>,
    _tokio_runtime_thread: JoinHandle<()>,

    error: Option<String>,

    file_dialog: egui_file_dialog::FileDialog,
}
impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut file_dialog = egui_file_dialog::FileDialog::new();
        let persisted = PersistedState::load(cc, &mut file_dialog);

        let (tokio_input_tx, tokio_input_rx) = std::sync::mpsc::channel();
        let (tokio_output_tx, tokio_output_rx) = std::sync::mpsc::channel();

        Self {
            persisted,
            user_state: FlowUserState::default(),

            object_info: None,

            tokio_input_tx,
            tokio_output_rx,
            _tokio_runtime_thread: std::thread::spawn(move || {
                tokio_runtime_thread(tokio_input_rx, tokio_output_tx)
            }),

            error: None,

            file_dialog,
        }
    }

    fn load(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let object_info = self.object_info.as_ref().context("No object info")?;
        let workflow = rucomfyui::Workflow::from_json(&std::fs::read_to_string(path)?)?;
        let sorted_node_ids = workflow.topological_sort_with_depth();

        let mut mapping = HashMap::<WorkflowNodeId, BuildNodeOutput>::new();
        let mut node_position = egui::Pos2::ZERO;

        let state = &mut self.persisted.state;
        for node_ids in sorted_node_ids {
            node_position.x += 300.0;
            node_position.y = 0.0;

            for node_id in node_ids {
                let node = workflow.0.get(&node_id).unwrap();
                let object = object_info.get(&node.class_type).with_context(|| {
                    format!(
                        "Node {} has unknown class type {}",
                        node_id.0, node.class_type
                    )
                })?;
                let template = FlowNodeTemplate(object.clone());
                let g_node_id = state.graph.add_node(
                    object.display_name.clone(),
                    FlowNodeData {
                        template: template.clone(),
                    },
                    |g, g_node_id| {
                        let bno = build_node(&template, g, g_node_id, Some(node));
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
}
impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.persisted.save(storage, &mut self.file_dialog);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for event in self.tokio_output_rx.try_iter() {
            match event {
                TokioOutputEvent::ObjectInfo(oi) => self.object_info = Some(oi),
                TokioOutputEvent::Error(err) => {
                    self.error = Some(err);
                }
            }
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            let is_connected = self.object_info.is_some();
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);

                ui.label("ComfyUI address:");
                ui.text_edit_singleline(&mut self.persisted.comfyui_address);
                if ui
                    .add(egui::Button::new(if !is_connected {
                        "Connect"
                    } else {
                        "Disconnect"
                    }))
                    .clicked()
                {
                    if !is_connected {
                        self.tokio_input_tx
                            .send(TokioInputEvent::Connect(
                                self.persisted.comfyui_address.clone(),
                            ))
                            .unwrap();
                    } else {
                        self.object_info = None;
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if is_connected {
                        if ui.button("Open API workflow").clicked() {
                            self.file_dialog.select_file();
                        }
                    }
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(object_info) = &self.object_info {
                self.persisted.state.draw_graph_editor(
                    ui,
                    AllMyNodeTemplates(object_info),
                    &mut self.user_state,
                    Vec::default(),
                )
            } else {
                ui.label(
                    egui::RichText::new("Not connected to ComfyUI").text_style(TextStyle::Heading),
                );
                Default::default()
            }
        });

        if let Some(error) = self.error.clone() {
            egui::Window::new("Error").show(ctx, |ui| {
                ui.label(error);
                if ui.button("Close").clicked() {
                    self.error = None;
                }
            });
        }

        self.file_dialog.update(ctx);
        if let Some(path) = self.file_dialog.take_selected() {
            if let Err(err) = self.load(path) {
                self.error = Some(err.to_string());
            }
        }
    }
}

enum TokioInputEvent {
    Connect(String),
}

enum TokioOutputEvent {
    ObjectInfo(ObjectInfo),
    Error(String),
}

fn tokio_runtime_thread(input: Receiver<TokioInputEvent>, output: Sender<TokioOutputEvent>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut _client = None;
    for event in input.iter() {
        match event {
            TokioInputEvent::Connect(address) => {
                let temp_client = rucomfyui::Client::new(address);
                let object_info = rt.block_on(async { temp_client.object_info().await });
                match object_info {
                    Ok(object_info) => {
                        output
                            .send(TokioOutputEvent::ObjectInfo(object_info))
                            .unwrap();
                        _client = Some(temp_client);
                    }
                    Err(err) => {
                        output
                            .send(TokioOutputEvent::Error(err.to_string()))
                            .unwrap();
                    }
                }
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FlowNodeData {
    template: FlowNodeTemplate,
}
impl NodeDataTrait for FlowNodeData {
    type Response = MyResponse;
    type UserState = FlowUserState;
    type DataType = ObjectType;
    type ValueType = FlowValueType;

    fn bottom_ui(
        &self,
        _ui: &mut egui::Ui,
        _node_id: NodeId,
        _graph: &Graph<FlowNodeData, ObjectType, FlowValueType>,
        _user_state: &mut Self::UserState,
    ) -> Vec<NodeResponse<MyResponse, FlowNodeData>>
    where
        MyResponse: UserResponseTrait,
    {
        vec![]
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub enum FlowValueType {
    Array {
        options: Vec<String>,
        selected: String,
    },
    String {
        value: String,
        multiline: bool,
    },
    Float {
        value: f64,
        default: f64,
        min: f64,
        max: f64,
        round: Option<f64>,
        step: f64,
    },
    SignedInt(i64),
    UnsignedInt(u64),
    Boolean(bool),
    Other(ObjectType),
    #[default]
    Unknown,
}
impl FlowValueType {
    fn convert_float(value: Option<f64>, typed_meta: Option<&ObjectInputMetaTyped>) -> Self {
        let typed_meta = typed_meta.and_then(|m| m.as_number());
        Self::Float {
            value: typed_meta
                .map(|m| f64::from(m.default))
                .or(value)
                .unwrap_or(0.0),
            default: typed_meta.map(|m| m.default.into()).unwrap_or(0.0),
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

    fn from_object_type(
        object_type: &ObjectType,
        typed_meta: Option<&ObjectInputMetaTyped>,
    ) -> Self {
        match object_type {
            ObjectType::Boolean => Self::Boolean(false),
            ObjectType::Float => Self::convert_float(None, typed_meta),
            ObjectType::Int => Self::SignedInt(0),
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
            WorkflowInput::I64(v) => Self::SignedInt(*v),
            WorkflowInput::U64(v) => Self::UnsignedInt(*v),
            WorkflowInput::Boolean(b) => Self::Boolean(*b),
            WorkflowInput::Slot(_, _) => Self::Other(object_type.clone()),
        }
    }
    #[must_use]
    pub fn is_connection_only(&self) -> bool {
        matches!(self, Self::Other(..)) || matches!(self, Self::Unknown)
    }
    #[must_use]
    pub fn is_constant_only(&self) -> bool {
        matches!(self, Self::Array { .. })
    }
}
impl WidgetValueTrait for FlowValueType {
    type Response = MyResponse;
    type UserState = FlowUserState;
    type NodeData = FlowNodeData;
    fn value_widget(
        &mut self,
        param_name: &str,
        _node_id: NodeId,
        ui: &mut egui::Ui,
        _user_state: &mut FlowUserState,
        _node_data: &FlowNodeData,
    ) -> Vec<MyResponse> {
        ui.label(param_name);
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
                default: _,
                min,
                max,
                round,
                step,
            } => {
                ui.add(DragValue::new(value).range(*min..=*max).speed(*step));
                *value = round.map(|r| (*value / r).round() * r).unwrap_or(*value);
            }
            FlowValueType::SignedInt(v) => {
                ui.add(DragValue::new(v));
            }
            FlowValueType::UnsignedInt(v) => {
                ui.add(DragValue::new(v));
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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
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
        FlowNodeData {
            template: self.clone(),
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
pub enum MyResponse {}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct FlowUserState {}

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

struct BuildNodeOutput {
    input_ids: HashMap<String, InputId>,
    output_ids: Vec<OutputId>,
}
fn build_node(
    template: &FlowNodeTemplate,
    graph: &mut Graph<FlowNodeData, ObjectType, FlowValueType>,
    node_id: NodeId,
    workflow_node: Option<&WorkflowNode>,
) -> BuildNodeOutput {
    let mut input_ids = HashMap::new();
    let mut output_ids = vec![];

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

        input_ids.insert(
            name.to_string(),
            graph.add_input_param(
                node_id,
                name.to_string(),
                type_.clone(),
                value_type,
                input_param_kind,
                true,
            ),
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

pub struct AllMyNodeTemplates<'a>(&'a ObjectInfo);
impl<'a> NodeTemplateIter for AllMyNodeTemplates<'a> {
    type Item = FlowNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        self.0.values().cloned().map(FlowNodeTemplate).collect()
    }
}

impl UserResponseTrait for MyResponse {}

type FlowEditorState =
    GraphEditorState<FlowNodeData, ObjectType, FlowValueType, FlowNodeTemplate, FlowUserState>;

#[derive(Serialize, Deserialize)]
struct PersistedState {
    pub comfyui_address: String,
    pub state: FlowEditorState,
}
impl PersistedState {
    pub fn load(
        cc: &eframe::CreationContext<'_>,
        file_dialog: &mut egui_file_dialog::FileDialog,
    ) -> Self {
        let default = Self::default();
        if let Some(storage) = cc.storage {
            *file_dialog.storage_mut() =
                eframe::get_value(storage, "file_dialog").unwrap_or_default();
            Self {
                comfyui_address: eframe::get_value(storage, "comfyui_address")
                    .unwrap_or(default.comfyui_address),
                state: eframe::get_value(storage, "state").unwrap_or(default.state),
            }
        } else {
            default
        }
    }
    pub fn save(
        &self,
        storage: &mut dyn eframe::Storage,
        file_dialog: &mut egui_file_dialog::FileDialog,
    ) {
        eframe::set_value(storage, "comfyui_address", &self.comfyui_address);
        eframe::set_value(storage, "state", &self.state);
        eframe::set_value(storage, "file_dialog", file_dialog.storage_mut());
    }
}
impl Default for PersistedState {
    fn default() -> Self {
        Self {
            comfyui_address: "http://127.0.0.1:8188".to_string(),
            state: Default::default(),
        }
    }
}
