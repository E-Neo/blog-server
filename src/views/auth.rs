use crate::error::ServiceError;
use crate::models::user::User;
use crate::Pool;
use actix_identity::Identity;
use actix_web::{error::BlockingError, web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct SlimUser {
    pub username: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            username: user.username,
        }
    }
}

pub async fn get(id: Identity) -> Result<HttpResponse, ServiceError> {
    if let Some(user) = id.identity() {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ServiceError::Unauthorized)
    }
}

pub async fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let res = web::block(move || query(auth_data.into_inner(), pool)).await;
    match res {
        Ok(user) => {
            id.remember(serde_json::to_string(&user).unwrap());
            Ok(HttpResponse::Ok().finish())
        }
        Err(err) => match err {
            BlockingError::Error(unauthorized_error) => Err(unauthorized_error),
            BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}

pub async fn logout(id: Identity) -> Result<HttpResponse, ServiceError> {
    if let Some(_) = id.identity() {
        id.forget();
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ServiceError::Unauthorized)
    }
}

fn verify(password: &str, salt: &str, hash: &str) -> bool {
    argon2::hash_encoded(
        password.as_bytes(),
        salt.as_bytes(),
        &argon2::Config::default(),
    )
    .unwrap()
    .as_str()
        == hash
}

fn query(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::api_user::dsl::{api_user, username};
    let conn = &pool.get().unwrap();
    let mut items = api_user
        .filter(username.eq(&auth_data.username))
        .load::<User>(conn)?;
    if let Some(user) = items.pop() {
        if verify(&auth_data.password, &user.salt, &user.hash) {
            return Ok(user.into());
        }
    }
    Err(ServiceError::Unauthorized)
}
