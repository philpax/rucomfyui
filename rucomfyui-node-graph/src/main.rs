use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
};

use eframe::egui::{self, DragValue, TextStyle, Visuals};
use egui_node_graph2::*;
use rucomfyui::object_info::{Object, ObjectInfo, ObjectInputType, ObjectType};
use serde::{Deserialize, Serialize};

const PERSISTENCE_KEY: &str = "rucomfyui-node-graph";

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
}
impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let persisted = cc
            .storage
            .and_then(|storage| eframe::get_value(storage, PERSISTENCE_KEY))
            .unwrap_or_default();

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
        }
    }
}
impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, PERSISTENCE_KEY, &self.persisted);
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
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
            });
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("ComfyUI address:");
                ui.text_edit_singleline(&mut self.persisted.comfyui_address);
                if ui
                    .add_enabled(
                        self.object_info.is_none(),
                        egui::Button::new(if self.object_info.is_none() {
                            "Connect"
                        } else {
                            "Connected"
                        }),
                    )
                    .clicked()
                {
                    self.tokio_input_tx
                        .send(TokioInputEvent::Connect(
                            self.persisted.comfyui_address.clone(),
                        ))
                        .unwrap();
                }
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub enum FlowValueType {
    Array {
        options: Vec<String>,
        selected: String,
    },
    String(String),
    Float(f32),
    Int(i32),
    Boolean(bool),
    Other(ObjectType),
    #[default]
    Unknown,
}
impl FlowValueType {
    #[must_use]
    pub fn is_connection_only(&self) -> bool {
        matches!(self, Self::Other(..)) || matches!(self, Self::Unknown)
    }
    #[must_use]
    pub fn is_constant_only(&self) -> bool {
        matches!(self, Self::Array { .. })
    }
}
impl<'a> From<&'a ObjectType> for FlowValueType {
    fn from(value: &'a ObjectType) -> Self {
        match value {
            ObjectType::Boolean => FlowValueType::Boolean(false),
            ObjectType::Float => FlowValueType::Float(0.0),
            ObjectType::Int => FlowValueType::Int(0),
            ObjectType::String => FlowValueType::String("".into()),
            _ => FlowValueType::Other(value.clone()),
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct FlowNodeTemplate(pub Object);

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
        for (name, input) in self
            .0
            .input
            .required
            .iter()
            .chain(self.0.input.optional.iter().flat_map(|v| v.iter()))
        {
            let (type_, value_type) = match input.as_input_type() {
                ObjectInputType::Array(vec) => (
                    ObjectType::String,
                    FlowValueType::Array {
                        options: vec.clone(),
                        selected: vec.first().cloned().unwrap_or_default(),
                    },
                ),
                ObjectInputType::Typed(object_type) => {
                    (object_type.clone(), FlowValueType::from(object_type))
                }
            };

            let input_param_kind = if value_type.is_connection_only() {
                InputParamKind::ConnectionOnly
            } else if value_type.is_constant_only() {
                InputParamKind::ConstantOnly
            } else {
                InputParamKind::ConnectionOrConstant
            };

            graph.add_input_param(
                node_id,
                name.clone(),
                type_.clone(),
                value_type,
                input_param_kind,
                true,
            );
        }

        for (name, output) in self.0.output_name.iter().zip(self.0.output.iter()) {
            graph.add_output_param(node_id, name.clone(), output.clone());
        }
    }
}

pub struct AllMyNodeTemplates<'a>(&'a ObjectInfo);
impl<'a> NodeTemplateIter for AllMyNodeTemplates<'a> {
    type Item = FlowNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        self.0.values().cloned().map(FlowNodeTemplate).collect()
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
            FlowValueType::String(string) => {
                ui.text_edit_singleline(string);
            }
            FlowValueType::Float(v) => {
                ui.add(DragValue::new(v));
            }
            FlowValueType::Int(v) => {
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

impl UserResponseTrait for MyResponse {}
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

type FlowEditorState =
    GraphEditorState<FlowNodeData, ObjectType, FlowValueType, FlowNodeTemplate, FlowUserState>;

#[derive(Serialize, Deserialize)]
struct PersistedState {
    pub comfyui_address: String,
    pub state: FlowEditorState,
}
impl Default for PersistedState {
    fn default() -> Self {
        Self {
            comfyui_address: "http://127.0.0.1:8188".to_string(),
            state: Default::default(),
        }
    }
}
