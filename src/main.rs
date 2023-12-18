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
    response::Html,
};
use tracing::{Level, info};
use tracing_subscriber::{EnvFilter, fmt::format};

//* Internal module imports
mod config;
mod errors;

/// Re-export the `config` function from the `config` module.
/// This allows the `config` function to be used as `config::config()` instead of `config::config::config()`.
pub use config::config;


// ? Main ------------------------------------------------------------------------------------
#[tokio::main]
async fn main() {
// async fn main() -> Result<()> {
    print_app_data(file!());  // Print application data such as version and build info.
    init_trace_subscriber(Level::INFO);  // Initialize the tracing subscriber.

    let app = Router::new()
        .route("/hello", get(hello))
        .route("/home", get(serve_html))
        // .route("/hi_bro", get("Hi bro!"))
    ;

    let port: u32 = 8080;
    let listener = tokio::net::TcpListener::bind(format!("localhost:{port}")).await.unwrap();
    info!("LISTENING ON {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();


    // Ok(())
}


/// Initialize the tracing subscriber.
fn init_trace_subscriber(level: Level) {
    // let subscriber = tracing_subscriber::fmt()
    //     .with_max_level(level)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    tracing_subscriber::fmt()
        .without_time()  // Do not include the time in the output.
        .with_target(false)  // Do not include the target in the output.
        .with_max_level(level)
        .init();
}

async fn hello() -> Html <&'static str> {
    Html("<h1>Hello, World!</h1>")
}


use tokio::fs;

// async fn serve_html(path: &'static str) -> Html<String> {
async fn serve_html() -> Html<String> {
    // let html_content = fs::read_to_string(" <&'static str>").await.unwrap();
    Html(fs::read_to_string("html/index.html").await.unwrap())
}