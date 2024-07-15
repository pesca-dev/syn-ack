use syn_ack::{api, services, use_db};

#[rocket::launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let db = use_db().await;
    println!("{db:?}");

    let r = rocket::build();

    let r = api::mount(r).await;

    services::mount(r).await
}
