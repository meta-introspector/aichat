use serde::Deserialize;

// Placeholder for OAuth configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: Option<String>, // New field
    // Add other OAuth related configs as needed
}
