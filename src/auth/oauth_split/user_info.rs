use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub email: String,
    // Add other user info fields as needed, e.g., name, picture
}