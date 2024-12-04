/*
<div x-data="{ open: false }">
    <button @click="open = ! open">Toggle</button>

    <div x-show="open" @click.outside="open = false">Contents...</div>
</div>
*/

use axum::response::IntoResponse;
use rust_html::{rhtml, Template, TemplateGroup};

use crate::components::*;
use crate::main_layout;

pub async fn home_page() -> impl IntoResponse {
    let result: Template = rhtml! { r#"
        <div class="text-gray-900 dark:text-gray-100 space-y-6">
            <h1 class="text-3xl">Home</h1>
            <div x-data="{{ open: false }}" class="space-y-2">
                <button @click="open = ! open" type="button" class="{ButtonColor::Primary}">Reveal secrets</button>
                <div x-show="open" @click.outside="open = false" class="bg-gray-200 dark:bg-gray-800 rounded-md p-4">Contents...</div>
            </div>
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
