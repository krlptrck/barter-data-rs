use crate::{
    event::{MarketEvent, MarketIter},
    exchange::{ExchangeId, ExchangeSub},
    subscription::trade::PublicTrade,
    Identifier,
};
use barter_integration::model::{instrument::Instrument, Exchange, Side, SubscriptionId};
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};

use super::channel::DeribitChannel;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DeribitMessageSingle<T> {
    pub jsonrpc: String,
    pub method: SubscriptionMethod,
    pub params: SingleData<T>
}

impl<T> Identifier<Option<SubscriptionId>> for DeribitMessageSingle<T> {
    fn id(&self) -> Option<SubscriptionId> {
        Some(self.params.subscription_id.clone())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DeribitMessageMultiple<T> {
    pub jsonrpc: String,
    pub method: SubscriptionMethod,
    pub params: MultipleData<T>
}

impl<T> Identifier<Option<SubscriptionId>> for DeribitMessageMultiple<T> {
    fn id(&self) -> Option<SubscriptionId> {
        Some(self.params.subscription_id.clone())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionMethod {
    Subscription,
    Heartbeat
}

impl SubscriptionMethod {
    pub fn is_subscription(self) -> bool { 
        match self {
            SubscriptionMethod::Subscription => true,
            SubscriptionMethod::Heartbeat => false
        }
    }
    
    pub fn is_heartbeat(self) -> bool {
        match self {
            SubscriptionMethod::Subscription => false,
            SubscriptionMethod::Heartbeat => true
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct MultipleData<T> {
    pub data: Vec<T>,
    #[serde(rename = "channel", deserialize_with = "de_deribit_message_channel_as_subscription_id")]
    pub subscription_id: SubscriptionId,
}


#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct SingleData<T> {
    pub data: T,
    #[serde(rename = "channel", deserialize_with = "de_deribit_message_channel_as_subscription_id")]
    pub subscription_id: SubscriptionId,
}

/// Deserialize an [`DeribitMessage`] "params/channel" field as a Barter [`SubscriptionId`].
fn de_deribit_message_channel_as_subscription_id<'de, D>(
    deserializer: D,
) -> Result<SubscriptionId, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    <&str as Deserialize>::deserialize(deserializer)
        .map(|market| {
            let instrument_name = match market.split('.').nth(1) {
                Some(instrument) => instrument,
                None => {
                    panic!("Invalid Deribit market: {}", market);
                }
            };
            ExchangeSub::from((DeribitChannel::ORDER_BOOK_L1, instrument_name)).id()
        })
}
