use std::{
    collections::HashMap,
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
    thread::JoinHandle,
    time::{Instant, SystemTime},
};

use anyhow::Context;
use eframe::egui;

use rucomfyui::{object_info::ObjectInfo, workflow::WorkflowNodeId, Workflow};

fn main() {
    eframe::run_native(
        "rucomfyui-node-graph",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Application::new(cc)))
        }),
    )
    .unwrap();
}

/// The main application as an [`eframe::App`].
pub struct Application {
    /// The address of the ComfyUI server.
    comfyui_address: String,

    /// The current graph; available when connected to ComfyUI.
    graph: Option<rucomfyui_node_graph::ComfyUiNodeGraph>,

    /// The input channel to the [`tokio`] runtime thread.
    tokio_input_tx: Sender<TokioInputEvent>,
    /// The output channel from the [`tokio`] runtime thread.
    tokio_output_rx: Receiver<TokioOutputEvent>,
    /// The [`tokio`] runtime thread.
    _tokio_runtime_thread: JoinHandle<()>,
    /// A thread responsible for issuing repaint requests periodically
    /// to ensure that any UI changes are reflected, even when the user
    /// is not interacting with the UI.
    _pump_repaint_thread: JoinHandle<()>,

    /// The current error, if any. Will be displayed.
    error: Option<(String, String)>,

    /// The file dialog.
    file_dialog: egui_file_dialog::FileDialog,
    /// The current mode for the file dialog.
    file_mode: FileMode,

    /// The time at which the last queue was issued.
    last_queue_time: Option<Instant>,

    /// Whether the settings window is open.
    settings_open: bool,
    /// Whether to allow insecure HTTPS.
    allow_insecure_https: bool,
    /// The authorization token to pass (`Bearer {authorization_token}` in the `Authorization` header).
    authorization_token: String,
}
/// The current mode for the file dialog.
enum FileMode {
    None,
    Load,
    Save,
}
impl Application {
    /// Create a new application.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut file_dialog = egui_file_dialog::FileDialog::new();

        let (tokio_input_tx, tokio_input_rx) = std::sync::mpsc::channel();
        let (tokio_output_tx, tokio_output_rx) = std::sync::mpsc::channel();

        if let Some(storage) = cc.storage {
            *file_dialog.storage_mut() =
                eframe::get_value(storage, "file_dialog").unwrap_or_default();
        }

        Self {
            comfyui_address: cc
                .storage
                .and_then(|s| eframe::get_value(s, "comfyui_address"))
                .unwrap_or_else(|| "http://127.0.0.1:8188".to_string()),

            graph: None,

            tokio_input_tx,
            tokio_output_rx,
            _tokio_runtime_thread: std::thread::spawn(move || {
                tokio_runtime_thread(tokio_input_rx, tokio_output_tx)
            }),
            _pump_repaint_thread: std::thread::spawn({
                let ctx = cc.egui_ctx.clone();
                move || loop {
                    ctx.request_repaint();
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }),

            error: None,

            file_dialog,
            file_mode: FileMode::None,

            last_queue_time: None,

            settings_open: false,
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
        eframe::set_value(storage, "file_dialog", self.file_dialog.storage_mut());
        eframe::set_value(storage, "comfyui_address", &self.comfyui_address);
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
                egui::widgets::global_dark_light_mode_switch(ui);
                if is_connected {
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
                    ui.label("Allow insecure HTTPS:");
                    ui.checkbox(&mut self.allow_insecure_https, "");
                    ui.label("Authorization token:");
                    ui.text_edit_singleline(&mut self.authorization_token);
                });
        }

        self.file_dialog.update(ctx);
    }
}
impl Application {
    /// Load the given file into the current graph.
    fn api_load(&mut self, path: PathBuf) -> anyhow::Result<()> {
        let workflow = rucomfyui::Workflow::from_json(
            &std::fs::read_to_string(path).context("Failed to read file")?,
        )
        .context("Failed to parse workflow")?;
        self.graph
            .as_mut()
            .context("No graph to load into")?
            .load_api_workflow(&workflow)
            .context("Failed to load workflow")?;

        Ok(())
    }

    /// Save the current graph to the given path.
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
    fn process_incoming_events(&mut self) -> bool {
        let mut needs_repaint = false;

        // Process async events
        for event in self.tokio_output_rx.try_iter() {
            match event {
                TokioOutputEvent::ObjectInfo(oi) => {
                    self.graph = Some(rucomfyui_node_graph::ComfyUiNodeGraph::new(oi));
                }
                TokioOutputEvent::QueueWorkflowResult((mapping, result)) => {
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
                                .filter(|(_, images)| !images.is_empty())
                                .flat_map(|(workflow_node_id, images)| {
                                    reverse_mapping
                                        .get(&workflow_node_id)
                                        .copied()
                                        .map(|id| (id, images))
                                }),
                        );
                    }
                    self.last_queue_time = None;
                }
                TokioOutputEvent::Error(err) => {
                    let err = match err {
                        TokioOutputError::Connection(err) => ("Connection".to_string(), err),
                        TokioOutputError::Queue(err) => {
                            self.last_queue_time = None;
                            ("Queue".to_string(), err)
                        }
                    };
                    self.error = Some(err);
                }
            }
            needs_repaint = true;
        }

        // Process file dialog
        if let Some(path) = self.file_dialog.take_selected() {
            match self.file_mode {
                FileMode::None => {
                    eprintln!("File mode is None, this should not happen. Path: {path:?}");
                }
                FileMode::Load => {
                    if let Err(err) = self.api_load(path) {
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
        self.tokio_input_tx
            .send(TokioInputEvent::Connect {
                address: self.comfyui_address.clone(),
                allow_insecure_https: self.allow_insecure_https,
                authorization_token: self.authorization_token.clone(),
            })
            .unwrap();
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

        self.tokio_input_tx
            .send(TokioInputEvent::QueueWorkflow((
                mapping,
                graph.into_workflow(),
            )))
            .unwrap();
        self.last_queue_time = Some(Instant::now());
        Ok(())
    }
}

/// Input to the `tokio` runtime thread.
enum TokioInputEvent {
    /// Connect to ComfyUI at the given address and obtain object info.
    Connect {
        address: String,
        allow_insecure_https: bool,
        authorization_token: String,
    },
    /// Queue the given workflow with a mapping that can be returned back to the
    /// caller to map the output images to the correct nodes.
    QueueWorkflow((rucomfyui_node_graph::NodeToWorkflowNodeMapping, Workflow)),
}
/// Output from the `tokio` runtime thread.
enum TokioOutputEvent {
    /// We have successfully connected and obtained object info.
    ObjectInfo(ObjectInfo),
    /// We have successfully executed the workflow and obtained output images.
    QueueWorkflowResult(
        (
            rucomfyui_node_graph::NodeToWorkflowNodeMapping,
            HashMap<WorkflowNodeId, Vec<rucomfyui::OwnedBytes>>,
        ),
    ),
    /// Some error occurred.
    Error(TokioOutputError),
}
/// Error that can occur when sending output from the `tokio` runtime thread.
enum TokioOutputError {
    /// Error connecting to ComfyUI.
    Connection(String),
    /// Error queuing the workflow.
    Queue(String),
}
fn tokio_runtime_thread(input: Receiver<TokioInputEvent>, output: Sender<TokioOutputEvent>) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut client = None;
    for event in input.iter() {
        match event {
            TokioInputEvent::Connect {
                address,
                allow_insecure_https,
                authorization_token,
            } => {
                let reqwest_client = {
                    let reqwest_client = reqwest::Client::builder();
                    let reqwest_client = if allow_insecure_https {
                        reqwest_client.danger_accept_invalid_certs(true)
                    } else {
                        reqwest_client
                    };
                    let reqwest_client = if !authorization_token.is_empty() {
                        reqwest_client.default_headers(
                            std::iter::once((
                                reqwest::header::AUTHORIZATION,
                                reqwest::header::HeaderValue::from_str(&format!(
                                    "Bearer {authorization_token}"
                                ))
                                .expect("Failed to create header value"),
                            ))
                            .collect(),
                        )
                    } else {
                        reqwest_client
                    };
                    reqwest_client
                        .build()
                        .expect("Failed to build reqwest client")
                };
                let temp_client = rucomfyui::Client::new_with_client(address, reqwest_client);

                let object_info = rt.block_on(async { temp_client.object_info().await });
                match object_info {
                    Ok(object_info) => {
                        output
                            .send(TokioOutputEvent::ObjectInfo(object_info))
                            .unwrap();
                        client = Some(temp_client);
                    }
                    Err(err) => {
                        output
                            .send(TokioOutputEvent::Error(TokioOutputError::Connection(
                                err.to_string(),
                            )))
                            .unwrap();
                    }
                }
            }
            TokioInputEvent::QueueWorkflow((mapping, workflow)) => {
                let Some(client) = &mut client else {
                    output
                        .send(TokioOutputEvent::Error(TokioOutputError::Connection(
                            "Not connected to ComfyUI".to_string(),
                        )))
                        .unwrap();
                    continue;
                };

                let result = rt.block_on(async { client.easy_queue(&workflow).await });
                match result {
                    Ok(result) => {
                        output
                            .send(TokioOutputEvent::QueueWorkflowResult((mapping, result)))
                            .unwrap();
                    }
                    Err(err) => {
                        output
                            .send(TokioOutputEvent::Error(TokioOutputError::Queue(
                                err.to_string(),
                            )))
                            .unwrap();
                    }
                }
            }
        }
    }
}
