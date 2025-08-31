use std::sync::Arc;

use crate::auth::oauth_split::oauth_config::OAuthConfig;
use crate::auth::credential_store::CredentialStore;

#[derive(Debug)]
pub struct OAuthAuthenticator {
    pub config: OAuthConfig,
    pub credential_store: Arc<CredentialStore>,
}
