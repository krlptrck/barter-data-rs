use barter_integration::{
    error::SocketError,
    model::{instrument::Instrument, Side, SubscriptionId},
    protocol::websocket::WsMessage,
};
use chrono::{Utc, DateTime};
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use crate::{subscription::book::OrderBookSide, exchange::{deribit::{message::DeribitSingleDataMessage, channel::DeribitChannel}, subscription::ExchangeSub}, Identifier};
use crate::{transformer::book::OrderBookUpdater, error::DataError, subscription::book::OrderBook};
use crate::transformer::book::InstrumentOrderBook;
use super::DeribitLevel;

/// Terse type alias for an [`Deribit`](super::Deribit) real-time trades WebSocket message.
pub type DeribitOrderBookL2 = DeribitSingleDataMessage<DeribitOrderBookL2Delta>;

#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct DeribitOrderBookL2Delta {
    pub instrument_name: String,
    pub change_id: Option<u64>,
    pub prev_change_id: Option<u64>,
    pub bids: Vec<DeribitLevel>,
    pub asks: Vec<DeribitLevel>
}

impl Identifier<Option<SubscriptionId>> for DeribitOrderBookL2 {
    fn id(&self) -> Option<SubscriptionId> {
        Some(ExchangeSub::from((DeribitChannel::ORDER_BOOK_L2, &self.params.data.instrument_name)).id())
    }
}

impl From<DeribitOrderBookL2Delta> for OrderBook {
    fn from(snapshot: DeribitOrderBookL2Delta) -> Self {
        Self {
            last_update_time: Utc::now(),
            bids: OrderBookSide::new(Side::Buy, snapshot.bids),
            asks: OrderBookSide::new(Side::Sell, snapshot.asks),
        }
    }
}


// The first notification will contain the whole book (bid and ask amounts for all prices).
// After that there will only be information about changes to individual price levels.

// The first notification will contain the amounts for all price levels (list of ['new', price, amount] tuples). 
// All following notifications will contain a list of tuples with action, price level and new amount ([action, price, amount]). 
// Action can be either new, change or delete.

// Each notification will contain a change_id field, and each message except for the first one will contain a field prev_change_id. 
// If prev_change_id is equal to the change_id of the previous message, this means that no messages have been missed.

// The amount for perpetual and futures is in USD units, for options it is in corresponding cryptocurrency contracts, e.g., BTC or ETH.


// Step 1: Update full snapshot from first notification



#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DeribitBookUpdater {
    pub updates_processed: u64,
    pub change_id: u64,
    pub prev_change_id: u64,
}

impl DeribitBookUpdater {
    pub fn new(change_id: u64) -> Self {
        Self {
            updates_processed: 0,
            prev_change_id: change_id,
            change_id,
        }
    }
}

#[async_trait]
impl OrderBookUpdater for DeribitBookUpdater{
    type OrderBook = OrderBook;
    type Update = DeribitOrderBookL2;

    async fn init<Exchange, Kind>(
        _: mpsc::UnboundedSender<WsMessage>,
        instrument: Instrument,
    ) -> Result<InstrumentOrderBook<Self>, DataError>
    where
        Exchange: Send,
        Kind: Send,
    {
        // No need to fetch snapshot separately, as the first notification will contain the whole book
        // Initialize empty OrderBook
        Ok(InstrumentOrderBook {
            instrument,
            updater: Self::new(0),
            book: OrderBook::from(DeribitOrderBookL2Delta {
                instrument_name: String::new(),
                change_id: Some(0),
                prev_change_id: Some(0),
                bids: Vec::new(),
                asks: Vec::new(),
            }),
        })
    }

    fn update(
        &mut self,
        book: &mut Self::OrderBook,
        update: Self::Update,
    ) -> Result<Option<Self::OrderBook>, DataError> {
        // BinanceSpot: How To Manage A Local OrderBook Correctly
        // See Self's Rust Docs for more information on each numbered step
        // See docs: <https://binance-docs.github.io/apidocs/spot/en/#how-to-manage-a-local-order-book-correctly>

        // 4. Drop any event where u is <= lastUpdateId in the snapshot:
        // if update.last_update_id <= self.last_update_id {
        //     return Ok(None);
        // }

        // if self.is_first_update() {
        //     // 5. The first processed event should have U <= lastUpdateId AND u >= lastUpdateId:
        //     self.validate_first_update(&update)?;
        // } else {
        //     // 6. Each new event's pu should be equal to the previous event's u:
        //     self.validate_next_update(&update)?;
        // }

        // // Update OrderBook metadata & Levels:
        // // 7. The data in each event is the absolute quantity for a price level.
        // // 8. If the quantity is 0, remove the price level.
        book.last_update_time = Utc::now();
        book.bids.upsert(update.params.data.bids);
        book.asks.upsert(update.params.data.asks);

        // // Update OrderBookUpdater metadata
        self.updates_processed += 1;
        self.prev_change_id = self.change_id;
        self.change_id = update.params.data.change_id.unwrap();

        Ok(Some(book.snapshot()))
    }
}