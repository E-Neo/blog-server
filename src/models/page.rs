use crate::schema::*;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "page"]
pub struct Page {
    pub pagename: String,
    pub markdown: String,
}

impl Page {
    pub fn new(pagename: String, markdown: String) -> Self {
        Page { pagename, markdown }
    }
}
