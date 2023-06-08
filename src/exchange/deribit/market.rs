use super::Deribit;
use crate::{Identifier, subscription::Subscription};
use barter_integration::model::instrument::{
    kind::{InstrumentKind, OptionKind},
    Instrument,
};
use chrono::{
    format::{DelayedFormat, StrftimeItems},
    DateTime, Utc,
};

use serde::{Deserialize, Serialize};

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Deribit`](super::Deribit) market that can be subscribed to.
///
/// See docs: <https://www.okx.com/docs-v5/en/#websocket-api-public-channel>
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct DeribitMarket(pub String);

impl <Kind> Identifier<DeribitMarket> for Subscription<Deribit, Kind> {
    fn id(&self) -> DeribitMarket {
        use InstrumentKind::*;

        let Instrument { base, quote, kind} = &self.instrument;
        
        DeribitMarket(match kind {
            Spot => format!("{base}_{quote}").to_uppercase(),
            Future(future) => {
                format!("{base}-{}", format_expiry(future.expiry)).to_uppercase()
            },
            Perpetual => format!("{base}-PERPETUAL").to_uppercase(),
            Option(option) => format!(
                "{base}-{}-{}-{}",
                format_expiry(option.expiry),
                option.strike,
                match option.kind {
                    OptionKind::Call => "C",
                    OptionKind::Put => "P",
                },
            )
            .to_uppercase(),
            
        })
    }
}


impl AsRef<str> for DeribitMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Format the expiry DateTime<Utc> to be Okx API compatible.
///
/// eg/ "230526" (26th of May 2023)
///
/// See docs: <https://www.okx.com/docs-v5/en/#rest-api-public-data-get-instruments>
fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.date_naive().format("%-d%b%y")
}
