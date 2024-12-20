use axum::{
    extract::{Form, Path},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rust_html::{rhtml, Template};

use super::utils::{router_fragment_stack, HxReq, RouteOptions};
use crate::components::button::ButtonColor;

pub async fn events_page(req: HxReq) -> impl IntoResponse {
    let template: Template = rhtml! { r#"
        <div class="space-y-4">
            <div class="flex items-center justify-between">
                <h1 class="text-3xl">Events</h1>
                <a href="/events/new" class="{ButtonColor::Info}">New Event</a>
            </div>
            <div class="overflow-hidden w-full overflow-x-auto rounded-md border border-neutral-300 dark:border-neutral-700">
                <table class="w-full text-left text-sm text-neutral-600 dark:text-neutral-300">
                    <thead class="border-b border-neutral-300 bg-neutral-50 text-sm text-neutral-900 dark:border-neutral-700 dark:bg-neutral-900 dark:text-white">
                        <tr>
                            <th scope="col" class="p-4">Name</th>
                            <th scope="col" class="p-4">Description</th>
                            <th scope="col" class="p-4">Attending</th>
                            <th scope="col" class="p-4"></th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-neutral-300 dark:divide-neutral-700">
                            <tr>
                                <td class="p-4">Event 1</td>
                                <td class="p-4">Description 1</td>
                                <td class="p-4">10 going, 5 invited</td>
                                <td>
                                    <a href="/events/edit/1" class="{ButtonColor::Primary}">Edit</a>
                                </td>
                            </tr>
                            <tr>
                                <td class="p-4">Event 2</td>
                                <td class="p-4">Description 2</td>
                                <td class="p-4">5 going, 2 invited</td>
                                <td>
                                    <a href="/events/edit/2" class="{ButtonColor::Primary}">Edit</a>
                                </td>
                            </tr>
                            <tr>
                                <td class="p-4">Event 3</td>
                                <td class="p-4">Description 3</td>
                                <td class="p-4">2 going, 0 invited</td>
                                <td>
                                    <a href="/events/edit/3" class="{ButtonColor::Primary}">Edit</a>
                                </td>
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
    let response_container = "response-container";
    let response_container_id = format!("#{}", response_container);

    let template: Template = rhtml! { r#"
        <div class="max-w-md space-y-2">
            <div id="{response_container}"></div>
            <form 
                hx-post="/api/events"
                hx-target={response_container_id}
                hx-swap="innerHTML"
                hx-on::after-request=" if(event.detail.successful) this.reset()"
                class="space-y-4"
                autocomplete="off"
            >
                <h1 class="text-3xl">New Event</h1>
                <div class="flex w-full max-w-xs flex-col gap-1 text-neutral-600 dark:text-neutral-300">
                    <label for="name" class="w-fit pl-0.5 text-sm">Name</label>
                    <input id="name" name="name" required placeholder="Enter an event name" type="text" class="w-full rounded-md border border-neutral-300 bg-neutral-50 px-2 py-2 text-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-black disabled:cursor-not-allowed disabled:opacity-75 dark:border-neutral-700 dark:bg-neutral-900/50 dark:focus-visible:outline-white" />
                </div>
                <div class="flex w-full max-w-md flex-col gap-1 text-neutral-600 dark:text-neutral-300">
                    <label for="description" class="w-fit pl-0.5 text-sm">Description</label>
                    <textarea id="description" name="description" required placeholder="Describe the event" class="w-full rounded-md border border-neutral-300 bg-neutral-50 px-2.5 py-2 text-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-black disabled:cursor-not-allowed disabled:opacity-75 dark:border-neutral-700 dark:bg-neutral-900/50 dark:focus-visible:outline-white" rows="3"></textarea>
                </div>
                <div class="flex w-full flex-col gap-1 text-neutral-600 dark:text-neutral-300">
                    <label for="location" class="w-fit pl-0.5 text-sm">Location</label>
                    <input id="location" required placeholder="Where is the event?" type="text" class="w-full rounded-md border border-neutral-300 bg-neutral-50 px-2 py-2 text-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-black disabled:cursor-not-allowed disabled:opacity-75 dark:border-neutral-700 dark:bg-neutral-900/50 dark:focus-visible:outline-white" name="location" />
                </div>
                <div class="flex items-center gap-4">
                    <div class="flex-1 flex flex-col gap-1 text-neutral-600 dark:text-neutral-300">
                        <label for="date" class="w-fit pl-0.5 text-sm">Date</label>
                        <input id="date" name="date" required type="date" class="w-full rounded-md border border-neutral-300 bg-neutral-50 px-2 py-2 text-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-black disabled:cursor-not-allowed disabled:opacity-75 dark:border-neutral-700 dark:bg-neutral-900/50 dark:focus-visible:outline-white" />
                    </div>
                    <div class="flex-1 flex flex-col gap-1 text-neutral-600 dark:text-neutral-300">
                        <label for="time" class="w-fit pl-0.5 text-sm">Time</label>
                        <input id="time" name="time" required type="time" class="w-full rounded-md border border-neutral-300 bg-neutral-50 px-2 py-2 text-sm focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-black disabled:cursor-not-allowed disabled:opacity-75 dark:border-neutral-700 dark:bg-neutral-900/50 dark:focus-visible:outline-white" />
                    </div>
                </div>
                <div class="flex justify-end gap-4">
                    <input
                        type="reset" 
                        value="Reset"
                        class="{ButtonColor::Secondary} justify-self-end" 
                    />
                    <button type="submit" class="{ButtonColor::Success}">Create</button>
                </div>
            </form>
        </div>
    "# };

    router_fragment_stack(
        req,
        RouteOptions {
            fragment: template,
            target: "/events/new",
        },
    )
}

#[derive(serde::Deserialize, Debug)]
pub struct EventRequest {
    name: String,
    description: String,
    location: String,
    time: NaiveTime,
    date: NaiveDate,
}

#[derive(serde::Serialize, Debug)]
pub struct Event {
    id: usize,
    name: String,
    description: String,
    location: String,
    time: NaiveDateTime,
}

impl Event {
    pub fn from_request(request: EventRequest) -> Self {
        Self {
            id: 4,
            name: request.name,
            description: request.description,
            location: request.location,
            time: NaiveDateTime::new(request.date, request.time),
        }
    }
}

pub async fn create_event_handler(Form(event): Form<EventRequest>) -> impl IntoResponse {
    let event = Event::from_request(event);
    dbg!(&event);

    let response = Response::builder()
        .status(StatusCode::CREATED)
        .header("Hx-Redirect", format!("/events/edit/{}", event.id))
        .body(serde_json::to_string(&event).unwrap())
        .unwrap();

    response
}

pub async fn edit_event_page(Path(id): Path<String>, req: HxReq) -> impl IntoResponse {
    let template: Template = rhtml! { r#"
        <h1 class="text-3xl">Edit Event {id}</h1>
    "# };

    router_fragment_stack(
        req,
        RouteOptions {
            fragment: template,
            target: "/events/edit/{id}",
        },
    )
}
