use crate::schema::*;
use diesel::{Insertable, Queryable};
use serde::Serialize;

#[derive(Serialize, Queryable, Insertable)]
#[table_name = "blog"]
pub struct Blog {
    pub id: uuid::Uuid,
    pub title: String,
    pub markdown: String,
    pub created_time: chrono::NaiveDateTime,
}

impl Blog {
    pub fn new(title: String, markdown: String) -> Self {
        Blog {
            id: uuid::Uuid::new_v4(),
            title,
            markdown,
            created_time: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize, Queryable)]
pub struct SlimBlog {
    pub id: uuid::Uuid,
    pub title: String,
    pub created_time: chrono::NaiveDateTime,
}

impl SlimBlog {
    pub fn new(id: uuid::Uuid, title: String, created_time: chrono::NaiveDateTime) -> Self {
        SlimBlog {
            id,
            title,
            created_time,
        }
    }
}

#[derive(Serialize, Queryable)]
pub struct MetaData {
    pub id: uuid::Uuid,
    pub created_time: chrono::NaiveDateTime,
}
