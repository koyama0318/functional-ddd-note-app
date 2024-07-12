mod adaptor;
mod domain;

use adaptor::handlers::{
    create_note, create_user, delete_note, delete_user, get_note, get_user, list_notes, list_users,
    update_note, update_user,
};
use axum::{
    routing::{get, post},
    Router,
};
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Context {}

static CONTEXT: OnceCell<Context> = OnceCell::new();

impl Context {
    pub fn global() -> &'static Context {
        CONTEXT.get().expect("context not initialized")
    }

    pub fn initialize() {
        let context = Context {};
        CONTEXT.set(context).expect("Failed to set context");
    }
}

#[tokio::main]
async fn main() {
    Context::initialize();

    let app = Router::new()
        .route("/users", post(create_user).get(list_users))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/notes", post(create_note).get(list_notes))
        .route(
            "/notes/:id",
            get(get_note).put(update_note).delete(delete_note),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
