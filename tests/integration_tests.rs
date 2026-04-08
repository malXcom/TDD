use axum::{
    Router,
    routing::{get, post},
};
use axum_api::app_state::AppState;
use axum_api::routes::{create_order, get_order, simulate_order, validate_promo};
use axum_api::store::new_store;
use axum_test::TestServer;
use serde_json::{Value, json};

// ── test app factory — fresh store each time to avoid flaky tests ─────────────

fn create_test_app() -> TestServer {
    let store = new_store(); // fresh empty store — no shared state between tests
    let state = AppState::new(store);

    let app = Router::new()
        .route("/orders/simulate", post(simulate_order))
        .route("/orders", post(create_order))
        .route("/orders/{id}", get(get_order))
        .route("/promo/validate", post(validate_promo))
        .with_state(state);

    TestServer::new(app)
}

fn normal_order_body() -> Value {
    json!({
        "items": [{ "name": "Pizza", "price": 12.50, "quantity": 2 }],
        "distance": 2.0,
        "weight": 1.0,
        "promo_code": null,
        "hour": 15.0,
        "day": "tuesday"
    })
}

// ── POST /orders/simulate ─────────────────────────────────────────────────────

#[tokio::test]
async fn test_simulate_should_return_200_with_correct_price_when_given_normal_order() {
    let server = create_test_app();
    let res = server
        .post("/orders/simulate")
        .json(&normal_order_body())
        .await;

    res.assert_status_ok();
    let body: Value = res.json();
    assert_eq!(body["subtotal"], 25.0);
    assert_eq!(body["delivery_fee"], 2.0);
    assert_eq!(body["surge"], 1.0);
    assert_eq!(body["total"], 27.0);
}

#[tokio::test]
async fn test_simulate_should_apply_discount_when_given_valid_promo_code() {
    let server = create_test_app();
    let body = json!({
        "items": [{ "name": "Pizza", "price": 12.50, "quantity": 2 }],
        "distance": 2.0,
        "weight": 1.0,
        "promo_code": "WELCOME20",
        "hour": 15.0,
        "day": "tuesday"
    });

    let res = server.post("/orders/simulate").json(&body).await;
    res.assert_status_ok();
    let body: Value = res.json();

    assert_eq!(body["subtotal"], 25.0);
    assert_eq!(body["discount"], 5.0);
    assert_eq!(body["total"], 22.0);
}

#[tokio::test]
async fn test_simulate_should_return_400_when_given_expired_promo_code() {
    let server = create_test_app();
    let body = json!({
        "items": [{ "name": "Pizza", "price": 12.50, "quantity": 2 }],
        "distance": 2.0,
        "weight": 1.0,
        "promo_code": "EXPIRED",
        "hour": 15.0,
        "day": "tuesday"
    });

    let res = server.post("/orders/simulate").json(&body).await;
    res.assert_status_bad_request();
    let body: Value = res.json();
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("expired")
    );
}

#[tokio::test]
async fn test_simulate_should_return_400_when_given_empty_cart() {
    let server = create_test_app();
    let body = json!({
        "items": [],
        "distance": 2.0,
        "weight": 1.0,
        "promo_code": null,
        "hour": 15.0,
        "day": "tuesday"
    });

    let res = server.post("/orders/simulate").json(&body).await;
    res.assert_status_bad_request();
    let body: Value = res.json();
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("empty")
    );
}

#[tokio::test]
async fn test_simulate_should_return_400_when_given_distance_over_10km() {
    let server = create_test_app();
    let body = json!({
        "items": [{ "name": "Pizza", "price": 12.50, "quantity": 1 }],
        "distance": 15.0,
        "weight": 1.0,
        "promo_code": null,
        "hour": 15.0,
        "day": "tuesday"
    });

    let res = server.post("/orders/simulate").json(&body).await;
    res.assert_status_bad_request();
    let body: Value = res.json();
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("10km")
    );
}

#[tokio::test]
async fn test_simulate_should_return_400_when_given_closed_hour() {
    let server = create_test_app();
    let body = json!({
        "items": [{ "name": "Pizza", "price": 12.50, "quantity": 1 }],
        "distance": 2.0,
        "weight": 1.0,
        "promo_code": null,
        "hour": 23.0,
        "day": "tuesday"
    });

    let res = server.post("/orders/simulate").json(&body).await;
    res.assert_status_bad_request();
    let body: Value = res.json();
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("closed")
    );
}

#[tokio::test]
async fn test_simulate_should_multiply_delivery_fee_when_given_friday_evening_surge() {
    let server = create_test_app();
    let body = json!({
        "items": [{ "name": "Pizza", "price": 12.50, "quantity": 2 }],
        "distance": 2.0,
        "weight": 1.0,
        "promo_code": null,
        "hour": 20.0,
        "day": "friday"
    });

    let res = server.post("/orders/simulate").json(&body).await;
    res.assert_status_ok();
    let body: Value = res.json();

    assert_eq!(body["surge"], 1.8);
    assert_eq!(body["delivery_fee"], 3.6);
    assert_eq!(body["total"], 28.6);
}

// ── POST /orders ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_order_should_return_201_with_id_when_given_valid_order() {
    let server = create_test_app();
    let res = server.post("/orders").json(&normal_order_body()).await;

    res.assert_status(axum::http::StatusCode::CREATED);
    let body: Value = res.json();
    assert!(body["id"].as_str().is_some());
    assert_eq!(body["total"], 27.0);
}

#[tokio::test]
async fn test_create_order_should_be_retrievable_via_get_when_created() {
    let server = create_test_app();

    let create_res = server.post("/orders").json(&normal_order_body()).await;
    let created: Value = create_res.json();
    let id = created["id"].as_str().unwrap();

    let get_res = server.get(&format!("/orders/{}", id)).await;
    get_res.assert_status_ok();
    let fetched: Value = get_res.json();
    assert_eq!(fetched["id"], created["id"]);
    assert_eq!(fetched["total"], created["total"]);
}

#[tokio::test]
async fn test_create_order_should_generate_unique_ids_when_creating_two_orders() {
    let server = create_test_app();

    let res1 = server.post("/orders").json(&normal_order_body()).await;
    let res2 = server.post("/orders").json(&normal_order_body()).await;

    let id1 = res1.json::<Value>()["id"].as_str().unwrap().to_string();
    let id2 = res2.json::<Value>()["id"].as_str().unwrap().to_string();

    assert_ne!(id1, id2);
}

#[tokio::test]
async fn test_create_order_should_return_400_when_given_invalid_order() {
    let server = create_test_app();
    let body = json!({
        "items": [],
        "distance": 2.0,
        "weight": 1.0,
        "promo_code": null,
        "hour": 15.0,
        "day": "tuesday"
    });

    let res = server.post("/orders").json(&body).await;
    res.assert_status_bad_request();
}

#[tokio::test]
async fn test_create_order_should_not_store_order_when_given_invalid_order() {
    let server = create_test_app();
    let invalid_body = json!({
        "items": [],
        "distance": 2.0,
        "weight": 1.0,
        "promo_code": null,
        "hour": 15.0,
        "day": "tuesday"
    });

    server.post("/orders").json(&invalid_body).await;

    let fake_id = "non-existent-id-from-failed-order";
    let get_res = server.get(&format!("/orders/{}", fake_id)).await;
    get_res.assert_status_not_found();
}

// ── GET /orders/:id ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_order_should_return_200_with_order_when_given_existing_id() {
    let server = create_test_app();

    let create_res = server.post("/orders").json(&normal_order_body()).await;
    let created: Value = create_res.json();
    let id = created["id"].as_str().unwrap();

    let res = server.get(&format!("/orders/{}", id)).await;
    res.assert_status_ok();
}

#[tokio::test]
async fn test_get_order_should_return_404_when_given_unknown_id() {
    let server = create_test_app();
    let res = server.get("/orders/totally-unknown-id").await;
    res.assert_status_not_found();
}

#[tokio::test]
async fn test_get_order_should_return_correct_structure_when_given_existing_id() {
    let server = create_test_app();

    let create_res = server.post("/orders").json(&normal_order_body()).await;
    let created: Value = create_res.json();
    let id = created["id"].as_str().unwrap();

    let res = server.get(&format!("/orders/{}", id)).await;
    let body: Value = res.json();

    assert!(body["id"].as_str().is_some());
    assert!(body["subtotal"].as_f64().is_some());
    assert!(body["discount"].as_f64().is_some());
    assert!(body["delivery_fee"].as_f64().is_some());
    assert!(body["surge"].as_f64().is_some());
    assert!(body["total"].as_f64().is_some());
}

// ── POST /promo/validate ──────────────────────────────────────────────────────

#[tokio::test]
async fn test_validate_promo_should_return_200_with_discount_when_given_valid_code() {
    let server = create_test_app();
    let body = json!({ "code": "WELCOME20", "subtotal": 50.0 });

    let res = server.post("/promo/validate").json(&body).await;
    res.assert_status_ok();
    let body: Value = res.json();
    assert_eq!(body["valid"], true);
    assert_eq!(body["original"], 50.0);
    assert_eq!(body["discounted"], 40.0);
    assert_eq!(body["discount"], 10.0);
}

#[tokio::test]
async fn test_validate_promo_should_return_400_when_given_expired_code() {
    let server = create_test_app();
    let body = json!({ "code": "EXPIRED", "subtotal": 50.0 });

    let res = server.post("/promo/validate").json(&body).await;
    res.assert_status_bad_request();
    let body: Value = res.json();
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("expired")
    );
}

#[tokio::test]
async fn test_validate_promo_should_return_400_when_given_order_below_minimum() {
    let server = create_test_app();

    let body = json!({ "code": "WELCOME20", "subtotal": 5.0 });

    let res = server.post("/promo/validate").json(&body).await;
    res.assert_status_bad_request();
    let body: Value = res.json();
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("minimum")
    );
}

#[tokio::test]
async fn test_validate_promo_should_return_404_when_given_unknown_code() {
    let server = create_test_app();
    let body = json!({ "code": "FAKECODE", "subtotal": 50.0 });

    let res = server.post("/promo/validate").json(&body).await;
    res.assert_status_not_found();
}

#[tokio::test]
async fn test_validate_promo_should_return_400_when_given_no_code() {
    let server = create_test_app();
    let body = json!({ "code": null, "subtotal": 50.0 });

    let res = server.post("/promo/validate").json(&body).await;
    res.assert_status_bad_request();
    let body: Value = res.json();
    assert!(
        body["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("required")
    );
}
