use super::utils::{ROUTER_CONTENT, ROUTER_CONTENT_ID};

use crate::constants::{APP_NAME, APP_VERSION};
use rust_html::{rhtml, Template};

pub fn base_html(content: Template) -> Template {
    rhtml! { r#"
        <!DOCTYPE html>
        <html>
            {title_fragment("")}
            <body hx-ext="head-support" class="bg-gray-100 dark:bg-gray-900 min-h-screen flex flex-col">
                {content}
            </body> 
        </html>
    "# }
}

pub fn title_fragment(title: &str) -> Template {
    let page_title = match title.len() {
        0 => APP_NAME.to_string(),
        _ => format!("{} ∙ {}", title, APP_NAME),
    };

    rhtml! { r#"
        <head>
            <link href="/assets/main.css" rel="stylesheet" />
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link href="/assets/main.css" rel="stylesheet" />
            <link href="https://rsms.me/inter/inter.css" rel="stylesheet" />
            <script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js"></script>
            <script src="https://unpkg.com/htmx.org@2.0.3" integrity="sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq" crossorigin="anonymous"></script>
            <script src="https://unpkg.com/htmx-ext-head-support@2.0.1/head-support.js"></script>
            <title>{page_title}</title>
        </head>
    "# }
}

pub fn main_layout(content: Template, active_page: &str) -> Template {
    // get current year in rust

    let layout: Template = rhtml! { r#"
    <header class="flex items-center gap-8 container max-w-screen-lg mx-auto py-4 px-4">
        <a href="/" class="text-3xl text-indigo-500 font-bold">{APP_NAME}</a>
        <nav class="flex items-center gap-2" hx-target={ROUTER_CONTENT_ID} hx-push-url="true" hx-boost="true">
            {nav_fragments(active_page)}
        </nav>
    </header>
    <main 
        id="{ROUTER_CONTENT}"
        hx-target={ROUTER_CONTENT_ID}
        hx-boost="true"
        class="flex-1 container mx-auto max-w-screen-lg text-gray-900 dark:text-gray-100 px-4">
        {content}
    </main>
    <footer class="flex flex-col items-center py-2 gap-1 text-gray-600 dark:text-gray-400 ">
        <div class="text-center text-sm">
            <span>Developed by</span>
            <a href="https://brightbeam.no" target="_blank" class="text-gray-700 dark:text-gray-300">Brightbeam AS</a>
        </div>
        <div class="text-xs">
            v{APP_VERSION}
        </div>
    </footer>
    "# };

    base_html(layout)
}

pub fn nav_fragments(active_page: &str) -> Template {
    rhtml! { r#"
    {nav_link("/events", "Events", active_page)}
    "#}
}

pub fn nav_link(href: &str, text: &str, active_path: &str) -> Template {
    let active_class = "from-blue-100 to-blue-400 dark:from-blue-900 dark:to-blue-700";
    let dynamic_classes = {
        if active_path == href {
            ["bg-gradient-to-r saturate-150", active_class].join(" ")
        } else {
            ["hover:bg-gradient-to-r", active_class].join(" ")
        }
    };
    rhtml! { r#"
        <button hx-get="{href}" class="text-blue-500 dark:text-blue-200 px-2 py-1 rounded-md {dynamic_classes}">{text}</button>
    "# }
}
