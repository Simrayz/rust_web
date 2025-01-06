use super::utils::RouteOptions;

use crate::constants::{APP_NAME, APP_VERSION};
use rust_html::{rhtml, Template};

pub fn base_html(content: Template, head: Template) -> Template {
    rhtml! { r#"
        <!DOCTYPE html>
        <html>
            {head}
            <body class="bg-gray-100 dark:bg-gray-900 min-h-screen flex flex-col">
                {content}
            </body> 
        </html>
    "# }
}

pub fn head_fragment(options: RouteOptions) -> Template {
    let page_title = match options.title.len() {
        0 => APP_NAME.to_string(),
        _ => format!("{} ∙ {}", options.title, APP_NAME),
    };

    rhtml! { r#"
        <head>
            <link href="/assets/main.css" rel="stylesheet" hx-p />
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link href="/assets/main.css" rel="stylesheet" />
            <link href="https://rsms.me/inter/inter.css" rel="stylesheet" />
            <script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js"></script>
            <script src="https://unpkg.com/htmx.org@2.0.3" integrity="sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq" crossorigin="anonymous"></script>
            <title>{page_title}</title>
            {options.head}
        </head>
    "# }
}

pub fn main_layout(options: RouteOptions) -> Template {
    // get current year in rust

    let layout: Template = rhtml! { r#"
    <header class="flex items-center gap-8 container max-w-screen-lg mx-auto py-4 px-4">
        <a href="/" class="text-3xl text-indigo-500 font-bold">{APP_NAME}</a>
        <nav class="flex items-center gap-2">
            {nav_fragments(options.path, options.exact)}
        </nav>
    </header>
    <main 
        class="flex-1 container mx-auto max-w-screen-lg text-gray-900 dark:text-gray-100 px-4">
        {options.content}
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

    base_html(layout, head_fragment(options))
}

pub fn nav_fragments(active_page: &str, exact: bool) -> Template {
    rhtml! { r#"
    {nav_link("/events", "Events", active_page, exact)}
    "#}
}

pub fn nav_link(href: &str, text: &str, active_path: &str, exact: bool) -> Template {
    let is_active = if exact {
        active_path == href
    } else {
        active_path.starts_with(href)
    };

    let active_class = "from-blue-100 to-blue-400 dark:from-blue-900 dark:to-blue-700";
    let dynamic_classes = {
        if is_active {
            ["bg-gradient-to-r saturate-150", active_class].join(" ")
        } else {
            ["hover:bg-gradient-to-r", active_class].join(" ")
        }
    };
    rhtml! { r#"
        <a href="{href}" class="text-blue-500 dark:text-blue-200 px-2 py-1 rounded-md {dynamic_classes}">{text}</a>
    "# }
}
