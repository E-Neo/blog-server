use crate::error::ServiceError;
use crate::models::blog::{Blog, MetaData, SlimBlog};
use crate::Pool;
use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryInfo {
    before_time: chrono::NaiveDateTime,
}

pub async fn index(
    web::Query(info): web::Query<QueryInfo>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query_index(info, pool)).await;
    match res {
        Ok(blogs) => Ok(HttpResponse::Ok().json(blogs)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

pub struct GetData {
    id: uuid::Uuid,
}

impl GetData {
    fn new(id: uuid::Uuid) -> Self {
        GetData { id }
    }
}

pub async fn get(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let get_id = req
        .match_info()
        .get("id")
        .unwrap()
        .parse()
        .map_err(|_| ServiceError::BadRequest(String::from("Invalid Id")))?;
    let res = web::block(move || query_get(GetData::new(get_id), pool)).await;
    match res {
        Ok(blog_detail) => Ok(HttpResponse::Ok().json(blog_detail)),
        Err(err) => match err {
            BlockingError::Error(service_error) => Err(service_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

#[derive(Deserialize)]
pub struct ComposeData {
    pub title: String,
    pub markdown: String,
}

pub async fn post(
    compose_data: web::Json<ComposeData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(_) = id.identity() {
        let res = web::block(move || query_post(compose_data.into_inner(), pool)).await;
        match res {
            Ok(meta_data) => Ok(HttpResponse::Ok().json(meta_data)),
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

pub async fn put(
    req: HttpRequest,
    compose_data: web::Json<ComposeData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(_) = id.identity() {
        let blog_id = req
            .match_info()
            .get("id")
            .unwrap()
            .parse()
            .map_err(|_| ServiceError::BadRequest(String::from("Invalid Id")))?;
        let res = web::block(move || query_put(blog_id, compose_data.into_inner(), pool)).await;
        match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[derive(Deserialize)]
pub struct DeleteData {
    pub id: uuid::Uuid,
}

impl DeleteData {
    fn new(id: uuid::Uuid) -> Self {
        DeleteData { id }
    }
}

pub async fn delete(
    req: HttpRequest,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    if let Some(_) = id.identity() {
        let delete_id = req
            .match_info()
            .get("id")
            .unwrap()
            .parse()
            .map_err(|_| ServiceError::BadRequest(String::from("Invalid Id")))?;
        let res = web::block(move || query_delete(DeleteData::new(delete_id), pool)).await;
        match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(err) => match err {
                BlockingError::Error(service_error) => Err(service_error),
                BlockingError::Canceled => Err(ServiceError::InternalServerError),
            },
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

fn query_index(info: QueryInfo, pool: web::Data<Pool>) -> Result<Vec<SlimBlog>, ServiceError> {
    use crate::schema::blog::dsl::{blog, created_time, id, title};
    let conn = &pool.get().unwrap();
    blog.filter(created_time.lt(info.before_time))
        .order(created_time.desc())
        .limit(20)
        .select((id, title, created_time))
        .load::<SlimBlog>(conn)
        .map_err(|err| err.into())
}

fn query_get(get_data: GetData, pool: web::Data<Pool>) -> Result<Blog, ServiceError> {
    use crate::schema::blog::dsl::{blog, id};
    let conn = &pool.get().unwrap();
    let mut items = blog.filter(id.eq(get_data.id)).load::<Blog>(conn)?;
    if let Some(detail) = items.pop() {
        Ok(detail)
    } else {
        Err(ServiceError::BadRequest(String::from("Invalid Id")))
    }
}

fn query_post(compose_data: ComposeData, pool: web::Data<Pool>) -> Result<MetaData, ServiceError> {
    use crate::schema::blog::dsl::{blog, created_time, id};
    let conn = &pool.get().unwrap();
    diesel::insert_into(blog)
        .values(&Blog::new(compose_data.title, compose_data.markdown))
        .returning((id, created_time))
        .get_result(conn)
        .map_err(|err| err.into())
}

fn query_put(
    blog_id: uuid::Uuid,
    compose_data: ComposeData,
    pool: web::Data<Pool>,
) -> Result<(), ServiceError> {
    use crate::schema::blog::dsl::{blog, id, markdown, title};
    let conn = &pool.get().unwrap();
    match diesel::update(blog.filter(id.eq(blog_id)))
        .set((
            title.eq(compose_data.title),
            markdown.eq(compose_data.markdown),
        ))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

fn query_delete(delete_data: DeleteData, pool: web::Data<Pool>) -> Result<(), ServiceError> {
    use crate::schema::blog::dsl::{blog, id};
    let conn = &pool.get().unwrap();
    match diesel::delete(blog)
        .filter(id.eq(delete_data.id))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
