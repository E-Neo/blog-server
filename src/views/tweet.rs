use crate::error::ServiceError;
use crate::models::tweet::{MetaData, Tweet};
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
        Ok(tweets) => Ok(HttpResponse::Ok().json(tweets)),
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
        let tweet_id = req
            .match_info()
            .get("id")
            .unwrap()
            .parse()
            .map_err(|_| ServiceError::BadRequest(String::from("Invalid Id")))?;
        let res = web::block(move || query_put(tweet_id, compose_data.into_inner(), pool)).await;
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

fn query_index(info: QueryInfo, pool: web::Data<Pool>) -> Result<Vec<Tweet>, ServiceError> {
    use crate::schema::tweet::dsl::{created_time, tweet};
    let conn = &pool.get().unwrap();
    tweet
        .filter(created_time.lt(info.before_time))
        .order(created_time.desc())
        .limit(20)
        .load::<Tweet>(conn)
        .map_err(|err| err.into())
}

fn query_post(compose_data: ComposeData, pool: web::Data<Pool>) -> Result<MetaData, ServiceError> {
    use crate::schema::tweet::dsl::{created_time, id, tweet};
    let conn = &pool.get().unwrap();
    diesel::insert_into(tweet)
        .values(&Tweet::new(compose_data.markdown))
        .returning((id, created_time))
        .get_result(conn)
        .map_err(|err| err.into())
}

fn query_put(
    tweet_id: uuid::Uuid,
    compose_data: ComposeData,
    pool: web::Data<Pool>,
) -> Result<(), ServiceError> {
    use crate::schema::tweet::dsl::{id, markdown, tweet};
    let conn = &pool.get().unwrap();
    match diesel::update(tweet.filter(id.eq(tweet_id)))
        .set(markdown.eq(compose_data.markdown))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

fn query_delete(delete_data: DeleteData, pool: web::Data<Pool>) -> Result<(), ServiceError> {
    use crate::schema::tweet::dsl::{id, tweet};
    let conn = &pool.get().unwrap();
    match diesel::delete(tweet)
        .filter(id.eq(delete_data.id))
        .execute(conn)
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
