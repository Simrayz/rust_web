use axum::response::IntoResponse;
use rust_html::{rhtml, Template};

use super::utils::{router_fragment_stack, HxReq, RouteOptions};
use crate::components::button::ButtonColor;

pub async fn events_page(req: HxReq) -> impl IntoResponse {
    let header_cell_class = "text-left border border-slate-500 p-2";
    let row_class = "hover:bg-gray-200 hover:dark:bg-gray-800 cursor-pointer";
    let cell_class = "border border-slashed border-slate-700 dark:border-slate-500 p-2";

    let template: Template = rhtml! { r#"
        <div class="space-y-4">
            <div class="w-full flex justify-between items-center">
                <h1 class="text-2xl">Your Events</h1>
                <a href="/events/new" class="{ButtonColor::Info}">New Event</a>
            </div>
            <div class="border-slate-500 overflow-hidden">
                <table class="w-full border-collapse text-gray-900 dark:text-gray-100">
                <thead>
                        <tr class="bg-gray-300 dark:bg-gray-700">
                            <th class="{header_cell_class}">Name</th>
                            <th class="{header_cell_class}">Description</th>
                            <th class="{header_cell_class}">Attending</th>
                            <th class="{header_cell_class} w-12"></th>
                        </tr>
                </thead>
                <tbody>
                        <tr class="{row_class}">
                            <td class="{cell_class}">Event 1</td>
                            <td class="{cell_class}">Description 1</td>
                            <td class="{cell_class}">5</td>
                            <td class="{cell_class} w-12"><button class="{ButtonColor::Primary}">View</button></td>
                        </tr>
                        <tr class="{row_class}">
                            <td class="{cell_class}">Event 2</td>
                            <td class="{cell_class}">Description 2</td>
                            <td class="{cell_class}">10</td>
                            <td class="{cell_class} w-12"><button class="{ButtonColor::Primary}">View</button></td>
                        </tr>
                </tbody>
                </table>
            </div>
        </div>  
    "# };

    router_fragment_stack(
        req,
        RouteOptions {
            fragment: template,
            target: "/events",
        },
    )
}

pub async fn new_event_page(req: HxReq) -> impl IntoResponse {
    let template: Template = rhtml! { r#"
        <h1 class="text-3xl">New Event</h1>
    "# };

    router_fragment_stack(
        req,
        RouteOptions {
            fragment: template,
            target: "/events/new",
        },
    )
}
