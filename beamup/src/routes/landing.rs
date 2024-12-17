use axum::response::IntoResponse;
use rust_html::{rhtml, Template};

use super::utils::{router_fragment_stack, HxReq, RouteOptions};
use crate::constants::APP_NAME;

pub async fn landing_page(hx_req: HxReq) -> impl IntoResponse {
    let template: Template = rhtml! { r#"
        <div class="space-y-8">
            {hero_card()}
            <article class="flex-1 flex items-center">
                <div class="grid grid-cols-2 gap-8 w-full">
                    {link_card("View Events", "/events")}
                    {link_card("Create Event", "/events/new")}
                </div>
            </article>
        </div>
    "# };

    router_fragment_stack(
        hx_req,
        RouteOptions {
            fragment: template,
            target: "/",
        },
    )
}

fn hero_card() -> Template {
    let styles = classes_to_string(vec![
        "text-center h-64 rounded-md",
        "flex flex-col items-center justify-center gap-4",
        // "bg-gradient-to-tr from-indigo-900 to-blue-800",
        "dot-pattern shadow-md",
    ]);

    rhtml! { r#"
        <div class="{styles}">
            <h1 class="text-4xl text-indigo-500">Welcome to <span class="font-bold">{APP_NAME}</span></h1>
            <p class="text-lg text-gray-200">No accounts, just connections.</p>
        </div>
    "# }
}

fn link_card(text: &str, href: &str) -> Template {
    let styles = classes_to_string(vec![
        "border",
        "bg-indigo-100",
        "hover:bg-indigo-200",
        "p-4",
        "rounded-md",
        "dark:border-indigo-700",
        "dark:bg-indigo-800",
        "hover:dark:bg-indigo-700",
    ]);
    rhtml! { r#"
        <a href="{href}" class="{styles}">
            <h2 class="text-xl">{text}</h2>
        </a>
    "# }
}

fn classes_to_string(classes: Vec<&str>) -> String {
    classes.join(" ")
}
