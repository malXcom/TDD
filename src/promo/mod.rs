use chrono::NaiveDate;

#[derive(Debug, PartialEq, Clone)]
pub enum PromoType {
    Percentage,
    Fixed,
}

#[derive(Debug, Clone)]
pub struct PromoCode {
    pub code: String,
    pub promo_type: PromoType,
    pub value: f64,
    pub min_order: f64,
    pub expires_at: NaiveDate,
}

#[derive(Debug, PartialEq)]
pub enum PromoError {
    CodeNotFound,
    CodeExpired,
    OrderTooLow,
    NegativeSubtotal,
}

pub fn apply_promo_code(
    subtotal: f64,
    promo_code: Option<&str>,
    promo_codes: &[PromoCode],
) -> Result<f64, PromoError> {
    if subtotal < 0.0 {
        return Err(PromoError::NegativeSubtotal);
    }

    let code = match promo_code {
        None | Some("") => return Ok(subtotal),
        Some(c) => c,
    };

    let promo = promo_codes
        .iter()
        .find(|p| p.code.eq_ignore_ascii_case(code))
        .ok_or(PromoError::CodeNotFound)?;

    let today = chrono::Local::now().date_naive();
    if promo.expires_at < today {
        return Err(PromoError::CodeExpired);
    }

    if subtotal < promo.min_order {
        return Err(PromoError::OrderTooLow);
    }

    let discounted = match promo.promo_type {
        PromoType::Percentage => subtotal * (1.0 - promo.value / 100.0),
        PromoType::Fixed => subtotal - promo.value,
    };

    let total = discounted.max(0.0);
    Ok((total * 100.0).round() / 100.0)
}
