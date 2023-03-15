// models/mod.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Presence {
    pub id: String,
    pub id_str: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typ: String,
    pub connection_id: String,
    pub fullreason: String,
    pub cs: String,
    pub ip: String,
    pub protocol: String,
    pub reason: String,
    pub asset: String,
    pub time: String,
}
