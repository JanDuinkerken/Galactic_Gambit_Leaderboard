use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::entries;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Entries {
    pub id: i32,
    pub level_id: i32,
    pub username: String,
    pub seconds: f64,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = entries)]
pub struct NewEntry<'a> {
    pub level_id: &'a i32,
    pub username: &'a String,
    pub seconds: &'a f64,
}
