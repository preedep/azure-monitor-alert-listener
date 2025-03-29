use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct AppState {
    pub(crate) tenant_id: String,
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) asc_url: String,
    pub(crate) sender: String,
    pub(crate) reply_to: String,
    pub(crate) display_name: String,
    pub(crate) workspace_id: String,
}