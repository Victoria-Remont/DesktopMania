use actix_web::{HttpResponse,ResponseError};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind,Error as DBError};
use std::convert::From;
use uuid::Error as UuidError;

#[derive(Debug,Display)]
pub enum AuthError {
    #[display(fmt = "DucplicateValue: {}",_0)]
    DucplicateValue(String),

    #[display(fmt = "BadId")]
    BadId,
    
    #[display(fmt = "GenericError: {}",_0)]
    GenericError(String),

}

impl ResponseError for AuthError{
    fn error_response(&self) -> HttpResponse{
        match self {
            AuthError::BadId =>
            HttpResponse::BadRequest().json("Invalid ID"),

            AuthError::DucplicateValue(ref message) =>
            HttpResponse::BadRequest().json(message),

            AuthError::GenericError(ref message) =>
            HttpResponse::BadRequest().json(message),
        }
    }
}

impl From<UuidError> for AuthError{
    fn from(_: UuidError) -> AuthError{
        AuthError::BadId
    }
}

impl From<DBError> for AuthError{
    fn from(error: DBError) -> AuthError{
        match error {
            DBError::DatabaseError(kind,info) =>{
                let message = info.details().unwrap_or_else(|| info.message()).to_string();
            
                match kind{
                DatabaseErrorKind::UniqueViolation =>
                AuthError::DucplicateValue(message),
                 _=> AuthError::GenericError(message)

            }
    
            }
            _ => AuthError::GenericError(String::from("Some database error occured")),
            
        }
    }
}


