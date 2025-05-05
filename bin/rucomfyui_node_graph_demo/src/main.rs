//! A demo application for [`rucomfyui_node_graph`] that allows you to load and queue workflows to a ComfyUI server.
#![deny(missing_docs)]

use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc,
    },
};
use web_time::{Instant, SystemTime};

use anyhow::Context;
use eframe::egui;

use rucomfyui::{object_info::ObjectInfo, queue::QueueEntry, workflow::WorkflowNodeId};

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

    /// The current error, if any. Will be displayed.
    error: Option<(String, String)>,

    /// The file dialog for loading and saving workflows.
    file_dialog: rfd::AsyncFileDialog,

    /// The queue for this application.
    queue: rucomfyui::queue::Queue,
    /// The time at which the queue was last updated.
    last_queue_update_time: Instant,
    /// The workflow that is currently being viewed.
    viewed_workflow: Option<rucomfyui_node_graph::ComfyUiNodeGraph>,

    /// The time at which the last prompt was queued.
    last_prompt_queue_time: Option<Instant>,

    /// The system statistics.
    system_stats: Option<rucomfyui::system_stats::SystemStats>,
    /// Whether the system stats window is open.
    system_stats_open: bool,

    /// Whether the settings window is open.
    settings_open: bool,
    /// Whether to allow insecure HTTPS.
    #[cfg(not(target_arch = "wasm32"))]
    allow_insecure_https: bool,
    /// The authorization token to pass (`Bearer {authorization_token}` in the `Authorization` header).
    authorization_token: String,
}
impl Application {
    /// Create a new application.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (async_output_tx, async_output_rx) = std::sync::mpsc::channel();

        // Get comfyui_address from command line args if available
        let cli_address = std::env::args().nth(1);
        let comfyui_address = cli_address.clone().unwrap_or_else(|| {
            cc.storage
                .and_then(|s| eframe::get_value(s, "comfyui_address"))
                .unwrap_or_else(|| "http://127.0.0.1:8188".to_string())
        });

        let mut app = Self {
            comfyui_address,

            graph: None,

            async_output_tx,
            async_output_rx,
            runtime: AsyncRuntime::new(),
            client: None,

            error: None,

            file_dialog: rfd::AsyncFileDialog::new().set_file_name("workflow.json"),

            queue: rucomfyui::queue::Queue::default(),
            last_queue_update_time: Instant::now(),
            viewed_workflow: None,

            last_prompt_queue_time: None,

            system_stats: None,
            system_stats_open: false,

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
        };

        // If comfyui_address came from command line args, connect immediately
        if cli_address.is_some() {
            app.connect();
        }

        app
    }
}
impl eframe::App for Application {
    /// Save the application state to the given storage.
    ///
    /// Note that the graph state is not serialized: it cannot be easily stored within
    /// the [`eframe`] paradigm as the object info it depends on will not be available
    /// at deserialization time.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "comfyui_address", &self.comfyui_address);
        #[cfg(not(target_arch = "wasm32"))]
        eframe::set_value(storage, "allow_insecure_https", &self.allow_insecure_https);
        eframe::set_value(storage, "authorization_token", &self.authorization_token);
    }

    /// Update the application and render the UI.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(web_time::Duration::from_millis(100));

        if let Some(client) = self.client.as_ref() {
            if self.last_queue_update_time.elapsed().as_millis() > 100 {
                let tx = self.async_output_tx.clone();
                let client = client.clone();
                self.runtime.spawn(async move {
                    let queue = client.get_queue().await;
                    tx.send(match queue {
                        Ok(queue) => AsyncResponse::Queue(queue),
                        Err(err) => AsyncResponse::error("Queue", err),
                    })
                    .unwrap();
                });
                self.last_queue_update_time = Instant::now();
            }
        }

        if self.handle_async_responses() {
            ctx.request_repaint();
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            let is_connected = self.graph.is_some();
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                if is_connected {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open API workflow").clicked() {
                            self.request_api_load();
                        }
                        if ui.button("Save API workflow").clicked() {
                            self.request_api_save();
                        }
                    });
                    if ui.button("System stats").clicked() {
                        self.request_system_stats();
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

        if self.graph.is_some() {
            egui::SidePanel::right("right").show(ctx, |ui| {
                ui.heading("Queue");
                fn render_queue(
                    ui: &mut egui::Ui,
                    object_info: &ObjectInfo,
                    queue_name: &str,
                    queue: &[QueueEntry],
                    viewed_workflow: &mut Option<rucomfyui_node_graph::ComfyUiNodeGraph>,
                ) {
                    egui::CollapsingHeader::new(queue_name)
                        .default_open(true)
                        .show(ui, |ui| {
                            for entry in queue {
                                ui.group(|ui| {
                                    ui.label(
                                        egui::RichText::from(format!(
                                            "{}: {}",
                                            entry.number, entry.prompt_id
                                        ))
                                        .monospace(),
                                    );
                                    ui.horizontal(|ui| {
                                        if ui.button("View").clicked() {
                                            let mut graph =
                                                rucomfyui_node_graph::ComfyUiNodeGraph::new(
                                                    object_info.clone(),
                                                );
                                            graph
                                                .load_api_workflow(&entry.prompt)
                                                .expect("Failed to load queued workflow");
                                            *viewed_workflow = Some(graph);
                                        }
                                        if ui.button("Cancel").clicked() {
                                            todo!("Cancellation not implemented");
                                        }
                                    });
                                });
                            }
                        });
                }
                if let Some(last_queue_time) = self.last_prompt_queue_time {
                    ui.label(format!(
                        "Queued: {:.02}s ago",
                        last_queue_time.elapsed().as_secs_f32()
                    ));
                } else if ui
                    .add(egui::Button::new("Run").min_size(egui::vec2(100.0, 0.0)))
                    .clicked()
                {
                    self.request_queue();
                }
                let viewed_workflow = &mut self.viewed_workflow;
                if let Some(object_info) = self.graph.as_ref().map(|g| &g.object_info) {
                    render_queue(
                        ui,
                        object_info,
                        "Running",
                        &self.queue.running,
                        viewed_workflow,
                    );
                    render_queue(
                        ui,
                        object_info,
                        "Pending",
                        &self.queue.pending,
                        viewed_workflow,
                    );
                }
            });
        }

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
            let mut open = true;
            egui::Window::new("Error").open(&mut open).show(ctx, |ui| {
                ui.label(egui::RichText::new(category).strong());
                ui.label(error);
                if ui.button("Close").clicked() {
                    self.error = None;
                }
            });
            if !open {
                self.error = None;
            }
        }

        if let Some(workflow) = self.viewed_workflow.as_mut() {
            let mut open = true;
            egui::Window::new("Viewed workflow")
                .open(&mut open)
                .show(ctx, |ui| {
                    // TODO: fix ID conflict with main graph
                    workflow.show(ui);
                });
            if !open {
                self.viewed_workflow = None;
            }
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

        if self.system_stats_open {
            self.draw_system_stats(ctx);
        }
    }
}
impl Application {
    fn draw_system_stats(&mut self, ctx: &egui::Context) {
        fn draw_system_info(ui: &mut egui::Ui, system: &rucomfyui::system_stats::SystemInfo) {
            ui.heading("System Information");
            ui.separator();

            let ram_total_gb = system.ram_total as f64 / 1_073_741_824.0; // Convert to GB
            let ram_free_gb = system.ram_free as f64 / 1_073_741_824.0; // Convert to GB
            let ram_used_gb = ram_total_gb - ram_free_gb;
            let ram_usage_percent = if ram_total_gb > 0.0 {
                (ram_used_gb / ram_total_gb) * 100.0
            } else {
                0.0
            };

            egui::Grid::new("system_info_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("OS:");
                    ui.label(&system.os);
                    ui.end_row();

                    ui.label("ComfyUI Version:");
                    ui.label(&system.comfyui_version);
                    ui.end_row();

                    ui.label("Python Version:");
                    ui.label(&system.python_version);
                    ui.end_row();

                    ui.label("PyTorch Version:");
                    ui.label(&system.pytorch_version);
                    ui.end_row();

                    ui.label("Embedded Python:");
                    ui.label(if system.embedded_python { "Yes" } else { "No" });
                    ui.end_row();

                    ui.label("RAM Usage:");
                    ui.label(format!(
                        "{ram_used_gb:.1} GB / {ram_total_gb:.1} GB ({ram_usage_percent:.1}%)"
                    ));
                    ui.end_row();
                });
        }

        fn draw_device_info(
            ui: &mut egui::Ui,
            i: usize,
            device: &rucomfyui::system_stats::DeviceInfo,
        ) {
            let vram_total_gb = device.vram_total as f64 / 1_073_741_824.0;
            let vram_free_gb = device.vram_free as f64 / 1_073_741_824.0;
            let vram_used_gb = vram_total_gb - vram_free_gb;
            let vram_usage_percent = if vram_total_gb > 0.0 {
                (vram_used_gb / vram_total_gb) * 100.0
            } else {
                0.0
            };

            ui.collapsing(
                format!("{}: {} ({})", device.name, device.device_type, device.index),
                |ui| {
                    egui::Grid::new(format!("device_info_grid_{}", i))
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            // VRAM usage
                            ui.label("VRAM Usage:");
                            ui.label(format!(
                                "{:.1} GB / {:.1} GB ({:.1}%)",
                                vram_used_gb, vram_total_gb, vram_usage_percent
                            ));
                            ui.end_row();

                            // Torch VRAM info
                            let torch_vram_total_gb =
                                device.torch_vram_total as f64 / 1_073_741_824.0;
                            let torch_vram_free_gb =
                                device.torch_vram_free as f64 / 1_073_741_824.0;
                            let torch_vram_used_gb = torch_vram_total_gb - torch_vram_free_gb;
                            let torch_vram_percent = if torch_vram_total_gb > 0.0 {
                                (torch_vram_used_gb / torch_vram_total_gb) * 100.0
                            } else {
                                0.0
                            };

                            ui.label("Torch VRAM Usage:");
                            ui.label(format!(
                                "{:.1} GB / {:.1} GB ({:.1}%)",
                                torch_vram_used_gb, torch_vram_total_gb, torch_vram_percent
                            ));
                            ui.end_row();
                        });
                },
            );
        }

        egui::Window::new("System stats")
            .open(&mut self.system_stats_open)
            .show(ctx, |ui| {
                let Some(system_stats) = self.system_stats.as_ref() else {
                    ui.label("No system stats available");
                    return;
                };

                draw_system_info(ui, &system_stats.system);

                // Device Info Section
                if !system_stats.devices.is_empty() {
                    ui.add_space(16.0);
                    ui.heading("Devices");
                    ui.separator();

                    for (i, device) in system_stats.devices.iter().enumerate() {
                        if i > 0 {
                            ui.add_space(8.0);
                        }
                        draw_device_info(ui, i, device);
                    }
                }

                // Command line arguments section
                let argv = &system_stats.system.argv;
                if !argv.is_empty() {
                    ui.add_space(16.0);
                    ui.collapsing("Command Line Arguments", |ui| {
                        for (i, arg) in argv.iter().enumerate() {
                            ui.label(format!("[{}] {}", i, arg));
                        }
                    });
                }
            });
    }
}
impl Application {
    /// Load the given API workflow into the current graph.
    fn api_load(&mut self, workflow: rucomfyui::Workflow) -> anyhow::Result<()> {
        self.graph
            .as_mut()
            .context("No graph to load into")?
            .load_api_workflow(&workflow)
            .context("Failed to load workflow")?;
        Ok(())
    }
    /// Request that an API workflow be loaded from the file dialog.
    fn request_api_load(&mut self) {
        let file_dialog = self.file_dialog.clone();
        let tx = self.async_output_tx.clone();
        self.runtime.spawn(async move {
            let Some(handle) = file_dialog.pick_file().await else {
                return;
            };
            let handler = || async move {
                let data = handle.read().await;
                anyhow::Ok(
                    rucomfyui::Workflow::from_json(std::str::from_utf8(&data).with_context(
                        || format!("Failed to convert {} to UTF-8", handle.file_name()),
                    )?)
                    .context("Failed to parse workflow")?,
                )
            };
            tx.send(match handler().await {
                Ok(workflow) => AsyncResponse::LoadedWorkflow(workflow),
                Err(err) => AsyncResponse::error("Load", err),
            })
            .unwrap();
        });
    }
    /// Request that the current graph be saved as an API workflow to the file dialog.
    fn request_api_save(&mut self) {
        let Some(graph) = self.graph.as_ref() else {
            self.async_output_tx
                .send(AsyncResponse::error("Save", "No graph to save"))
                .unwrap();
            return;
        };

        let workflow = graph.save_api_workflow();
        let file_dialog = self.file_dialog.clone();
        let tx = self.async_output_tx.clone();
        self.runtime.spawn(async move {
            let Some(handle) = file_dialog.save_file().await else {
                return;
            };
            let handler = || async move {
                handle
                    .write(workflow.to_json()?.as_bytes())
                    .await
                    .context("Failed to write to file")
            };
            if let Err(err) = handler().await {
                tx.send(AsyncResponse::error("Save", err)).unwrap();
            }
        });
    }

    /// Connect to ComfyUI at the address specified in [`Self::comfyui_address`].
    fn connect(&mut self) {
        let mut builder = reqwest::Client::builder();
        #[cfg(not(target_arch = "wasm32"))]
        if self.allow_insecure_https {
            builder = builder.danger_accept_invalid_certs(true);
        }
        if !self.authorization_token.is_empty() {
            builder = builder.default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!(
                        "Bearer {}",
                        self.authorization_token
                    ))
                    .expect("Failed to create header value"),
                ))
                .collect(),
            )
        }
        let client = rucomfyui::Client::new_with_client(
            self.comfyui_address.clone(),
            builder.build().expect("Failed to build reqwest client"),
        );

        let tx = self.async_output_tx.clone();
        self.runtime.spawn(async move {
            let object_info = client.object_info().await;
            tx.send(match object_info {
                Ok(object_info) => AsyncResponse::ObjectInfo {
                    object_info,
                    client,
                },
                Err(err) => AsyncResponse::error("Connection", err),
            })
            .unwrap();
        });
    }

    /// Disconnect from ComfyUI.
    fn disconnect(&mut self) {
        self.graph = None;
    }

    /// Request that the current workflow is queued.
    fn request_queue(&mut self) {
        let tx = self.async_output_tx.clone();
        let Some(graph) = self.graph.as_ref() else {
            tx.send(AsyncResponse::error("Queue", "No graph to queue from"))
                .unwrap();
            return;
        };
        let Some(client) = self.client.clone() else {
            tx.send(AsyncResponse::error(
                "Connection",
                "Not connected to ComfyUI",
            ))
            .unwrap();
            return;
        };

        let (graph, mapping) = graph.as_workflow_graph_with_mapping();
        self.runtime.spawn(async move {
            let output = client.easy_queue(&graph.into_workflow()).await;
            tx.send(match output {
                Ok(output) => AsyncResponse::QueueWorkflowResult { mapping, output },
                Err(err) => AsyncResponse::error("Queue", err),
            })
            .unwrap();
        });
        self.last_prompt_queue_time = Some(Instant::now());
    }

    /// Request that system statistics be fetched.
    fn request_system_stats(&mut self) {
        let tx = self.async_output_tx.clone();
        let client = self.client.clone().unwrap();
        self.runtime.spawn(async move {
            let stats = client.system_stats().await;
            tx.send(match stats {
                Ok(stats) => AsyncResponse::SystemStats(stats),
                Err(err) => AsyncResponse::error("System stats", err),
            })
            .unwrap();
        });
    }
}

/// Output from the async handler.
pub enum AsyncResponse {
    /// We have successfully connected and obtained object info.
    ObjectInfo {
        /// The object info.
        object_info: ObjectInfo,
        /// The client.
        client: rucomfyui::Client,
    },
    /// We have successfully executed the workflow and obtained output images.
    QueueWorkflowResult {
        /// The mapping from graph nodes to workflow nodes.
        mapping: rucomfyui_node_graph::NodeToWorkflowNodeMapping,
        /// The output from the workflow.
        output: HashMap<WorkflowNodeId, rucomfyui::queue::EasyQueueNodeOutput>,
    },
    /// Some error occurred.
    Error {
        /// The category of the error.
        category: String,
        /// The error message.
        error: String,
    },
    /// The workflow was loaded from the file dialog.
    LoadedWorkflow(rucomfyui::Workflow),
    /// A queue was received.
    Queue(rucomfyui::queue::Queue),
    /// System statistics were received.
    SystemStats(rucomfyui::system_stats::SystemStats),
}
impl AsyncResponse {
    /// Create an error response.
    pub fn error(category: impl std::fmt::Display, error: impl std::fmt::Display) -> Self {
        Self::Error {
            category: category.to_string(),
            error: error.to_string(),
        }
    }
}
impl Application {
    fn handle_async_responses(&mut self) -> bool {
        let mut needs_repaint = false;
        let mut load_workflow = None;

        // Process async events
        for event in self.async_output_rx.try_iter() {
            match event {
                AsyncResponse::ObjectInfo {
                    object_info: info,
                    client,
                } => {
                    self.graph = Some(rucomfyui_node_graph::ComfyUiNodeGraph::new(info));
                    self.client = Some(Arc::new(client));
                    load_workflow = Some(
                        rucomfyui::Workflow::from_json(include_str!(
                            "../../../crates/rucomfyui/examples/existing_workflow.json"
                        ))
                        .unwrap(),
                    );
                }
                AsyncResponse::QueueWorkflowResult { mapping, output } => {
                    self.last_prompt_queue_time = None;
                    let Some(graph) = self.graph.as_mut() else {
                        continue;
                    };

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
                        output
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
                AsyncResponse::Error { category, error } => {
                    self.error = Some((category, error));
                }
                AsyncResponse::LoadedWorkflow(workflow) => {
                    load_workflow = Some(workflow);
                }
                AsyncResponse::Queue(queue) => {
                    self.queue = queue;
                }
                AsyncResponse::SystemStats(stats) => {
                    self.system_stats = Some(stats);
                    self.system_stats_open = true;
                }
            }
            needs_repaint = true;
        }

        if let Some(workflow) = load_workflow {
            if let Err(err) = self.api_load(workflow) {
                self.async_output_tx
                    .send(AsyncResponse::error("Load", err))
                    .unwrap();
            }
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
