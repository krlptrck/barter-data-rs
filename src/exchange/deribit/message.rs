use serde::{Deserialize, Serialize};


// Separated into two message types: Single and Multiple data message types for easier deserialization.

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DeribitSingleDataMessage <T> {
    pub jsonrpc: String,
    pub method: SubscriptionMethod,
    pub params: SingleData<T>
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DeribitMultipleDataMessage <T> {
    pub jsonrpc: String,
    pub method: SubscriptionMethod,
    pub params: MultipleData<T>
}


#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct SingleData<T> {
    pub data: T,
    pub channel: String,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct MultipleData<T> {
    pub data: Vec<T>,
    pub channel: String,
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