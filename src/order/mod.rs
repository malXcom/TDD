use crate::pricing::{PricingError, calculate_delivery_fee};
use crate::promo::{PromoCode, PromoError, apply_promo_code};
use crate::surge::{DayOfWeek, calculate_surge};

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Debug, PartialEq)]
pub struct OrderTotal {
    pub subtotal: f64,
    pub discount: f64,
    pub delivery_fee: f64,
    pub surge: f64,
    pub total: f64,
}

#[derive(Debug, PartialEq)]
pub enum OrderError {
    EmptyCart,
    NegativePrice,
    ClosedAtThisHour,
    DeliveryError(PricingError),
    PromoError(PromoError),
}

impl From<PricingError> for OrderError {
    fn from(e: PricingError) -> Self {
        OrderError::DeliveryError(e)
    }
}

impl From<PromoError> for OrderError {
    fn from(e: PromoError) -> Self {
        OrderError::PromoError(e)
    }
}

pub fn calculate_order_total(
    items: &[Item],
    distance: f64,
    weight: f64,
    promo_code: Option<&str>,
    promo_codes: &[PromoCode],
    hour: f64,
    day: DayOfWeek,
) -> Result<OrderTotal, OrderError> {
    if items.is_empty() {
        return Err(OrderError::EmptyCart);
    }

    for item in items {
        if item.price < 0.0 {
            return Err(OrderError::NegativePrice);
        }
    }

    let surge = calculate_surge(hour, day);
    if surge == 0.0 {
        return Err(OrderError::ClosedAtThisHour);
    }

    let subtotal: f64 = items.iter().map(|i| i.price * i.quantity as f64).sum();
    let subtotal = (subtotal * 100.0).round() / 100.0;

    let discounted = apply_promo_code(subtotal, promo_code, promo_codes)?;
    let discount = ((subtotal - discounted) * 100.0).round() / 100.0;

    let delivery_fee = calculate_delivery_fee(distance, weight)?;
    let delivery_with_surge = (delivery_fee * surge * 100.0).round() / 100.0;

    let total = ((discounted + delivery_with_surge) * 100.0).round() / 100.0;

    Ok(OrderTotal {
        subtotal,
        discount,
        delivery_fee: delivery_with_surge,
        surge,
        total,
    })
}
