use crate::schema::*;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Serialize, Queryable, Insertable)]
#[table_name = "tweet"]
pub struct Tweet {
    pub id: uuid::Uuid,
    pub markdown: String,
    pub created_time: chrono::NaiveDateTime,
}

impl Tweet {
    pub fn new(markdown: String) -> Self {
        Tweet {
            id: uuid::Uuid::new_v4(),
            markdown,
            created_time: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize, Queryable)]
pub struct MetaData {
    pub id: uuid::Uuid,
    pub created_time: chrono::NaiveDateTime,
}
