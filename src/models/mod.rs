// models/mod.rs

use super::schema::presences;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Queryable, Deserialize, Serialize)]
#[diesel(table_name = presences)]
#[serde(rename_all = "lowercase")]
#[derive(Debug)]
pub struct Presence {
    pub id: i64,
    pub id_str: String,
    pub r#type: String,
    pub connection_id: i64,
    pub fullreason: String,
    pub cs: String,
    pub ip: String,
    pub protocol: String,
    pub reason: String,
    pub asset: String,
    pub time: String,
}

impl fmt::Display for Presence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id_str)
    }
}
