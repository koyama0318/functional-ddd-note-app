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

#[tokio::main]
async fn main() {
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
