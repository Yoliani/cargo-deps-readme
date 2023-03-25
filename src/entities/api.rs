use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateApiEntity {
    pub crates: Vec<Crate>,
    pub meta: Meta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crate {
    pub badges: Vec<String>,
    pub categores: Option<String>,
    pub created_at: String,
    pub description: Option<String>,
    pub documentation: Option<String>,
    pub downloads: i64,
    pub exact_match: bool,
    pub homepage: Option<String>,
    pub id: String,
    pub newest_version: String,
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
    pub total: i64,
}
