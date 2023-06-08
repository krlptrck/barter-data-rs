use crate::{
    event::{MarketEvent, MarketIter},
    exchange::{ExchangeId},
    subscription::trade::PublicTrade
};
use barter_integration::model::{instrument::Instrument, Exchange, Side};
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};

use super::notification::DeribitMessageMultiple;




/// Terse type alias for an [`Deribit`](super::Deribit) real-time trades WebSocket message.
pub type DeribitTrades = DeribitMessageMultiple<DeribitTrade>;

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

    // #[serde(
    //     rename = "timestamp",
    //     deserialize_with = "deserialize_timestamp"
    // )]
    // pub time: DateTime<Utc>,
}


impl From<(ExchangeId, Instrument, DeribitTrades)> for MarketIter<PublicTrade> {
    fn from((exchange_id, instrument, trades): (ExchangeId, Instrument, DeribitTrades)) -> Self {
        trades
            .params.data
            .into_iter()
            .map(|trade| {
                Ok(MarketEvent { 
                    exchange_time: Utc::now(), 
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

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp_ms: u64 = serde::Deserialize::deserialize(deserializer)?;
    let timestamp_sec = timestamp_ms / 1000;
    let timestamp_subsec = ((timestamp_ms % 1000) * 1_000_000) as u32;
    let timestamp_utc = Utc.timestamp(timestamp_sec as i64, timestamp_subsec);
    Ok(timestamp_utc)
}
