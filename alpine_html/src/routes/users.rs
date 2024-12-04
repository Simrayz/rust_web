use axum::response::IntoResponse;
use rust_html::{rhtml, Template};

use crate::main_layout;

use crate::components::*;

#[axum::debug_handler]
pub async fn users_page() -> impl IntoResponse {
    let result: Template = rhtml! { r#"
        <div class="space-y-4">
            <h1 class="text-3xl">Users Overview</h1>
            {UserTableComponent {}}
        </div>
    "# };

    main_layout(result, "/users")
}
