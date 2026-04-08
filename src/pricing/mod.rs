const BASE_FEE: f64 = 2.00;
const FREE_DISTANCE_KM: f64 = 3.0;
const MAX_DISTANCE_KM: f64 = 10.0;
const FEE_PER_KM: f64 = 0.50;
const HEAVY_WEIGHT_KG: f64 = 5.0;
const HEAVY_WEIGHT_SUPPLEMENT: f64 = 1.50;

#[derive(Debug, PartialEq)]
pub enum PricingError {
    DistanceTooFar,
    NegativeDistance,
    NegativeWeight,
}

pub fn calculate_delivery_fee(distance: f64, weight: f64) -> Result<f64, PricingError> {
    if distance < 0.0 {
        return Err(PricingError::NegativeDistance);
    }
    if weight < 0.0 {
        return Err(PricingError::NegativeWeight);
    }
    if distance > MAX_DISTANCE_KM {
        return Err(PricingError::DistanceTooFar);
    }

    let mut fee = BASE_FEE;

    if distance > FREE_DISTANCE_KM {
        fee += (distance - FREE_DISTANCE_KM) * FEE_PER_KM;
    }

    if weight > HEAVY_WEIGHT_KG {
        fee += HEAVY_WEIGHT_SUPPLEMENT;
    }

    Ok((fee * 100.0).round() / 100.0)
}
