use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Presence {
    pub id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_str: Option<String>,

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    pub connection_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullreason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl Default for Presence {
    fn default() -> Self {
        Self::new()
    }
}

impl Presence {
    pub fn new() -> Presence {
        Presence {
            id: 0,
            id_str: None,
            typ: None,
            reason: None,
            fullreason: None,
            asset: None,
            time: None,
            cs: None,
            ip: None,
            protocol: None,
            connection_id: 0,
        }
    }
}
