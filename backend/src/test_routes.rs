use axum::{
    Router,
    routing::{get, post, get_service}, extract::{Path, Query}, response::{IntoResponse, Html}, serve::Serve,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;


/// This function sets up the routes for the test endpoints.
///
/// It returns a `Router` with two routes:
/// - "/test_query" which responds to GET requests and expects query parameters.
/// - "/test_path/:a/:b" which responds to GET requests and expects path parameters.
pub fn test_routes() -> Router {
    Router::new()
        //* route means that the service is mounted at the given path
        .route("/test_query", get(test_query))
        .route("/test_path/:a/:b", get(test_path))
        //* a fallback service is a service that is called if no other service matches the request
        .fallback_service(static_routes())
}

/// This struct represents the query parameters for the "/test_query" endpoint.
///
/// It has two fields, `a` and `b`, which are optional and can be of type `u32` and `String` respectively.
#[derive(Debug, Deserialize, Serialize)]
struct SomeStruct {
    a: Option<u32>,
    b: Option<String>,
}

/// This function handles GET requests to the "/test_query" endpoint.
///
/// It expects query parameters `a` and `b` and returns an HTML response with the values of `a` and `b`.
/// If `a` or `b` are not provided, it defaults to `0` and `"Some"` respectively.
///
/// Example: path: /test_query?a=1&b=2
async fn test_query(Query(params): Query<SomeStruct>) -> impl IntoResponse {
    Html(format!("SomeStruct Query: {} {}",  // Return a formatted string.
        params.a.unwrap_or(0), 
        params.b.unwrap_or("Some".to_string())
    ))
}

/// This function handles GET requests to the "/test_path/:a/:b" endpoint.
///
/// It expects path parameters `a` and `b` and returns an HTML response with the values of `a` and `b`.
///
/// Example: path: /test_path/1/2
async fn test_path(Path((a, b)): Path<(u32, String)>) -> impl IntoResponse {
    Html(format!("Path: {} {}",  // Return a formatted string.
        a, 
        b
    ))
}

/// This function sets up a static file server.
///
/// It returns a `Router` with a single route, "/", which serves files from the current directory.
/// The `ServeDir::new("./")` call creates a new `ServeDir` service that serves files from the current directory.
/// The `nest_service("/", get_service(ServeDir::new("./")))` call adds a route to the router that matches all requests to "/" and serves them with the `ServeDir` service.
///
/// The `ServeDir` service automatically handles requests for directory listings, `index.html` files, and `Range` requests.
pub fn static_routes() -> Router {
    Router::new()
        // nest_service means that the service is nested under the given path
        .nest_service("/", 
        // get_service converts a service into a handler function
        // a handler function is a function that takes a request and returns a future of a response
        // a future is a value that will be computed asynchronously (it means that the value will be computed in the background)
        get_service(ServeDir::new("./")
    ))
}
