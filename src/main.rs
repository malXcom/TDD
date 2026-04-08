use axum::{
    Router,
    routing::{get, post},
};
use axum_api::app_state::AppState;
use axum_api::routes::{create_order, get_order, simulate_order, validate_promo};
use axum_api::store::new_store;

pub fn create_app() -> Router {
    let store = new_store();
    let state = AppState::new(store);

    Router::new()
        .route("/orders/simulate", post(simulate_order))
        .route("/orders", post(create_order))
        .route("/orders/{id}", get(get_order))
        .route("/promo/validate", post(validate_promo))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let app = create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
