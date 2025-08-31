use std::sync::Arc;

use crate::auth::oauth_split::oauth_config::OAuthConfig;
use crate::auth::credential_store::CredentialStore;
use crate::auth::oauth_split::oauth_authenticator_struct::OAuthAuthenticator;

impl OAuthAuthenticator {
    pub fn new(config: OAuthConfig, credential_store: Arc<CredentialStore>) -> Self {
        Self { config, credential_store }
    }
}
