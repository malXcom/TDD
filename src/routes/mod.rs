use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::order::{Item, OrderError, calculate_order_total};
use crate::pricing::PricingError;
use crate::promo::{PromoError, apply_promo_code};
use crate::store::StoredOrder;
use crate::surge::DayOfWeek;

// ── shared request body ───────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct OrderRequest {
    pub items: Vec<ItemDto>,
    pub distance: f64,
    pub weight: f64,
    pub promo_code: Option<String>,
    pub hour: f64,
    pub day: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemDto {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Debug, Serialize)]
pub struct OrderTotalResponse {
    pub subtotal: f64,
    pub discount: f64,
    pub delivery_fee: f64,
    pub surge: f64,
    pub total: f64,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

fn error(msg: &str) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: msg.to_string(),
    })
}

fn map_order_error(e: OrderError) -> (StatusCode, Json<ErrorResponse>) {
    let msg = match &e {
        OrderError::EmptyCart => "Cart is empty".to_string(),
        OrderError::NegativePrice => "Item price cannot be negative".to_string(),
        OrderError::ClosedAtThisHour => "Service is closed at this hour".to_string(),
        OrderError::DeliveryError(PricingError::DistanceTooFar) => {
            "Distance exceeds 10km limit".to_string()
        }
        OrderError::DeliveryError(PricingError::NegativeDistance) => {
            "Distance cannot be negative".to_string()
        }
        OrderError::DeliveryError(PricingError::NegativeWeight) => {
            "Weight cannot be negative".to_string()
        }
        OrderError::PromoError(PromoError::CodeExpired) => "Promo code has expired".to_string(),
        OrderError::PromoError(PromoError::OrderTooLow) => {
            "Order does not meet minimum amount for this promo".to_string()
        }
        OrderError::PromoError(PromoError::CodeNotFound) => "Promo code not found".to_string(),
        OrderError::PromoError(PromoError::NegativeSubtotal) => {
            "Subtotal cannot be negative".to_string()
        }
    };
    (StatusCode::BAD_REQUEST, error(&msg))
}

type ParseResult<'a> =
    Result<(Vec<Item>, Option<&'a str>, DayOfWeek), (StatusCode, Json<ErrorResponse>)>;

fn parse_request(req: &OrderRequest) -> ParseResult<'_> {
    let day = req
        .day
        .parse::<DayOfWeek>()
        .map_err(|_| (StatusCode::BAD_REQUEST, error("Invalid day of week")))?;

    let items: Vec<Item> = req
        .items
        .iter()
        .map(|i| Item {
            name: i.name.clone(),
            price: i.price,
            quantity: i.quantity,
        })
        .collect();

    let promo = req.promo_code.as_deref();

    Ok((items, promo, day))
}

// ── POST /orders/simulate ─────────────────────────────────────────────────────

pub async fn simulate_order(
    State(state): State<AppState>,
    Json(req): Json<OrderRequest>,
) -> impl IntoResponse {
    let (items, promo, day) = match parse_request(&req) {
        Ok(v) => v,
        Err(e) => return e.into_response(),
    };

    match calculate_order_total(
        &items,
        req.distance,
        req.weight,
        promo,
        &state.promo_codes,
        req.hour,
        day,
    ) {
        Ok(total) => (
            StatusCode::OK,
            Json(OrderTotalResponse {
                subtotal: total.subtotal,
                discount: total.discount,
                delivery_fee: total.delivery_fee,
                surge: total.surge,
                total: total.total,
            }),
        )
            .into_response(),
        Err(e) => map_order_error(e).into_response(),
    }
}

// ── POST /orders ──────────────────────────────────────────────────────────────

pub async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<OrderRequest>,
) -> impl IntoResponse {
    let (items, promo, day) = match parse_request(&req) {
        Ok(v) => v,
        Err(e) => return e.into_response(),
    };

    match calculate_order_total(
        &items,
        req.distance,
        req.weight,
        promo,
        &state.promo_codes,
        req.hour,
        day,
    ) {
        Ok(total) => {
            let stored = StoredOrder::from_total(total);
            let mut store = state.store.lock().unwrap();
            store.insert(stored.id.clone(), stored.clone());
            (StatusCode::CREATED, Json(stored)).into_response()
        }
        Err(e) => map_order_error(e).into_response(),
    }
}

// ── GET /orders/:id ───────────────────────────────────────────────────────────

pub async fn get_order(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let store = state.store.lock().unwrap();
    match store.get(&id) {
        Some(order) => (StatusCode::OK, Json(order.clone())).into_response(),
        None => (StatusCode::NOT_FOUND, error("Order not found")).into_response(),
    }
}

// ── POST /promo/validate ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct PromoValidateRequest {
    pub code: Option<String>,
    pub subtotal: f64,
}

#[derive(Debug, Serialize)]
pub struct PromoValidateResponse {
    pub valid: bool,
    pub original: f64,
    pub discounted: f64,
    pub discount: f64,
}

pub async fn validate_promo(
    State(state): State<AppState>,
    Json(req): Json<PromoValidateRequest>,
) -> impl IntoResponse {
    let code = match req.code.as_deref() {
        None | Some("") => {
            return (StatusCode::BAD_REQUEST, error("Promo code is required")).into_response();
        }
        Some(c) => c,
    };

    match apply_promo_code(req.subtotal, Some(code), &state.promo_codes) {
        Ok(discounted) => {
            let discount = ((req.subtotal - discounted) * 100.0).round() / 100.0;
            (
                StatusCode::OK,
                Json(PromoValidateResponse {
                    valid: true,
                    original: req.subtotal,
                    discounted,
                    discount,
                }),
            )
                .into_response()
        }
        Err(PromoError::CodeNotFound) => {
            (StatusCode::NOT_FOUND, error("Promo code not found")).into_response()
        }
        Err(PromoError::CodeExpired) => {
            (StatusCode::BAD_REQUEST, error("Promo code has expired")).into_response()
        }
        Err(PromoError::OrderTooLow) => (
            StatusCode::BAD_REQUEST,
            error("Order does not meet minimum amount for this promo"),
        )
            .into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, error(&format!("{:?}", e))).into_response(),
    }
}
