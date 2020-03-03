use crate::error::ServiceError;
use crate::models::page::Page;
use crate::Pool;
use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

pub async fn get(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let pagename = String::from(req.match_info().get("pagename").unwrap());
    let res = web::block(move || query_get(&pagename, pool)).await;
    match res {
        Ok(page) => Ok(HttpResponse::Ok().json(page)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

#[derive(Deserialize)]
pub struct ComposeData {
    pub markdown: String,
}

pub async fn put(
    req: HttpRequest,
    compose_data: web::Json<ComposeData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(_) = id.identity() {
        let pagename = String::from(req.match_info().get("pagename").unwrap());
        let res = web::block(move || {
            query_put(
                Page::new(pagename, compose_data.into_inner().markdown),
                pool,
            )
        })
        .await;
        res.map(|_| HttpResponse::Ok().finish())
            .map_err(|err| match err {
                BlockingError::Error(service_error) => service_error,
                BlockingError::Canceled => ServiceError::InternalServerError,
            })
    } else {
        Err(ServiceError::Unauthorized)
    }
}

fn query_get(page_name: &str, pool: web::Data<Pool>) -> Result<Page, ServiceError> {
    use crate::schema::page::dsl::{page, pagename};
    let conn = &pool.get().unwrap();
    let mut items = page.filter(pagename.eq(page_name)).load::<Page>(conn)?;
    if let Some(p) = items.pop() {
        Ok(p)
    } else {
        Err(ServiceError::BadRequest(String::from(page_name)))
    }
}

fn query_put(page_data: Page, pool: web::Data<Pool>) -> Result<(), ServiceError> {
    use crate::schema::page::dsl::{markdown, page, pagename};
    let conn = &pool.get().unwrap();
    diesel::update(page.filter(pagename.eq(&page_data.pagename)))
        .set(markdown.eq(page_data.markdown))
        .execute(conn)
        .map(|_| ())
        .map_err(|err| err.into())
}
