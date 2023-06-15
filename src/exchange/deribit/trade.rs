use crate::{
    event::{MarketEvent, MarketIter},
    exchange::{ExchangeId, subscription::ExchangeSub},
    subscription::trade::PublicTrade, Identifier
};
use barter_integration::model::{instrument::Instrument, Exchange, Side, SubscriptionId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{message::{DeribitMultipleDataMessage}, channel::DeribitChannel};

/// Terse type alias for an [`Deribit`](super::Deribit) real-time trades WebSocket message.
pub type DeribitTrades = DeribitMultipleDataMessage<DeribitTrade>;

// {
//     "params" : {
//       "data" : [
//         {
//           "trade_seq" : 30289442,
//           "trade_id" : "48079269",
//           "timestamp" : 1590484512188,
//           "tick_direction" : 2,
//           "price" : 8950,
//           "mark_price" : 8948.9,
//           "instrument_name" : "BTC-PERPETUAL",
//           "index_price" : 8955.88,
//           "direction" : "sell",
//           "amount" : 10
//         }
//       ],
//       "channel" : "trades.BTC-PERPETUAL.raw"
//     },
//     "method" : "subscription",
//     "jsonrpc" : "2.0"
//   }


/// [`Deribit`](super::Deribit) real-time trade WebSocket message.
///
/// See [`DeribitMessage`] for full raw payload examples.
///
/// See docs: <https://docs.deribit.com/#trades-instrument_name-interval>
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct DeribitTrade {
    #[serde(rename = "trade_id")]
    pub id: String,
    
    pub price: f64,
    
    pub amount: f64,

    #[serde(rename = "direction")]
    pub side: Side,

    pub instrument_name: String,

    #[serde(
        alias = "timestamp",
        deserialize_with = "barter_integration::de::de_u64_epoch_ms_as_datetime_utc"
    )]
    pub time: DateTime<Utc>,
}


impl Identifier<Option<SubscriptionId>> for DeribitTrades {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((DeribitChannel::TRADES_RAW, &self.params.data[0].instrument_name)).id())
    }
}


impl From<(ExchangeId, Instrument, DeribitTrades)> for MarketIter<PublicTrade> {
    fn from((exchange_id, instrument, trades): (ExchangeId, Instrument, DeribitTrades)) -> Self {
        trades
            .params.data
            .into_iter()
            .map(|trade| {
                Ok(MarketEvent { 
                    exchange_time: trade.time, 
                    received_time: Utc::now(), 
                    exchange: Exchange::from(exchange_id), 
                    instrument: instrument.clone(), 
                    kind: PublicTrade { 
                        id: trade.id, 
                        price: trade.price, 
                        amount: trade.amount, 
                        side: trade.side 
                    } 
                })
            })
            .collect()
    }
}