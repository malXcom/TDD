use crate::order::OrderTotal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredOrder {
    pub id: String,
    pub subtotal: f64,
    pub discount: f64,
    pub delivery_fee: f64,
    pub surge: f64,
    pub total: f64,
}

impl StoredOrder {
    #[must_use]
    pub fn from_total(total: &OrderTotal) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            subtotal: total.subtotal,
            discount: total.discount,
            delivery_fee: total.delivery_fee,
            surge: total.surge,
            total: total.total,
        }
    }
}

pub type OrderStore = Arc<Mutex<HashMap<String, StoredOrder>>>;

#[must_use]
pub fn new_store() -> OrderStore {
    Arc::new(Mutex::new(HashMap::new()))
}
