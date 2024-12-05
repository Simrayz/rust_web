use axum::response::IntoResponse;
use axum::{extract::State, Form};
use rust_html::{rhtml, Template, TemplateGroup};
use std::sync::Arc;

use crate::components::*;
use crate::main_layout;
use crate::AppState;

pub async fn todos_page(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let todos = state.todos.lock().unwrap();
    let todos = render_todos(todos.clone());

    let result: Template = rhtml! { r#"
    <div class="max-w-xl">
        <h1 class="text-3xl">Todos</h1>
        {todo_form()}
        <div id="todos">
        {TemplateGroup(todos)}
        </div>
    </div>
    "#
    };

    main_layout(result, "/todos")
}

fn todo_form() -> Template {
    let todo_form_id = "#todos";
    rhtml! { r#"
    <form
        hx-post="/api/todos"
        hx-target="{todo_form_id}"
        hx-swap="innerHTML"
        hx-on::after-request=" if(event.detail.successful) this.reset()"
        class="w-full"
    >
        <label for="todo" class="block text-sm font-medium leading-6 text-gray-900"
            >Todo</label
        >
        <div class="mt-2 inline-flex flex-row space-x-2 w-full">
            <input
                type="text"
                name="todo"
                id="todo"
                autocomplete="off"
                class="max-w-md flex-1 px-2 block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                placeholder="Write something to do..."
            />
            <button
                type="submit"
                class="{ButtonColor::Success}"
                @
            >
                Add
            </button>
            <button 
                type="button" 
                class="{ButtonColor::Secondary} justify-self-end" 
                hx-delete="/api/todos"
                hx-target="{todo_form_id}"
                hx-swap="innerHTML"
            >
                Clear
            </button>
        </div>
    </form>
    "# }
}

#[derive(serde::Deserialize, Debug)]
pub struct TodoRequest {
    todo: String,
}

pub async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    let mut lock = state.todos.lock().unwrap();

    lock.push(todo.todo);

    let todos = render_todos(lock.clone());

    rhtml! { r#"
        {TemplateGroup(todos)}
    "# }
}

pub async fn clear_todos(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    state.todos.lock().unwrap().clear();

    no_todos()
}

fn render_todos(todos: Vec<String>) -> Vec<Template> {
    if todos.is_empty() {
        return vec![no_todos()];
    }

    todos
        .iter()
        .map(|todo| {
            rhtml! { r#"
                <blockquote class="p-4 my-4 bg-blue-50 border-l-4 border-blue-300 dark:border-blue-600 dark:bg-blue-900">
                    {todo}
                </blockquote>
             "# }
        })
        .collect::<Vec<Template>>()
}

fn no_todos() -> Template {
    rhtml! { r#"
         <p class="p-4 my-4 bg-gray-50 border rounded-md border-gray-300 dark:border-gray-700 dark:bg-gray-800">
            No todos yet. Add one maybe?
        </p>
    "# }
}
