use std::sync::atomic::{AtomicU64, Ordering};

use crate::Client;

/// Constructors for [`Client`].
impl Client {
    /// Create a new client with the default [`reqwest`] client.
    pub fn new(api_base: impl Into<String>) -> Self {
        Self::new_with_client(api_base, reqwest::Client::default())
    }
    /// Create a new client with a custom [`reqwest`] client.
    /// This is useful for setting custom timeouts, headers, etc.
    pub fn new_with_client(api_base: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            api_base: api_base.into(),
            client,
            client_id: generate_client_id(),
        }
    }

    /// Override the [`Client::client_id`] used to correlate queued prompts and
    /// WebSocket events.
    ///
    /// A unique ID is generated automatically on construction, so this is only
    /// needed if you want to supply your own (e.g. a stable per-session UUID).
    pub fn with_client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = client_id.into();
        self
    }
}

/// Generates a process-unique client ID.
///
/// We avoid pulling in a UUID/random dependency (and the associated
/// `getrandom` WASM configuration) by combining the current time with a
/// monotonic counter. This is unique within a process and unlikely to collide
/// across processes; callers needing a stronger guarantee can supply their own
/// via [`Client::with_client_id`].
fn generate_client_id() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = web_time::SystemTime::now()
        .duration_since(web_time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("rucomfyui-{nanos:x}-{counter:x}")
}
