#[macro_use]
extern crate diesel;

use diesel::{
    r2d2::{self, ConnectionManager},
    PgConnection,
};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod config;
pub mod error;
pub mod frontend;
pub mod models;
pub mod schema;
pub mod views;
