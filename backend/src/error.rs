//! # Error types
//! 
//! This module contains the error types used in the application.
//! 
//! The error types are defined as enums, and they are used to represent the different types of errors that can occur in the application.
//! 
//! 
use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    }
};


/// Specify the error types that can occur in the application.
/// It works as a wrapper around the standard library's `Result` type.
pub type Result<T> = std::result::Result<T, AppError>;
// ^ Wrapper (en) == Envoltura (es) == Enveloppe (fr)


#[derive(
    Debug,  // This trait allows us to use the `{:?}` formatter to print the error.
)]
pub enum AppError {
    LoginFail,  // This error is used when the user fails to login.


    EnvVarNotFound(&'static str),
    // DatabaseError(String),
    // DatabasePoolError(String),
    // DatabasePoolTimeoutError(String),
    // DatabaseUrlError(String),
    // DatabaseConnectionError(String),
    // DatabaseQueryError(String),
    // DatabaseTransactionError(String),
    // DatabaseMigrat
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // use tracing to log the error
        tracing::error!("{:?}", self);

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}


