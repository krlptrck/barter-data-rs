use barter_data::{
    exchange::{ExchangeId, deribit::Deribit},
    streams::Streams,
    subscription::book::OrderBooksL2,
};
use barter_integration::model::instrument::kind::{InstrumentKind, OptionContract, OptionKind, OptionExercise};
use chrono::{Utc, TimeZone};
use tracing::info;

#[rustfmt::skip]
#[tokio::main]
async fn main() {
    // Initialise INFO Tracing log subscriber
    init_logging();

    // Initialise OrderBooksL2 Streams for Deribit only
    // '--> each call to StreamBuilder::subscribe() creates a separate WebSocket connection
    let mut streams = Streams::<OrderBooksL2>::builder()

        // Separate WebSocket connection for BTC_USDT stream since it's very high volume
        .subscribe([
            (Deribit::default(), "btc", "usdc", InstrumentKind::Perpetual, OrderBooksL2),
        ])
        .subscribe([
            (Deribit::default(), "btc", "usdc", InstrumentKind::Option(call_contract()), OrderBooksL2),
        ])

        .init()
        .await
        .unwrap();

    // Select the ExchangeId::Deribit stream
    // Notes:
    //  - Use `streams.select(ExchangeId)` to interact with the individual exchange streams!
    //  - Use `streams.join()` to join all exchange streams into a single mpsc::UnboundedReceiver!
    let mut binance_stream = streams
        .select(ExchangeId::Deribit)
        .unwrap();

    while let Some(order_book_l2) = binance_stream.recv().await {
        info!("{:?}", order_book_l2.kind.mid_price());
    }
}

// Initialise an INFO `Subscriber` for `Tracing` Json logs and install it as the global default.
fn init_logging() {
    tracing_subscriber::fmt()
        // Filter messages based on the INFO
        .with_env_filter(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // Disable colours on release builds
        .with_ansi(cfg!(debug_assertions))
        // Enable Json formatting
        .json()
        // Install this Tracing subscriber as global default
        .init()
}


fn call_contract() -> OptionContract {
    OptionContract {
        kind: OptionKind::Call,
        exercise: OptionExercise::American,
        expiry: Utc.timestamp_millis_opt(1688120754000).unwrap(),
        strike: rust_decimal_macros::dec!(25000),
    }
}