#![allow(unused)]

use anyhow::Result;
use serde_json::json;


use dev_utils::{
    print_app_data, // Utility function to print application data.
};

/// The main function of the application.
///
/// This function sets up an HTTP server that listens on localhost port 8080.
/// The server has a single route, "/", which responds to GET requests with the text "Hello, World!".
/// The server uses the `TraceLayer` middleware from the `tower_http` crate to add tracing for all HTTP requests.
///
/// The function is asynchronous and returns a `Result`. If the server runs successfully, it returns `Ok(())`.
/// If there is an error (for example, if the server port is already in use), it returns `Err`.
#[tokio::main]
async fn main() -> Result<()> {
    print_app_data(file!());  // Print application data such as version and build info.
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?; // Create a new HTTP client.
    
    // //* Test the home route.
    // // The home route is a route that responds to GET requests with the text "Hello, World!".
    // hc.do_get("/home").await?.print().await?;

    // //* Test path and query parameters.
    // // A path parameter is a parameter that is part of the path.
    // hc.do_get("/test_query?a=1&b=hi_bro").await?.print().await?;
    // hc.do_get("/test_path/1/hi_bro").await?.print().await?;

    // //* Test static routes.
    // // A static route is a route that serves static files from a directory.
    // hc.do_get("/src/main.rs").await?.print().await?;
    // hc.do_get("/assets/icons/data-server.png").await?.print().await?;


    hc.do_post(
        "/api/login", 
        json!({
            "username": "admin",
            "password": "etesech",
            // "password": "admin",
        })
    ).await?.print().await?;








    Ok(())
}
