use barter_integration::{error::SocketError, protocol::websocket::WsMessage};
use barter_macro::{DeExchange, SerExchange};
use url::Url;
use std::time::Duration;
use serde_json::json;

use crate::{
    exchange::{Connector, ExchangeId, ExchangeSub, StreamSelector},
    subscriber::{validator::WebSocketSubValidator, WebSocketSubscriber},
    subscription::{book::{OrderBooksL2}},
    transformer::{book::MultiBookTransformer},
    ExchangeWsStream,
};

use self::{channel::AevoChannel, market::AevoMarket, subscription::AevoSubResponse, book::l2::AevoBookUpdater};

pub mod channel;

pub mod market;

pub mod subscription;

pub mod message;

pub mod book;


/// [`Aevo`] server base url.
///
/// See docs: <https://docs.aevo.xyz/reference/endpoints>
pub const BASE_URL_AEVO: &str = "wss://ws.aevo.xyz";

/// [`Aevo`] server [`PingInterval`] duration.
///
/// See docs: <https://docs.aevo.xyz/reference/overview-1>
pub const PING_INTERVAL_AEVO: Duration = Duration::from_secs(840);

/// [`Aevo`] exchange.
///
/// See docs: <https://docs.aevo.xyz/reference/overview>
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DeExchange, SerExchange,
)]
pub struct Aevo;

impl Connector for Aevo {
    const ID: ExchangeId = ExchangeId::Aevo;
    type Channel = AevoChannel;
    type Market = AevoMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = AevoSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_AEVO).map_err(SocketError::UrlParse)
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        let stream_names = exchange_subs
            .into_iter()
            .map(|sub| format!("{}:{}", sub.channel.as_ref(), sub.market.as_ref()))
            .collect::<Vec<String>>();

        vec![WsMessage::Text(
            json!({
                "op": "subscribe",
                "data": stream_names
            })
            .to_string(),
        )]
    }
}

impl StreamSelector<OrderBooksL2> for Aevo {
    type Stream =
        ExchangeWsStream<MultiBookTransformer<Self, OrderBooksL2, AevoBookUpdater>>;
}
