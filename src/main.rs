use syn_ack::{api, services, use_db};
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

    let db = use_db().await;

    let r = rocket::build();

    let r = api::mount(r).await;

    services::mount(r).await
}
