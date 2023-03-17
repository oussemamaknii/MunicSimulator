// models/mod.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Presence {
    pub id: i64,
    pub id_str: Option<String>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typ: Option<String>,
    pub connection_id: i64,
    pub fullreason: Option<String>,
    pub cs: Option<String>,
    pub ip: Option<String>,
    pub protocol: Option<String>,
    pub reason: Option<String>,
    pub asset: Option<String>,
    pub time: Option<String>,
}
