use serde::{Deserialize, Serialize};

use crate::subscription::book::Level;

pub mod l2;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct AevoLevel{
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub price: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub amount: f64,
    #[serde(deserialize_with = "barter_integration::de::de_str")]
    pub iv: f64,
}

impl From<AevoLevel> for Level {
    fn from(level: AevoLevel) -> Self {
        Self {
            price: level.price,
            amount: level.amount,
        }
    }
}