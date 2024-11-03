#![deny(missing_docs)]
//! A demo application for [`rucomfyui_node_graph`] that allows you to load and queue workflows to a ComfyUI server.

use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
};
#[cfg(not(target_arch = "wasm32"))]
use std::{path::PathBuf, thread::JoinHandle};
use web_time::{Instant, SystemTime};

use anyhow::Context;
use eframe::egui;

use rucomfyui::{object_info::ObjectInfo, workflow::WorkflowNodeId, Workflow};

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eframe::run_native(
        "rucomfyui-node-graph",
        eframe::NativeOptions::default(),
        Box::new(make_app),
    )
    .unwrap();
}
#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(canvas, web_options, Box::new(make_app))
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
fn make_app(
    cc: &eframe::CreationContext<'_>,
) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
    cc.egui_ctx.set_visuals(egui::Visuals::dark());
    egui_extras::install_image_loaders(&cc.egui_ctx);
    Ok(Box::new(Application::new(cc)))
}

/// The main application as an [`eframe::App`].
pub struct Application {
    /// The address of the ComfyUI server.
    comfyui_address: String,

    /// The current graph; available when connected to ComfyUI.
    graph: Option<rucomfyui_node_graph::ComfyUiNodeGraph>,

    /// Responses to pass back to the main thread.
    async_output_tx: Sender<AsyncResponse>,
    /// Responses from the async handler.
    async_output_rx: Receiver<AsyncResponse>,
    /// The async runtime.
    runtime: AsyncRuntime,
    /// The [`rucomfyui`] client.
    client: Option<Arc<rucomfyui::Client>>,
    /// A thread responsible for issuing repaint requests periodically
    /// to ensure that any UI changes are reflected, even when the user
    /// is not interacting with the UI.
    #[cfg(not(target_arch = "wasm32"))]
    _pump_repaint_thread: JoinHandle<()>,

    /// The current error, if any. Will be displayed.
    error: Option<(String, String)>,

    /// The file dialog.
    #[cfg(not(target_arch = "wasm32"))]
    file_dialog: egui_file_dialog::FileDialog,
    /// The current mode for the file dialog.
    #[cfg(not(target_arch = "wasm32"))]
    file_mode: FileMode,

    /// The time at which the last queue was issued.
    last_queue_time: Option<Instant>,

    /// Whether the settings window is open.
    settings_open: bool,
    /// Whether to allow insecure HTTPS.
    #[cfg(not(target_arch = "wasm32"))]
    allow_insecure_https: bool,
    /// The authorization token to pass (`Bearer {authorization_token}` in the `Authorization` header).
    authorization_token: String,
}
/// The current mode for the file dialog.
#[cfg(not(target_arch = "wasm32"))]
enum FileMode {
    None,
    Load,
    Save,
}
impl Application {
    /// Create a new application.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let mut file_dialog = egui_file_dialog::FileDialog::new();

        #[cfg(not(target_arch = "wasm32"))]
        if let Some(storage) = cc.storage {
            *file_dialog.storage_mut() =
                eframe::get_value(storage, "file_dialog").unwrap_or_default();
        }

        let (async_output_tx, async_output_rx) = std::sync::mpsc::channel();
        Self {
            comfyui_address: cc
                .storage
                .and_then(|s| eframe::get_value(s, "comfyui_address"))
                .unwrap_or_else(|| "http://127.0.0.1:8188".to_string()),

            graph: None,

            async_output_tx,
            async_output_rx,
            runtime: AsyncRuntime::new(),
            client: None,
            #[cfg(not(target_arch = "wasm32"))]
            _pump_repaint_thread: std::thread::spawn({
                let ctx = cc.egui_ctx.clone();
                move || loop {
                    ctx.request_repaint();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }),

            error: None,

            #[cfg(not(target_arch = "wasm32"))]
            file_dialog,
            #[cfg(not(target_arch = "wasm32"))]
            file_mode: FileMode::None,

            last_queue_time: None,

            settings_open: false,
            #[cfg(not(target_arch = "wasm32"))]
            allow_insecure_https: cc
                .storage
                .and_then(|s| eframe::get_value(s, "allow_insecure_https"))
                .unwrap_or_default(),
            authorization_token: cc
                .storage
                .and_then(|s| eframe::get_value(s, "authorization_token"))
                .unwrap_or_default(),
        }
    }
}
impl eframe::App for Application {
    /// Save the application state to the given storage.
    ///
    /// Note that the graph state is not serialized: it cannot be easily stored within
    /// the [`eframe`] paradigm as the object info it depends on will not be available
    /// at deserialization time.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        #[cfg(not(target_arch = "wasm32"))]
        eframe::set_value(storage, "file_dialog", self.file_dialog.storage_mut());
        eframe::set_value(storage, "comfyui_address", &self.comfyui_address);
        #[cfg(not(target_arch = "wasm32"))]
        eframe::set_value(storage, "allow_insecure_https", &self.allow_insecure_https);
        eframe::set_value(storage, "authorization_token", &self.authorization_token);
    }

    /// Update the application and render the UI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.process_incoming_events() {
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            let is_connected = self.graph.is_some();
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                if is_connected {
                    #[cfg(not(target_arch = "wasm32"))]
                    ui.menu_button("File", |ui| {
                        if ui.button("Open API workflow").clicked() {
                            self.file_mode = FileMode::Load;
                            self.file_dialog.select_file();
                        }

                        if ui.button("Save API workflow").clicked() {
                            self.file_mode = FileMode::Save;
                            self.file_dialog.save_file();
                        }
                    });

                    if let Some(last_queue_time) = self.last_queue_time {
                        ui.label(format!(
                            "Queued: {:.02}s ago",
                            last_queue_time.elapsed().as_secs_f32()
                        ));
                    } else if ui.button("Queue").clicked() {
                        if let Err(err) = self.queue() {
                            self.error = Some(("Queue".to_string(), err.to_string()));
                        }
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add(egui::Button::new(if !is_connected {
                            "Connect"
                        } else {
                            "Disconnect"
                        }))
                        .clicked()
                    {
                        if !is_connected {
                            self.connect();
                        } else {
                            self.disconnect();
                        }
                    }
                    if ui.button("⛭").clicked() {
                        self.settings_open = !self.settings_open;
                    }
                    ui.text_edit_singleline(&mut self.comfyui_address);
                    ui.label("ComfyUI address:");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(graph) = self.graph.as_mut() {
                graph.show(ui);
            } else {
                ui.label(
                    egui::RichText::new("Not connected to ComfyUI")
                        .text_style(egui::TextStyle::Heading),
                );
                Default::default()
            }
        });

        if let Some((category, error)) = self.error.clone() {
            egui::Window::new("Error").show(ctx, |ui| {
                ui.label(egui::RichText::new(category).strong());
                ui.label(error);
                if ui.button("Close").clicked() {
                    self.error = None;
                }
            });
        }

        if self.settings_open {
            egui::Window::new("Settings")
                .open(&mut self.settings_open)
                .show(ctx, |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        ui.label("Allow insecure HTTPS:");
                        ui.checkbox(&mut self.allow_insecure_https, "");
                    }
                    ui.label("Authorization token:");
                    ui.text_edit_singleline(&mut self.authorization_token);
                });
        }

        #[cfg(not(target_arch = "wasm32"))]
        self.file_dialog.update(ctx);
    }
}
impl Application {
    /// Load the given file into the current graph.
    #[cfg(not(target_arch = "wasm32"))]
    fn api_load_from_path(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let workflow = rucomfyui::Workflow::from_json(
            &std::fs::read_to_string(path).context("Failed to read file")?,
        )
        .context("Failed to parse workflow")?;
        self.api_load(workflow)
    }
    /// Load the given API workflow into the current graph.
    fn api_load(&mut self, workflow: rucomfyui::Workflow) -> anyhow::Result<()> {
        self.graph
            .as_mut()
            .context("No graph to load into")?
            .load_api_workflow(&workflow)
            .context("Failed to load workflow")?;
        Ok(())
    }

    /// Save the current graph to the given path.
    #[cfg(not(target_arch = "wasm32"))]
    fn api_save(&self, path: PathBuf) -> anyhow::Result<()> {
        std::fs::write(
            path,
            self.graph
                .as_ref()
                .context("No graph to save")?
                .save_api_workflow()
                .to_json()
                .context("Failed to serialize workflow")?,
        )?;
        Ok(())
    }

    /// Process incoming events and return whether a repaint is needed.
    #[allow(unused_mut)]
    fn process_incoming_events(&mut self) -> bool {
        let mut needs_repaint = self.handle_async_responses();

        // Process file dialog
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(path) = self.file_dialog.take_selected() {
            match self.file_mode {
                FileMode::None => {
                    eprintln!("File mode is None, this should not happen. Path: {path:?}");
                }
                FileMode::Load => {
                    if let Err(err) = self.api_load_from_path(path) {
                        self.error = Some(("File load".to_string(), err.to_string()));
                        needs_repaint = true;
                    }
                }
                FileMode::Save => {
                    if let Err(err) = self.api_save(path) {
                        self.error = Some(("File save".to_string(), err.to_string()));
                        needs_repaint = true;
                    }
                }
            }
            self.file_mode = FileMode::None;
        }

        needs_repaint
    }

    /// Connect to ComfyUI at the address specified in [`Self::comfyui_address`].
    fn connect(&mut self) {
        self.async_request(AsyncRequest::Connect {
            address: self.comfyui_address.clone(),
            #[cfg(not(target_arch = "wasm32"))]
            allow_insecure_https: self.allow_insecure_https,
            authorization_token: self.authorization_token.clone(),
        });
    }

    /// Disconnect from ComfyUI.
    fn disconnect(&mut self) {
        self.graph = None;
    }

    /// Queue the current workflow.
    fn queue(&mut self) -> anyhow::Result<()> {
        let (graph, mapping) = self
            .graph
            .as_ref()
            .context("No graph to queue from")?
            .as_workflow_graph_with_mapping();

        self.async_request(AsyncRequest::QueueWorkflow((
            mapping,
            graph.into_workflow(),
        )));
        self.last_queue_time = Some(Instant::now());
        Ok(())
    }
}

/// Input to the async handler.
pub enum AsyncRequest {
    /// Connect to ComfyUI at the given address and obtain object info.
    Connect {
        /// The address of the ComfyUI server.
        address: String,
        /// Whether to allow insecure HTTPS.
        #[cfg(not(target_arch = "wasm32"))]
        allow_insecure_https: bool,
        /// The authorization token to pass (`Bearer {authorization_token}` in the `Authorization` header).
        authorization_token: String,
    },
    /// Queue the given workflow with a mapping that can be returned back to the
    /// caller to map the output images to the correct nodes.
    QueueWorkflow((rucomfyui_node_graph::NodeToWorkflowNodeMapping, Workflow)),
}
/// Output from the async handler.
pub enum AsyncResponse {
    /// We have successfully connected and obtained object info.
    ObjectInfo(ObjectInfo, rucomfyui::Client),
    /// We have successfully executed the workflow and obtained output images.
    QueueWorkflowResult(
        (
            rucomfyui_node_graph::NodeToWorkflowNodeMapping,
            HashMap<WorkflowNodeId, rucomfyui::queue::EasyQueueNodeOutput>,
        ),
    ),
    /// Some error occurred.
    Error(AsyncOutputError),
}
/// Error that can occur when sending output from the async handler.
pub enum AsyncOutputError {
    /// Error connecting to ComfyUI.
    Connection(String),
    /// Error queuing the workflow.
    Queue(String),
}

impl Application {
    /// Send an async request, which will then be processed and responded to through the
    /// [`Self::async_output_tx`] channel.
    fn async_request(&self, event: AsyncRequest) {
        match event {
            AsyncRequest::Connect {
                address,
                #[cfg(not(target_arch = "wasm32"))]
                allow_insecure_https,
                authorization_token,
            } => {
                let reqwest_client = {
                    let mut reqwest_client = reqwest::Client::builder();
                    #[cfg(not(target_arch = "wasm32"))]
                    if allow_insecure_https {
                        reqwest_client = reqwest_client.danger_accept_invalid_certs(true);
                    }
                    if !authorization_token.is_empty() {
                        reqwest_client = reqwest_client.default_headers(
                            std::iter::once((
                                reqwest::header::AUTHORIZATION,
                                reqwest::header::HeaderValue::from_str(&format!(
                                    "Bearer {authorization_token}"
                                ))
                                .expect("Failed to create header value"),
                            ))
                            .collect(),
                        )
                    }
                    reqwest_client
                        .build()
                        .expect("Failed to build reqwest client")
                };
                let temp_client = rucomfyui::Client::new_with_client(address, reqwest_client);

                let tx = self.async_output_tx.clone();
                self.runtime.spawn(async move {
                    let object_info = temp_client.object_info().await;

                    match object_info {
                        Ok(object_info) => {
                            tx.send(AsyncResponse::ObjectInfo(object_info, temp_client))
                                .unwrap();
                        }
                        Err(err) => {
                            tx.send(AsyncResponse::Error(AsyncOutputError::Connection(
                                err.to_string(),
                            )))
                            .unwrap();
                        }
                    }
                });
            }
            AsyncRequest::QueueWorkflow((mapping, workflow)) => {
                let Some(client) = self.client.clone() else {
                    self.async_output_tx
                        .send(AsyncResponse::Error(AsyncOutputError::Connection(
                            "Not connected to ComfyUI".to_string(),
                        )))
                        .unwrap();
                    return;
                };

                let tx = self.async_output_tx.clone();
                self.runtime.spawn(async move {
                    let result = client.easy_queue(&workflow).await;
                    match result {
                        Ok(result) => {
                            tx.send(AsyncResponse::QueueWorkflowResult((mapping, result)))
                                .unwrap();
                        }
                        Err(err) => {
                            tx.send(AsyncResponse::Error(AsyncOutputError::Queue(
                                err.to_string(),
                            )))
                            .unwrap();
                        }
                    }
                });
            }
        }
    }
    fn handle_async_responses(&mut self) -> bool {
        let mut needs_repaint = false;
        let mut load_default_workflow = false;

        // Process async events
        for event in self.async_output_rx.try_iter() {
            match event {
                AsyncResponse::ObjectInfo(oi, client) => {
                    self.graph = Some(rucomfyui_node_graph::ComfyUiNodeGraph::new(oi));
                    self.client = Some(Arc::new(client));
                    load_default_workflow = true;
                }
                AsyncResponse::QueueWorkflowResult((mapping, result)) => {
                    if let Some(graph) = self.graph.as_mut() {
                        let now_timestamp = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .map(|t| t.as_secs_f64())
                            .unwrap_or_default();
                        let reverse_mapping = mapping
                            .into_iter()
                            .map(|(k, v)| (v, k))
                            .collect::<HashMap<_, _>>();

                        graph.populate_output_images(
                            &now_timestamp.to_string(),
                            result
                                .into_iter()
                                .filter(|(_, output)| !output.images.is_empty())
                                .flat_map(|(workflow_node_id, output)| {
                                    reverse_mapping
                                        .get(&workflow_node_id)
                                        .copied()
                                        .map(|id| (id, output.images))
                                }),
                        );
                    }
                    self.last_queue_time = None;
                }
                AsyncResponse::Error(err) => {
                    let err = match err {
                        AsyncOutputError::Connection(err) => ("Connection".to_string(), err),
                        AsyncOutputError::Queue(err) => {
                            self.last_queue_time = None;
                            ("Queue".to_string(), err)
                        }
                    };
                    self.error = Some(err);
                }
            }
            needs_repaint = true;
        }

        if load_default_workflow {
            self.api_load(
                rucomfyui::Workflow::from_json(include_str!(
                    "../../../crates/rucomfyui/examples/existing_workflow.json"
                ))
                .unwrap(),
            )
            .unwrap();
        }

        needs_repaint
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    /// A runtime for async tasks.
    pub struct AsyncRuntime {
        /// The [`tokio`] runtime.
        runtime: tokio::runtime::Runtime,
    }
    impl AsyncRuntime {
        /// Create a new async runtime.
        pub fn new() -> Self {
            Self {
                runtime: tokio::runtime::Runtime::new().unwrap(),
            }
        }
        /// Spawn the given future on the runtime.
        pub fn spawn<F: std::future::Future<Output = ()> + Send + 'static>(&self, future: F) {
            self.runtime.spawn(future);
        }
    }
}
#[cfg(not(target_arch = "wasm32"))]
use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    /// A runtime for async tasks.
    pub struct AsyncRuntime;
    impl AsyncRuntime {
        /// Create a new async runtime.
        pub fn new() -> Self {
            Self
        }
        /// Spawn the given future on the runtime.
        pub fn spawn<F: std::future::Future<Output = ()> + 'static>(&self, future: F) {
            wasm_bindgen_futures::spawn_local(future);
        }
    }
}
#[cfg(target_arch = "wasm32")]
use wasm::*;
