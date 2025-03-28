use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub order_id: String,
    pub order_type: String,  // "buy" or "sell"
    pub amount: f64,
    pub price: f64,
}

impl Order {
    pub fn new(order_id: &str, order_type: &str, amount: f64, price: f64) -> Self {
        Self {
            order_id: order_id.to_string(),
            order_type: order_type.to_string(),
            amount,
            price,
        }
    }
}
