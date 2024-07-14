mod adaptor;
mod domain;

use adaptor::controllers::{note_handlers, user_handlers};
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", post(user_handlers::create_user))
        .route(
            "/users/:id",
            get(user_handlers::get_user)
                .put(user_handlers::update_user)
                .delete(user_handlers::delete_user),
        )
        .route("/users/:user_id/notes", get(note_handlers::list_note))
        .route("/notes", post(note_handlers::create_note))
        .route(
            "/notes/:id",
            get(note_handlers::get_note)
                .put(note_handlers::update_note)
                .delete(note_handlers::delete_note),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
