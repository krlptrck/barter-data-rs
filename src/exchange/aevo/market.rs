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

use super::Aevo;

/// Type that defines how to translate a Barter [`Subscription`] into a
/// [`Aevo`](super::Aevo) market that can be subscribed to.
///
/// See docs: <https://docs.aevo.xyz/reference/subscribe>
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct AevoMarket(pub String);

impl <Kind> Identifier<AevoMarket> for Subscription<Aevo, Kind> {
    fn id(&self) -> AevoMarket {
        use InstrumentKind::*;

        let Instrument { base, quote, kind} = &self.instrument;
        
        AevoMarket(match kind {
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

impl AsRef<str> for AevoMarket {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Format the expiry DateTime<Utc> to be Aevo API compatible.
///
/// eg/ "21JUN23" (21th of June 2023)
///
/// See docs: <https://docs.aevo.xyz/reference/getinstrumentinstrumentname>
fn format_expiry<'a>(expiry: DateTime<Utc>) -> DelayedFormat<StrftimeItems<'a>> {
    expiry.date_naive().format("%-d%b%y")
}
