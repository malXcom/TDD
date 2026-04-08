use axum_api::pricing::{calculate_delivery_fee, PricingError};

// ── normal cases ─────────────────────────────────────────────────────────────

#[test]
fn test_calculate_delivery_fee_should_return_base_fee_when_given_short_distance() {
    assert_eq!(calculate_delivery_fee(2.0, 1.0), Ok(2.00));
}

#[test]
fn test_calculate_delivery_fee_should_add_per_km_fee_when_given_distance_over_3km() {
    assert_eq!(calculate_delivery_fee(7.0, 3.0), Ok(4.00));
}

#[test]
fn test_calculate_delivery_fee_should_add_heavy_supplement_when_given_weight_over_5kg() {
    assert_eq!(calculate_delivery_fee(5.0, 8.0), Ok(4.50));
}

#[test]
fn test_calculate_delivery_fee_should_return_correct_total_when_given_6km_and_2kg() {
    assert_eq!(calculate_delivery_fee(6.0, 2.0), Ok(3.50));
}

// ── boundary cases ────────────────────────────────────────────────────────────

#[test]
fn test_calculate_delivery_fee_should_not_add_km_fee_when_given_exactly_3km() {
    assert_eq!(calculate_delivery_fee(3.0, 1.0), Ok(2.00));
}

#[test]
fn test_calculate_delivery_fee_should_accept_when_given_exactly_10km() {
    assert_eq!(calculate_delivery_fee(10.0, 1.0), Ok(5.50));
}

#[test]
fn test_calculate_delivery_fee_should_not_add_supplement_when_given_exactly_5kg() {
    assert_eq!(calculate_delivery_fee(2.0, 5.0), Ok(2.00));
}

#[test]
fn test_calculate_delivery_fee_should_return_correct_when_given_10km_and_6kg() {
    assert_eq!(calculate_delivery_fee(10.0, 6.0), Ok(7.00));
}

// ── error cases ───────────────────────────────────────────────────────────────

#[test]
fn test_calculate_delivery_fee_should_return_error_when_given_distance_over_10km() {
    assert_eq!(calculate_delivery_fee(15.0, 1.0), Err(PricingError::DistanceTooFar));
}

#[test]
fn test_calculate_delivery_fee_should_return_error_when_given_negative_distance() {
    assert_eq!(calculate_delivery_fee(-1.0, 1.0), Err(PricingError::NegativeDistance));
}

#[test]
fn test_calculate_delivery_fee_should_return_error_when_given_negative_weight() {
    assert_eq!(calculate_delivery_fee(2.0, -1.0), Err(PricingError::NegativeWeight));
}

#[test]
fn test_calculate_delivery_fee_should_return_base_fee_when_given_zero_distance() {
    assert_eq!(calculate_delivery_fee(0.0, 1.0), Ok(2.00));
}