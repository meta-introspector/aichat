use async_trait::async_trait;
pub mod credential_store;
pub mod oauth_split;


#[async_trait]
pub trait Authenticator: Send + Sync {
    async fn authenticate(&self) -> anyhow::Result<String>; // Returns an access token
}

pub struct ApiKeyAuthenticator {
    api_key: String,
}

impl ApiKeyAuthenticator {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl Authenticator for ApiKeyAuthenticator {
    async fn authenticate(&self) -> anyhow::Result<String> {
        Ok(self.api_key.clone())
    }
}