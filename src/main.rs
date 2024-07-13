mod adaptor;
mod domain;

use adaptor::controllers::user_handlers;
use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/users",
            post(user_handlers::create_user).get(user_handlers::list_users),
        )
        .route(
            "/users/:id",
            get(user_handlers::get_user)
                .put(user_handlers::update_user)
                .delete(user_handlers::delete_user),
        );
    // .route("/notes", post(create_note).get(list_notes))
    // .route(
    //     "/notes/:id",
    //     get(get_note).put(update_note).delete(delete_note),
    // );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
