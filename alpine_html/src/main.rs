use axum::{routing::get, Router};

mod components;
mod layouts;
mod routes;

use layouts::*;
use routes::*;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tower_livereload::LiveReloadLayer;
use tracing::Level;

#[tokio::main]
async fn main() -> miette::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new().route("/hello", get(hello_from_the_server));

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .nest("/api", api_router)
        .route("/", get(home_page))
        .route("/users", get(users_page))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(LiveReloadLayer::new())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    // run our app with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[axum::debug_handler]
async fn hello_from_the_server() -> &'static str {
    "Hello from the server!"
}
