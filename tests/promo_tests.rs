use axum_api::promo::{apply_promo_code, PromoCode, PromoError, PromoType};
use chrono::NaiveDate;

fn make_promo_codes() -> Vec<PromoCode> {
    vec![
        PromoCode {
            code: "WELCOME20".to_string(),
            promo_type: PromoType::Percentage,
            value: 20.0,
            min_order: 15.00,
            expires_at: NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
        },
        PromoCode {
            code: "SAVE5".to_string(),
            promo_type: PromoType::Fixed,
            value: 5.0,
            min_order: 10.00,
            expires_at: NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
        },
        PromoCode {
            code: "EXPIRED".to_string(),
            promo_type: PromoType::Fixed,
            value: 5.0,
            min_order: 0.00,
            expires_at: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        },
        PromoCode {
            code: "ALL100".to_string(),
            promo_type: PromoType::Percentage,
            value: 100.0,
            min_order: 0.00,
            expires_at: NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
        },
        PromoCode {
            code: "MINUS10".to_string(),
            promo_type: PromoType::Fixed,
            value: 10.0,
            min_order: 0.00,
            expires_at: NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
        },
    ]
}

// ── normal cases ──────────────────────────────────────────────────────────────

#[test]
fn test_apply_promo_code_should_apply_percentage_when_given_valid_percentage_code() {
    let codes = make_promo_codes();
    assert_eq!(apply_promo_code(50.0, Some("WELCOME20"), &codes), Ok(40.0));
}

#[test]
fn test_apply_promo_code_should_apply_fixed_discount_when_given_valid_fixed_code() {
    let codes = make_promo_codes();
    assert_eq!(apply_promo_code(30.0, Some("SAVE5"), &codes), Ok(25.0));
}

#[test]
fn test_apply_promo_code_should_return_subtotal_when_given_no_code() {
    let codes = make_promo_codes();
    assert_eq!(apply_promo_code(30.0, None, &codes), Ok(30.0));
}

#[test]
fn test_apply_promo_code_should_return_subtotal_when_given_empty_string() {
    let codes = make_promo_codes();
    assert_eq!(apply_promo_code(30.0, Some(""), &codes), Ok(30.0));
}

// ── refusal cases ─────────────────────────────────────────────────────────────

#[test]
fn test_apply_promo_code_should_return_error_when_given_expired_code() {
    let codes = make_promo_codes();
    assert_eq!(
        apply_promo_code(30.0, Some("EXPIRED"), &codes),
        Err(PromoError::CodeExpired)
    );
}

#[test]
fn test_apply_promo_code_should_return_error_when_given_order_below_min() {
    let codes = make_promo_codes();
    assert_eq!(
        apply_promo_code(5.0, Some("WELCOME20"), &codes),
        Err(PromoError::OrderTooLow)
    );
}

#[test]
fn test_apply_promo_code_should_return_error_when_given_unknown_code() {
    let codes = make_promo_codes();
    assert_eq!(
        apply_promo_code(30.0, Some("FAKECODE"), &codes),
        Err(PromoError::CodeNotFound)
    );
}

#[test]
fn test_apply_promo_code_should_return_error_when_given_negative_subtotal() {
    let codes = make_promo_codes();
    assert_eq!(
        apply_promo_code(-10.0, Some("SAVE5"), &codes),
        Err(PromoError::NegativeSubtotal)
    );
}

// ── boundary cases ────────────────────────────────────────────────────────────

#[test]
fn test_apply_promo_code_should_return_zero_when_fixed_discount_exceeds_subtotal() {
    let codes = make_promo_codes();
    // MINUS10 = 10€ fixed, subtotal = 5€ → should not go below 0
    assert_eq!(apply_promo_code(5.0, Some("MINUS10"), &codes), Ok(0.0));
}

#[test]
fn test_apply_promo_code_should_return_zero_when_given_100_percent_code() {
    let codes = make_promo_codes();
    assert_eq!(apply_promo_code(50.0, Some("ALL100"), &codes), Ok(0.0));
}

#[test]
fn test_apply_promo_code_should_return_zero_when_given_zero_subtotal_and_fixed_code() {
    let codes = make_promo_codes();
    assert_eq!(apply_promo_code(0.0, Some("MINUS10"), &codes), Ok(0.0));
}
