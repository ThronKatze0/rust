use axum::{routing::get, routing::post, Form, Router};
use maud::{html, Markup};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/add_todo", post(add_todo));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Markup {
    html! {
        h1 {"Hello World"}
        form hx-get="/add_todo" {
            input id="todo_name" placeholder="Todo name" {

            }
        }
        ul id="todo_list" {
            li {"test"}
        }
    }
}

struct AddTodoForm {
    name: String,
}

async fn add_todo(Form(form_data): Form<AddTodoForm>) -> Markup {
    html! {}
}
