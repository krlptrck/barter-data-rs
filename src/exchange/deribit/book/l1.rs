
use barter_integration::model::{instrument::Instrument, Exchange};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

use crate::{exchange::{ExchangeId, deribit::notification::DeribitMessageSingle}, event::{MarketIter, MarketEvent}, subscription::book::{OrderBookL1, Level, OrderBooksL1}};


pub type DeribitOrderBookL1 = DeribitMessageSingle<DeribitOrderBookL1Inner>;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct DeribitOrderBookL1Inner {
    // pub time: DateTime<Utc>,
    // pub instrument_name: String,
    pub best_bid_price: f64,
    pub best_bid_amount: f64,
    pub best_ask_price: f64,
    pub best_ask_amount: f64,
}


impl From<(ExchangeId, Instrument, DeribitOrderBookL1)> for MarketIter<OrderBookL1> {
    fn from((exchange_id, instrument, book): (ExchangeId, Instrument, DeribitOrderBookL1)) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: Utc::now(), // TODO change this to exchange time
            received_time: Utc::now(),
            exchange: Exchange::from(exchange_id),
            instrument,
            kind: OrderBookL1 {
                last_update_time: Utc::now(), // TODO change this to exchange time
                best_bid: Level::new(book.params.data.best_bid_price, book.params.data.best_bid_amount),
                best_ask: Level::new(book.params.data.best_ask_price, book.params.data.best_ask_amount),
            },
        })])
    }
}