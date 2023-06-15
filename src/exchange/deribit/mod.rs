use barter_integration::{error::SocketError, protocol::websocket::WsMessage};
use barter_macro::{DeExchange, SerExchange};
use url::Url;
use std::time::Duration;
use serde_json::json;

use crate::{
    exchange::{Connector, ExchangeId, ExchangeSub, PingInterval, StreamSelector},
    subscriber::{validator::WebSocketSubValidator, WebSocketSubscriber},
    subscription::{trade::PublicTrades, book::{OrderBooksL1, OrderBooksL2}},
    transformer::{stateless::StatelessTransformer, book::MultiBookTransformer},
    ExchangeWsStream,
};

use self::{channel::DeribitChannel, market::DeribitMarket, subscription::{DeribitSubResponse}, trade::DeribitTrades, book::{l1::DeribitOrderBookL1, l2::DeribitBookUpdater}};

pub mod channel;

pub mod market;

pub mod subscription;

pub mod message;

pub mod trade;

pub mod book;


/// [`Deribit`] server base url.
///
/// See docs: <https://docs.deribit.com/#json-rpc-over-websocket>
pub const BASE_URL_DERIBIT: &str = "wss://streams.deribit.com/ws/api/v2";
// pub const BASE_URL_DERIBIT: &str = "wss://www.deribit.com/ws/api/v2";

/// [`Deribit`] server [`PingInterval`] duration.
///
/// See docs: <https://docs.deribit.com/#public-set_heartbeat>
pub const PING_INTERVAL_DERIBIT: Duration = Duration::from_secs(29);

/// [`Deribit`] exchange.
///
/// See docs: <https://docs.deribit.com/#json-rpc-over-websocket>
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DeExchange, SerExchange,
)]
pub struct Deribit;

impl Connector for Deribit {
    const ID: ExchangeId = ExchangeId::Deribit;
    type Channel = DeribitChannel;
    type Market = DeribitMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = DeribitSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_DERIBIT).map_err(SocketError::UrlParse)
    }

    fn ping_interval() -> Option<super::PingInterval> {
        // Some(PingInterval {
        //     interval: tokio::time::interval(PING_INTERVAL_DERIBIT),
        //     ping: || WsMessage::text("ping"), // TODO: public/test
        // })
        None
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        let stream_names = exchange_subs
            .into_iter()
            .map(|sub| format!("{}", sub.channel.as_ref().replace("{}", sub.market.as_ref())))
            .collect::<Vec<String>>();

        vec![WsMessage::Text(
            json!({
                "jsonrpc": "2.0",
                "method": "public/subscribe",
                "params": {
                    "channels": stream_names
                }
            })
            .to_string(),
        )]
    }
}

impl StreamSelector<PublicTrades> for Deribit {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, PublicTrades, DeribitTrades>>;
}

impl StreamSelector<OrderBooksL1> for Deribit {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, OrderBooksL1, DeribitOrderBookL1>>;
}

impl StreamSelector<OrderBooksL2> for Deribit {
    type Stream =
        ExchangeWsStream<MultiBookTransformer<Self, OrderBooksL2, DeribitBookUpdater>>;
}

