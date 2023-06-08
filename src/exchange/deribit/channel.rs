use serde::Serialize;

use crate::{Identifier, subscription::{Subscription, trade::PublicTrades, book::{OrderBookL1, OrderBooksL1, OrderBooksL2}}};

use super::Deribit;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Deribit`](super::Deribit) channel to be subscribed to.
///
/// See docs: <https://docs.deribit.com/#subscriptions>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct DeribitChannel(pub &'static str);

impl DeribitChannel {
    /// [`Deribit`] real-time trades channel.
    ///
    /// See docs: <https://docs.deribit.com/#trades-instrument_name-interval>
    pub const TRADES_RAW: Self = Self("trades.{}.raw");

    /// [`Deribit`](super::Deribit) real-time OrderBook Level1 (top of book) channel name.
    ///
    /// See docs:<https://docs.deribit.com/#quote-instrument_name>
    pub const ORDER_BOOK_L1: Self = Self("quote.{}");

    /// [`Deribit`](super::Deribit) OrderBook Level2 channel name (raw updates).
    ///
    /// See docs: <https://docs.deribit.com/#book-instrument_name-interval>
    pub const ORDER_BOOK_L2: Self = Self("book.{}.raw");
}

impl Identifier<DeribitChannel> for Subscription<Deribit, PublicTrades> {
    fn id(&self) -> DeribitChannel {
        DeribitChannel::TRADES_RAW
    }
}

impl Identifier<DeribitChannel> for Subscription<Deribit, OrderBooksL1> {
    fn id(&self) -> DeribitChannel {
        DeribitChannel::ORDER_BOOK_L1
    }
}

impl Identifier<DeribitChannel> for Subscription<Deribit, OrderBooksL2> {
    fn id(&self) -> DeribitChannel {
        DeribitChannel::ORDER_BOOK_L2
    }
}

impl AsRef<str> for DeribitChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}