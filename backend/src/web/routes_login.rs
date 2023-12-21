use crate::{error::{
    Result,  // Custom Result type. (Result<T> == std::result::Result<T, AppError>)
    AppError  // Custom error type for the application.
}, web};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use axum::{
    extract::{Extension, Path},
    response::Html,
    routing::post,
    Router, 
    Json,
};
use tower_cookies::{Cookies, Cookie};


pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
}


async fn api_login(
    cookies: Cookies,  // Cookies are automatically parsed from the request.
    payload: Json<LoginPayload>
) -> Result<Json<Value>> {
    tracing::info!("API LOGIN:\n{:#?}", payload);

    match (payload.username.as_str(), payload.password.as_str()) {
        ("admin", "etesech") => {  // If the username and password are correct...
            // todo: Implement a real authentication system
            // how:
            // 1. Generate a random token.
            // 2. Store the token in a database.
            // 3. Send the token to the client.
            // 4. When the client sends a request, check if the token is valid.
            // 5. If the token is valid, allow the request.
            // 6. If the token is invalid, deny the request.
            // 7. Periodically delete expired/ unused tokens from the database.
            cookies.add(Cookie::new(web::COOKIE_AUTH_TOKEN, "user-1.exp.sign"));
            
            Ok(Json(json!({  // Return a JSON response.
                "result": {
                    "success": true,
                }
            })))
        },
        _ => Err(AppError::LoginFail)
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
