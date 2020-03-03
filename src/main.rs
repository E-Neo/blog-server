use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use api::config;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .data(
                r2d2::Pool::builder()
                    .build(ConnectionManager::<PgConnection>::new(
                        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
                    ))
                    .unwrap(),
            )
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(
                    std::env::var("COOKIE_SECRET_KEY")
                        .expect("COOKIE_SECRET_KEY must be set")
                        .as_bytes(),
                )
                .name("auth")
                .path("/")
                .max_age_time(chrono::Duration::days(1))
                .secure(
                    std::env::var("COOKIE_SECURE")
                        .unwrap_or(String::from("true"))
                        .parse()
                        .expect("COOKIE_SECURE must be set properly"),
                ),
            ))
            .data(web::JsonConfig::default().limit(4096))
            .configure(config::config)
    })
    .bind((
        "0.0.0.0",
        std::env::var("PORT")
            .unwrap_or(String::from("8000"))
            .parse()
            .expect("PORT must be set properly"),
    ))?
    .run()
    .await
}
