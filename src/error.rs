use actix_web::{HttpResponse, ResponseError};
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Internal server error")]
    Internal,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(err) => {
                HttpResponse::InternalServerError().json(ErrorResponse::new(err.to_string()))
            }
            AppError::NotFound => {
                HttpResponse::NotFound().json(ErrorResponse::new("Resource not found".to_string()))
            }
            AppError::Validation(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse::new(msg.to_string()))
            }
            AppError::Internal => {
                HttpResponse::InternalServerError().json(ErrorResponse::new("Internal server error".to_string()))
            }
        }
    }
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl ErrorResponse {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}
