use axum_api::order::{Item, OrderError, calculate_order_total};
use axum_api::pricing::PricingError;
use axum_api::promo::{PromoCode, PromoType};
use axum_api::surge::DayOfWeek;
use chrono::NaiveDate;

fn make_promo_codes() -> Vec<PromoCode> {
    vec![PromoCode {
        code: "PROMO20".to_string(),
        promo_type: PromoType::Percentage,
        value: 20.0,
        min_order: 0.00,
        expires_at: NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
    }]
}

fn make_items() -> Vec<Item> {
    vec![Item {
        name: "Pizza".to_string(),
        price: 12.50,
        quantity: 2,
    }]
}

// ── complete scenarios ────────────────────────────────────────────────────────

#[test]
fn test_calculate_order_total_should_return_correct_total_when_given_normal_order() {
    let result = calculate_order_total(
        &make_items(),
        2.0,
        1.0,
        None,
        &make_promo_codes(),
        15.0,
        DayOfWeek::Tuesday,
    );
    let order = result.unwrap();
    assert_eq!(order.subtotal, 25.00);
    assert_eq!(order.discount, 0.0);
    assert_eq!(order.surge, 1.0);
    assert_eq!(order.total, 27.00);
}

#[test]
fn test_calculate_order_total_should_apply_promo_when_given_valid_code() {
    let result = calculate_order_total(
        &make_items(),
        2.0,
        1.0,
        Some("PROMO20"),
        &make_promo_codes(),
        15.0,
        DayOfWeek::Tuesday,
    );
    let order = result.unwrap();
    assert_eq!(order.subtotal, 25.00);
    assert_eq!(order.total, 22.00);
}

#[test]
fn test_calculate_order_total_should_apply_surge_when_given_friday_evening() {
    let result = calculate_order_total(
        &make_items(),
        2.0,
        1.0,
        None,
        &make_promo_codes(),
        20.0,
        DayOfWeek::Friday,
    );
    let order = result.unwrap();
    assert_eq!(order.surge, 1.8);
    assert_eq!(order.delivery_fee, 3.60);
    assert_eq!(order.total, 28.60);
}

#[test]
fn test_calculate_order_total_should_return_correct_when_given_multiple_items() {
    let items = vec![
        Item {
            name: "Pizza".to_string(),
            price: 12.50,
            quantity: 2,
        },
        Item {
            name: "Drink".to_string(),
            price: 2.50,
            quantity: 3,
        },
    ];

    let result = calculate_order_total(
        &items,
        2.0,
        1.0,
        None,
        &make_promo_codes(),
        15.0,
        DayOfWeek::Monday,
    );
    let order = result.unwrap();
    assert_eq!(order.subtotal, 32.50);
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn test_calculate_order_total_should_return_error_when_given_empty_cart() {
    let result = calculate_order_total(
        &[],
        2.0,
        1.0,
        None,
        &make_promo_codes(),
        15.0,
        DayOfWeek::Tuesday,
    );
    assert_eq!(result, Err(OrderError::EmptyCart));
}

#[test]
fn test_calculate_order_total_should_return_error_when_given_negative_price() {
    let items = vec![Item {
        name: "Pizza".to_string(),
        price: -5.0,
        quantity: 1,
    }];
    let result = calculate_order_total(
        &items,
        2.0,
        1.0,
        None,
        &make_promo_codes(),
        15.0,
        DayOfWeek::Tuesday,
    );
    assert_eq!(result, Err(OrderError::NegativePrice));
}

#[test]
fn test_calculate_order_total_should_return_error_when_given_closed_hour() {
    let result = calculate_order_total(
        &make_items(),
        2.0,
        1.0,
        None,
        &make_promo_codes(),
        23.0,
        DayOfWeek::Tuesday,
    );
    assert_eq!(result, Err(OrderError::ClosedAtThisHour));
}

#[test]
fn test_calculate_order_total_should_return_error_when_given_distance_too_far() {
    let result = calculate_order_total(
        &make_items(),
        15.0,
        1.0,
        None,
        &make_promo_codes(),
        15.0,
        DayOfWeek::Tuesday,
    );
    assert_eq!(
        result,
        Err(OrderError::DeliveryError(PricingError::DistanceTooFar))
    );
}

#[test]
fn test_calculate_order_total_should_have_zero_discount_when_given_no_promo_code() {
    let result = calculate_order_total(
        &make_items(),
        2.0,
        1.0,
        None,
        &make_promo_codes(),
        15.0,
        DayOfWeek::Tuesday,
    );
    let order = result.unwrap();
    assert_eq!(order.discount, 0.0);
}

#[test]
fn test_calculate_order_total_should_apply_lunch_surge_when_given_weekday_noon() {
    let result = calculate_order_total(
        &make_items(),
        2.0,
        1.0,
        None,
        &make_promo_codes(),
        12.5,
        DayOfWeek::Wednesday,
    );
    let order = result.unwrap();
    assert_eq!(order.surge, 1.3);
    assert_eq!(order.delivery_fee, 2.60);
    assert_eq!(order.total, 27.60);
}
