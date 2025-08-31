pub mod credential_store;
pub mod oauth;

pub trait Authenticator {
    fn authenticate(&self) -> anyhow::Result<String>; // Returns an access token
}

pub struct ApiKeyAuthenticator {
    api_key: String,
}

impl ApiKeyAuthenticator {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl Authenticator for ApiKeyAuthenticator {
    fn authenticate(&self) -> anyhow::Result<String> {
        Ok(self.api_key.clone())
    }
}
