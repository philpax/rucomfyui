//! Configuration types for rucomfyui_mlua.
//!
//! These types allow integrators to control which functionality is exposed to Lua.

/// Configuration for which Client methods to expose to Lua.
///
/// Each field corresponds to a method on the Client type.
/// If a field is `false`, the corresponding method will not be available in Lua.
#[derive(Debug, Clone, Default)]
pub struct ClientConfig {
    // Object Info
    /// Allow `get_object_info` - retrieves info about all available nodes.
    pub get_object_info: bool,
    /// Allow `get_object_for_name` - retrieves info about a specific node.
    pub get_object_for_name: bool,

    // Queue Operations
    /// Allow `queue` - queues a workflow for execution.
    pub queue: bool,
    /// Allow `easy_queue` - queues a workflow and waits for results.
    pub easy_queue: bool,
    /// Allow `get_queue` - gets the current queue status.
    pub get_queue: bool,
    /// Allow `interrupt` - interrupts the current workflow.
    pub interrupt: bool,
    /// Allow `delete_from_queue` - deletes workflows from the queue.
    pub delete_from_queue: bool,
    /// Allow `clear_queue` - clears the entire queue.
    pub clear_queue: bool,

    // History Operations
    /// Allow `get_history` - gets history with a maximum number of items.
    pub get_history: bool,
    /// Allow `get_history_for_prompt` - gets history for a specific prompt.
    pub get_history_for_prompt: bool,
    /// Allow `delete_from_history` - deletes entries from history.
    pub delete_from_history: bool,
    /// Allow `clear_history` - clears all history.
    pub clear_history: bool,

    // Model Operations
    /// Allow `get_model_categories` - gets all model categories.
    pub get_model_categories: bool,
    /// Allow `get_models` - gets models in a category.
    pub get_models: bool,

    // System Operations
    /// Allow `system_stats` - gets system statistics.
    pub system_stats: bool,
    /// Allow `free` - frees memory and/or unloads models.
    pub free: bool,

    // Upload Operations
    /// Allow `upload` - uploads an image to ComfyUI.
    pub upload: bool,
}

impl ClientConfig {
    /// Create a config with all methods enabled.
    pub fn all() -> Self {
        Self {
            get_object_info: true,
            get_object_for_name: true,
            queue: true,
            easy_queue: true,
            get_queue: true,
            interrupt: true,
            delete_from_queue: true,
            clear_queue: true,
            get_history: true,
            get_history_for_prompt: true,
            delete_from_history: true,
            clear_history: true,
            get_model_categories: true,
            get_models: true,
            system_stats: true,
            free: true,
            upload: true,
        }
    }

    /// Create a config with no methods enabled.
    pub fn none() -> Self {
        Self::default()
    }

    /// Create a config with only read operations enabled.
    ///
    /// This includes: get_object_info, get_object_for_name, get_queue,
    /// get_history, get_history_for_prompt, get_model_categories, get_models, system_stats
    pub fn read_only() -> Self {
        Self {
            get_object_info: true,
            get_object_for_name: true,
            get_queue: true,
            get_history: true,
            get_history_for_prompt: true,
            get_model_categories: true,
            get_models: true,
            system_stats: true,
            ..Self::default()
        }
    }

    /// Create a config suitable for executing workflows.
    ///
    /// This includes read operations plus: queue, easy_queue
    pub fn execute() -> Self {
        Self {
            queue: true,
            easy_queue: true,
            ..Self::read_only()
        }
    }
}

/// Top-level configuration for the rucomfyui Lua module.
#[derive(Debug, Clone, Default)]
pub struct IntegrationConfig {
    /// Configuration for Client methods.
    pub client: ClientConfig,
}

impl IntegrationConfig {
    /// Create a config with all functionality enabled.
    pub fn all() -> Self {
        Self {
            client: ClientConfig::all(),
        }
    }

    /// Create a config with no functionality enabled.
    pub fn none() -> Self {
        Self::default()
    }
}
