use crate::frontend;
use crate::views::{auth, blog, page, tweet};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .route("", web::get().to(auth::get))
                    .route("", web::post().to(auth::login))
                    .route("", web::delete().to(auth::logout)),
            )
            .service(
                web::scope("/page")
                    .route("/{pagename}", web::get().to(page::get))
                    .route("/{pagename}", web::put().to(page::put)),
            )
            .service(
                web::scope("/tweet")
                    .route("/", web::get().to(tweet::index))
                    .route("/", web::post().to(tweet::post))
                    .route("/{id}", web::put().to(tweet::put))
                    .route("/{id}", web::delete().to(tweet::delete)),
            )
            .service(
                web::scope("/blog")
                    .route("/", web::get().to(blog::index))
                    .route("/", web::post().to(blog::post))
                    .route("/{id}", web::get().to(blog::get))
                    .route("/{id}", web::put().to(blog::put))
                    .route("/{id}", web::delete().to(blog::delete)),
            ),
    )
    .service(web::scope("").configure(frontend::config));
}
