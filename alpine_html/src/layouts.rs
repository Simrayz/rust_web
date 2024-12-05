use rust_html::{rhtml, Template};

pub fn base_html(content: Template) -> Template {
    rhtml! { r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Alpine HTML</title>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <link href="/assets/main.css" rel="stylesheet" />
                <link href="https://rsms.me/inter/inter.css" rel="stylesheet" />
                <script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js"></script>
                <script src="https://unpkg.com/htmx.org@2.0.3" integrity="sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq" crossorigin="anonymous"></script>
            </head> 
            <body class="bg-gray-100 dark:bg-gray-900">
                {content}
            </body> 
        </html>
    "# }
}

pub fn main_layout(content: Template, active_page: &str) -> Template {
    let layout: Template = rhtml! { r#"
    <header class="flex items-center gap-8 container mx-auto py-4 px-4">
        <h1 class="text-3xl text-blue-500 font-bold underline">Alpine HTML</h1>
        <div class="flex items-center gap-2">
            {link_component("/", "Home", active_page)}
            {link_component("/users", "Users", active_page)}
            {link_component("/todos", "Todos", active_page)}
        </div>
    </header>
    <main class="container mx-auto text-gray-900 dark:text-gray-100 px-4">
        {content}
    </main>
    "# };

    base_html(layout)
}

pub fn link_component(href: &str, text: &str, active_path: &str) -> Template {
    let active_class = "from-blue-100 to-blue-400 dark:from-blue-900 dark:to-blue-700";
    let dynamic_classes = {
        if active_path == href {
            ["bg-gradient-to-r saturate-150", active_class].join(" ")
        } else {
            ["hover:bg-gradient-to-r", active_class].join(" ")
        }
    };
    rhtml! { r#"
        <a href="{href}" class="text-blue-500 dark:text-blue-200 px-2 py-1 rounded-md {dynamic_classes}">{text}</a>
    "# }
}
