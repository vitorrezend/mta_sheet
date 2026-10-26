use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct SheetAclEntry {
    pub id: String,
    pub sheet_id: String,
    pub grantee_type: String, // "user", "room", "public"
    #[serde(default)]
    pub grantee_id: Option<String>,
    #[serde(default)]
    pub grantee_name: String,
    pub permission: String, // "read", "write", "admin"
    #[serde(default)]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct SheetShareSettings {
    pub sheet_id: String,
    pub sheet_name: String,
    #[serde(default)]
    pub share_token: Option<String>,
    #[serde(default = "default_share_permission")]
    pub share_permission: String, // "none" (restrito), "view" (qualquer um com o link)
    #[serde(default)]
    pub is_public: bool,
    #[serde(default)]
    pub acls: Vec<SheetAclEntry>,
}

fn default_share_permission() -> String {
    "none".to_string()
}
