use serde::Serialize;

use crate::{Identifier, subscription::{Subscription, book::{OrderBooksL2}}};

use super::Aevo;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Aevo`](super::Aevo) channel to be subscribed to.
///
/// See docs: <https://docs.aevo.xyz/reference/publish-channel>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize)]
pub struct AevoChannel(pub &'static str);

impl AevoChannel {
    /// [`Aevo`](super::Aevo) OrderBook Level2 channel name (raw updates).
    ///
    /// See docs: <https://docs.aevo.xyz/reference/subscribe>
    pub const ORDER_BOOK_L2: Self = Self("orderbook");
}

impl Identifier<AevoChannel> for Subscription<Aevo, OrderBooksL2> {
    fn id(&self) -> AevoChannel {
        AevoChannel::ORDER_BOOK_L2
    }
}

impl AsRef<str> for AevoChannel {
    fn as_ref(&self) -> &str {
        self.0
    }
}