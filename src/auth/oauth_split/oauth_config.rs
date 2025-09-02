use serde::{Deserialize, Serialize};

// Placeholder for OAuth configuration
#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: Option<String>,
    pub scopes: Option<Vec<String>>, // New field for OAuth scopes
    // Add other OAuth related configs as needed
}
