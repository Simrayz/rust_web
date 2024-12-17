use axum::response::IntoResponse;
use rust_html::{rhtml, Template};

use crate::components::*;

use super::utils::{router_fragment_stack, HxReq, RouteOptions};

#[axum::debug_handler]
pub async fn users_page(hx_req: HxReq) -> impl IntoResponse {
    let result: Template = rhtml! { r#"
        <div class="space-y-4">
            <h1 class="text-3xl">Users Overview</h1>
            {UserTableComponent {}}
        </div>
    "# };

    router_fragment_stack(
        hx_req,
        RouteOptions {
            fragment: result,
            target: "/users",
        },
    )
}
