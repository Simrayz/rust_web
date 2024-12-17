use std::sync::{Arc, Mutex};

use axum::http::Request;
use axum::{routing::get, routing::post, Router};

mod components;
mod constants;
mod layouts;
mod routes;

use layouts::*;
use routes::*;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tower_livereload::LiveReloadLayer;
use tracing::Level;

struct AppState {
    todos: Mutex<Vec<String>>,
}

fn not_htmx_predicate<T>(req: &Request<T>) -> bool {
    !req.headers().contains_key("hx-request")
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let assets_path = std::env::current_dir().unwrap();
    let api_router = Router::new()
        .route("/hello", get(hello_from_the_server))
        .route("/todos", post(add_todo).delete(clear_todos));

    let app_state = Arc::new(AppState {
        todos: Mutex::new(vec![]),
    });

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .nest("/api", api_router)
        .route("/", get(home_page))
        .route("/users", get(users_page))
        .route("/todos", get(todos_page))
        .with_state(app_state)
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(LiveReloadLayer::new().request_predicate(not_htmx_predicate))
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
