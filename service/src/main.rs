#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod entities;
pub mod logger;
pub mod metadata;

#[actix_web::get("/")]
async fn hello() -> impl actix_web::Responder {
    "Hello!"
}

async fn start_http_server(
    db_handle: actix_web::web::Data<entities::EntityConnector>,
) -> anyhow::Result<()> {
    let cloned_db_handle = db_handle.clone();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(db_handle.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(hello)
    })
    .bind("0.0.0.0:8765")?
    .run()
    .await?;

    cloned_db_handle.close().await;

    Ok(())
}

async fn db_loop(db_handle: actix_web::web::Data<entities::EntityConnector>) -> anyhow::Result<()> {
    let with_grace_time = tokio::time::Duration::from_secs(1);

    while db_handle.is_connected() {
        tokio::time::sleep(with_grace_time).await;

        db_handle
            .insert_new_transaction(entities::NewTransaction::create_random())
            .await?;

        logger::info!("DB KeepAlive: TICK");
    }

    logger::warn!("DB is closed, exiting...");

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_logger();

    let db_handle = actix_web::web::Data::new(entities::EntityConnector::new().await?);
    let http_server_handle = start_http_server(db_handle.clone());
    let db_updater_handle = db_loop(db_handle);

    let (http_result, db_updater_result) = futures::join!(http_server_handle, db_updater_handle);

    if http_result.is_err() {
        logger::error!("HTTP Error: {}", http_result.err().unwrap());
    }

    if db_updater_result.is_err() {
        logger::error!("DB Updater Error: {}", db_updater_result.err().unwrap());
    }

    Ok(())
}
