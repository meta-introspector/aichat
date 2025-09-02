use tlns_google_oauth2::scopes::Scopes;
use tlns_google_oauth2::ToGoogleScope;

pub struct AdminAccess;

impl AdminAccess {
    pub fn required_scopes() -> Vec<&'static str> {
        vec![
            "https://www.googleapis.com/auth/cloud-platform",
            "https://www.googleapis.com/auth/admin.directory.user",
        ]
    }

    pub fn check_access(granted_scopes: &[Scopes]) -> bool {
        Self::required_scopes().iter().all(|req_scope| {
            granted_scopes.iter().any(|granted_scope| granted_scope.to_google_scope() == *req_scope)
        })
    }
}
