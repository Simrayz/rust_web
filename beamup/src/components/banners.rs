use rust_html::{rhtml, Template};

pub fn success_message(title: &str, message: &str) -> Template {
    rhtml! { r#"
        <div class="space-y-1 border-l-4 rounded-r-md bg-green-200 text-green-800 border-green-400 p-2 dark:bg-green-900 dark:text-green-100 dark:border-green-700">
            <p class="text-xs">{title}</p>
            <p class="font-bold">{message}</p>
        </div>
    "# }
}
