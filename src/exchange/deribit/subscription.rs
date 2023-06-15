use barter_integration::{Validator, error::SocketError};
use serde::{Deserialize, Serialize};

/// [`Deribit`](super::Deribit) WebSocket subscription response.
///
/// ### Raw Payload Examples
/// #### Subscription Trades Ok Response
/// ```json
/// {
///    "jsonrpc": "2.0",
///    "id": 3600,
///    "result": [
///      "deribit_price_index.btc_usd"
///    ]
///  }
///  
/// ```
///
/// #### Subscription Trades Error Response
/// ```json
/// {
///     "jsonrpc": "2.0",
///     "id": 8163,
///    "error": {
///         "code": 11050,
///         "message": "bad_request"
///     },
///     "testnet": false,
///     "usIn": 1535037392434763,
///     "usOut": 1535037392448119,
///     "usDiff": 13356
/// }
/// ```
///
/// See docs: <https://docs.deribit.com/#public-subscribe>
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DeribitSubResponse {
    pub jsonrpc: String,
    pub id: Option<i32>,
    pub result: Vec<String>,
    //TODO: add error object
}

impl Validator for DeribitSubResponse {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized,
    {

        //TODO: check for error variant

        match self.result.len() {
            0 => Err(SocketError::Subscribe(format!("received empty subscription response"))),
            _ => Ok(self),
        }
    }

}