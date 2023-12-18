#![allow(unused)]

use anyhow::Result;

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
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?; // Create a new HTTP client.
    
    // Test the home route.
    // hc.do_get("/home").await?.print().await?;

    // // Test path and query parameters.
    // hc.do_get("/test_query?a=1&b=hi_bro").await?.print().await?;
    // hc.do_get("/test_path/1/hi_bro").await?.print().await?;

    // Test static routes.
    hc.do_get("/src/main.rs").await?.print().await?;
    hc.do_get("/assets/html/index.html").await?.print().await?;

    Ok(())
}
