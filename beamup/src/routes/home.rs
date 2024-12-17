use axum::response::IntoResponse;
use rust_html::{rhtml, Template};

use crate::components::*;

use super::utils::{router_fragment_stack, HxReq, RouteOptions};

pub async fn home_page(hx_req: HxReq) -> impl IntoResponse {
    let result: Template = rhtml! { r#"
        <div class="text-gray-900 dark:text-gray-100 space-y-6">
            <h1 class="text-3xl">Home</h1>
            <div x-data="{{ open: false }}" class="space-y-2">
                <button @click="open = ! open" type="button" class="{ButtonColor::Primary}">Do you want the juith? 🧃</button>
                <blockquote x-show="open" @click.outside="open = false" class="p-4 my-4 bg-gray-50 border-l-4 border-gray-300 dark:border-gray-500 dark:bg-gray-800">
                    The truth is that the universe has been answering you all of your life, but you cannot receive the answers unless you are awake.
                </blockquote>
            </div>
            
            <div class="space-y-4">
                <h3 class="text-lg">Some interesting boxes</h3>
            </div>
        </div>
    "# };

    router_fragment_stack(
        hx_req,
        RouteOptions {
            fragment: result,
            target: "/home",
        },
    )
}
