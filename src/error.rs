use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DatabaseError};

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Not Found")]
    NotFound,

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(message) => HttpResponse::BadRequest().json(message),
            ServiceError::NotFound => HttpResponse::NotFound().json("Not Found"),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

impl From<DatabaseError> for ServiceError {
    fn from(error: DatabaseError) -> Self {
        match error {
            DatabaseError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    ServiceError::BadRequest(String::from(info.details().unwrap_or(info.message())))
                } else {
                    ServiceError::InternalServerError
                }
            }
            _ => ServiceError::InternalServerError,
        }
    }
}
