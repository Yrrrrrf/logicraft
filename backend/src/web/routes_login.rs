use crate::error::{
    Result,  // Custom Result type. (Result<T> == std::result::Result<T, AppError>)
    AppError  // Custom error type for the application.
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use axum::{
    extract::{Extension, Path},
    response::Html,
    routing::post,
    Router, 
    Json,
};


pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
}


async fn api_login(
    payload: Json<LoginPayload>
) -> Result<Json<Value>> {
    tracing::info!("API LOGIN:\n{:#?}", payload);

    if payload.username == "admin" && payload.password == "etesech" {
        Ok(Json(json!({  // Return a JSON response.
            "result": {
                "success": true,
            }
        })))
    } else {
        Err(AppError::LoginFail)
    }
}


/// The `login` function is used to create a new `Router` that handles the `/login` route.
#[derive(
    Debug, 
    Serialize,
    Deserialize, 
)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}
