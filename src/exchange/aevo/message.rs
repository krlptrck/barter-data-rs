use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct AevoMessage<T> {
    pub data: T,
    pub channel: Option<String>,
    pub id: Option<String>,
    pub error: Option<String>,
}