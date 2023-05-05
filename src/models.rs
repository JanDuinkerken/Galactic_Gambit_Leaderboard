use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Entries {
    pub id: i32,
    pub level_id: i32,
    pub username: String,
    pub seconds: i32,
}
