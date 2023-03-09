// models/mod.rs

use super::schema::presences;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Serialize)]
#[diesel(table_name = presences)]
pub struct Presence {
    pub id: i32,
    pub id_str: String,
    pub msg_type: String,
    pub reason: String,
    pub asset: String,
    pub time: String,
}
