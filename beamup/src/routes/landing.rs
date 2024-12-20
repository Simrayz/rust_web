use axum::response::IntoResponse;
use rust_html::{rhtml, Template};

use super::utils::{router_fragment_stack, HxReq, RouteOptions};
use crate::constants::{APP_DESCRIPTION, APP_NAME};

pub async fn landing_page(hx_req: HxReq) -> impl IntoResponse {
    let template: Template = rhtml! { r##"
        <div class="space-y-8">
            {hero_card()}
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 justify-center">
                {info_card(
                    "Start planning in seconds.", 
                    "Create your event with just a few clicks. As the host, all you need is one account to get started—no extra steps, no hassle."
                )}
                {info_card(
                    "Your guests, your way.",
                    "Attendees don’t need to sign up. Simply share a unique invite link, and they can RSVP with ease. No barriers, just connection."
                )}
                {info_card(
                    "Clarity for your plans.",
                    "Track who’s coming in real time. With BeamUp, replies are instant, keeping you in control and stress-free."
                )}
            </div>
            <article class="flex-1 flex items-center">
                <div class="grid grid-cols-2 gap-8 w-full">
                    {link_card("View Events", "/events")}
                    {link_card("Create Event", "/events/new")}
                </div>
            </article>
        </div>
    "## };

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
        "diagonal-pattern border border-indigo-200 dark:border-none dark:shadow-md",
    ]);

    rhtml! { r#"
        <div class="{styles}">
            <h1 class="text-4xl text-indigo-500">Welcome to <span class="font-bold">{APP_NAME}</span></h1>
            <p class="text-lg text-gray-800 dark:text-gray-200">{APP_DESCRIPTION}</p>
        </div>
    "# }
}

fn info_card(title: &str, text: &str) -> Template {
    rhtml! { r#"
        <div class="bg-gray-50 border border-indigo-200 p-4 rounded-md dark:border-indigo-900 space-y-2 dark:bg-indigo-900 dark:bg-opacity-20">
            <h2 class="text-xl text-indigo-800 dark:text-indigo-500">{title}</h2>
            <p class="text-sm lg:text-base text-gray-600 dark:text-gray-200">{text}</p>
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
