#![allow(unused)]

// ? Module imports ---------------------------------------------------------------------------

//* Standard library imports
// (Currently, no standard library imports are used.)

//* Third-party crate imports
use dev_utils::{
    print_app_data, // Utility function to print application data.
    // log::rlog::RLog, // Custom logging utility.
};
use axum::{
    routing::{get, post},
    Router,
    response::{Html, Response}, middleware,
};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{Level, info};

//* Internal module imports
mod config;
mod error;
mod web;

// Test related imports
mod test_routes;
/// Re-export the `config` function from the `config` module.
/// This allows the `config` function to be used as `config::config()` instead of `config::config::config()`.
// pub use config::config;
use test_routes::test_routes;

// ? Main ------------------------------------------------------------------------------------
#[tokio::main]
// The `Box<dyn std::error::Error>` return type allows the function to return any error type.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_app_data(file!());  // Print application data such as version and build info.
    
    tracing_subscriber::fmt()  // Format the output of the tracing subscriber.
        .without_time()  // Do not include the time in the output.
        .with_target(false)  // Do not include the target in the output.
        // .with_max_level(Level::INFO)  // Set the maximum log level to INFO.
        .with_max_level(Level::TRACE)  // Set the maximum log level to INFO.
        .init();  // Initialize the tracing subscriber.

    // * Code...
    let app = Router::new()
        .route("/hello_world", get(Html("Hello World!")))
        .merge(test_routes::test_routes())
        .merge(web::routes_login::routes())
        // .layer(middleware::map_response(main_response_mapper))
        // .layer(CookieManagerLayer::new())
        // * Static file server ---------------------------------------------------------------
        // to see the static files, run `npm run build` in the frontend directory
        // .nest_service("/", ServeDir::new("./frontend/build/index.html"))
        .nest_service("/", ServeDir::new("./"))
        // to check the static files on the server use 127.0.0.1:8080/frontend/build/index.html
    ;

    let port: u32 = 8080;
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{port}")).await.unwrap();
    info!("LISTENING ON {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())  // Return `Ok` to indicate that the program ran successfully.
}


async fn main_response_mapper(response: Response) -> Response {
    tracing::trace!("RESPONSE: {:?}\n", response);


    response
}












// good check character: ✓
