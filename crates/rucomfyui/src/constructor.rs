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
        }
    }
}
