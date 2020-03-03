use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/favicon.ico").route(web::get().to(favicon_ico)))
        .service(web::resource("/layout.css").route(web::get().to(layout_css)))
        .service(web::resource("/marked.min.js").route(web::get().to(marked_min_js)))
        .service(web::resource("/index.js").route(web::get().to(index_js)))
        .service(web::resource("/tweet/").route(web::get().to(index)))
        .service(web::resource("/blog/").route(web::get().to(index)))
        .service(web::resource("/blog/{id}").route(web::get().to(index)))
        .service(web::resource("/login").route(web::get().to(index)))
        .service(web::resource("/logout").route(web::get().to(index)))
        .service(web::resource("/admin/").route(web::get().to(index)))
        .service(web::resource("/admin/tweet/").route(web::get().to(index)))
        .service(web::resource("/admin/blog/").route(web::get().to(index)))
        .service(web::resource("/admin/blog/{id}").route(web::get().to(index)));
}

async fn favicon_ico() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/vnd.microsoft.icon")
        .body(include_bytes!("static/favicon.ico").as_ref())
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("static/index.html"))
}

async fn layout_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(include_str!("static/layout.css"))
}

async fn marked_min_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(include_str!("static/marked.min.js"))
}

async fn index_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(include_str!("static/index.js"))
}
