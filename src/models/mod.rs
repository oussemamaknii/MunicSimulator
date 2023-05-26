// models/mod.rs

pub mod presences;
pub mod tracks;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Req<T> {
    pub meta: Eveent,
    pub payload: T,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Eveent {
    pub account: String,
    pub event: String,
}
