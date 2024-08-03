use syn_ack::start;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

fn init() {
    dotenv::dotenv().ok();

    let _filter =
        filter::Targets::new().with_target("syn_ack", tracing::metadata::LevelFilter::TRACE);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
                .compact()
                .with_ansi(false),
        )
        .with(filter::LevelFilter::DEBUG)
        .init();
}

#[rocket::launch]
async fn rocket() -> _ {
    init();

    start().await
}
