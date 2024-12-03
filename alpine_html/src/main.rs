use axum::{response::IntoResponse, routing::get, Router};
use rust_html::{rhtml, Template, TemplateGroup};

mod components;
mod layouts;

use components::*;
use layouts::*;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let assets_path = std::env::current_dir().unwrap();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/users", get(users))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .layer(LiveReloadLayer::new());

    // run our app with hyper, listening globally on port 4000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    let result: Template = rhtml! { r#"
        <div class="text-gray-900 dark:text-gray-100 space-y-6">
            <h1 class="text-3xl">Home</h1>
            <div class="space-y-4">
                <h3 class="text-lg">Some interesting boxes</h3>
                {grid_layout()}
            </div>
        </div>
    "# };

    main_layout(result, "/")
}

fn grid_layout() -> Template {
    rhtml! { r#"
    <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-4">
        {TemplateGroup(grid_items())}
    </div>
    "# }
}

fn grid_items() -> Vec<Template> {
    (0..10)
        .map(|i| {
            rhtml! { r#"
            <div class="bg-blue-400 dark:bg-blue-700 dark:text-gray-100 rounded-md px-4 py-2">
                {i}
            </div>
        "# }
        })
        .collect::<Vec<Template>>()
}

#[axum::debug_handler]
async fn users() -> impl IntoResponse {
    let result: Template = rhtml! { r#"
        <div class="space-y-4">
            <h1 class="text-3xl">Users Overview</h1>
            {UserTableComponent {}}
        </div>
    "# };

    main_layout(result, "/users")
}
