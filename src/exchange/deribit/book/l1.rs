
use barter_integration::model::{instrument::Instrument, Exchange, SubscriptionId};
use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};

use crate::{exchange::{ExchangeId, deribit::{message::DeribitSingleDataMessage, channel::DeribitChannel}, subscription::ExchangeSub}, event::{MarketIter, MarketEvent}, subscription::book::{OrderBookL1, Level}, Identifier};


pub type DeribitOrderBookL1 = DeribitSingleDataMessage<DeribitOrderBookL1Inner>;

// {
//     "params" : {
//       "data" : {
//         "timestamp" : 1550658624149,
//         "instrument_name" : "BTC-PERPETUAL",
//         "best_bid_price" : 3914.97,
//         "best_bid_amount" : 40,
//         "best_ask_price" : 3996.61,
//         "best_ask_amount" : 50
//       },
//       "channel" : "quote.BTC-PERPETUAL"
//     },
//     "method" : "subscription",
//     "jsonrpc" : "2.0"
//   }


#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct DeribitOrderBookL1Inner {
    pub instrument_name: String,
    pub best_bid_price: f64,
    pub best_bid_amount: f64,
    pub best_ask_price: f64,
    pub best_ask_amount: f64,
    #[serde(
        alias = "timestamp",
        deserialize_with = "barter_integration::de::de_u64_epoch_ms_as_datetime_utc"
    )]
    pub time: DateTime<Utc>,
}

impl Identifier<Option<SubscriptionId>> for DeribitOrderBookL1 {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((DeribitChannel::ORDER_BOOK_L1, &self.params.data.instrument_name)).id())
    }
}


impl From<(ExchangeId, Instrument, DeribitOrderBookL1)> for MarketIter<OrderBookL1> {
    fn from((exchange_id, instrument, book): (ExchangeId, Instrument, DeribitOrderBookL1)) -> Self {
        Self(vec![Ok(MarketEvent {
            exchange_time: book.params.data.time,
            received_time: Utc::now(),
            exchange: Exchange::from(exchange_id),
            instrument,
            kind: OrderBookL1 {
                last_update_time: book.params.data.time, 
                best_bid: Level::new(book.params.data.best_bid_price, book.params.data.best_bid_amount),
                best_ask: Level::new(book.params.data.best_ask_price, book.params.data.best_ask_amount),
            },
        })])
    }
}