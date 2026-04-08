use crate::promo::{PromoCode, PromoType};
use crate::store::OrderStore;
use chrono::NaiveDate;

#[derive(Clone)]
pub struct AppState {
    pub store: OrderStore,
    pub promo_codes: Vec<PromoCode>,
}

impl AppState {
    pub fn new(store: OrderStore) -> Self {
        Self {
            store,
            promo_codes: default_promo_codes(),
        }
    }
}

fn default_promo_codes() -> Vec<PromoCode> {
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
    ]
}
