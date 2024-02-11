#[derive(serde::Deserialize)]
struct TxQuery {
    from_utc: Option<chrono::DateTime<chrono::Utc>>,
    to_utc: Option<chrono::DateTime<chrono::Utc>>,
    limit: Option<u32>,
}

async fn reject_unmapped_handler() -> impl actix_web::Responder {
    actix_web::HttpResponse::Forbidden().finish()
}

fn get_cors_config() -> actix_cors::Cors {
    actix_cors::Cors::default().allow_any_origin()
}

#[actix_web::get("/account/{account}")]
async fn get_by_account(
    state: actix_web::web::Data<crate::wrapper::MysqlClient>,
    account: actix_web::web::Path<String>,
    query: actix_web::web::Query<TxQuery>,
) -> actix_web::Result<actix_web::web::Json<alloc::vec::Vec<crate::wrapper::TransferHistory>>> {
    let account = <crate::wrapper::AccountId32 as core::str::FromStr>::from_str(&account)
        .map_err(|_| actix_web::error::ErrorBadRequest("Bad account!"))?;
    let results = state
        .get_by_account(account, query.from_utc, query.to_utc, query.limit)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Database compromised!"))?;

    Ok(actix_web::web::Json(results))
}

pub(super) async fn run(config: crate::config::Config) -> anyhow::Result<()> {
    let mysql_client = config.mysql().into_connection().await?;
    let shared_state = actix_web::web::Data::new(mysql_client);

    actix_web::HttpServer::new(move || {
        let cors = get_cors_config();
        let shared_state_clone = shared_state.clone();

        actix_web::App::new()
            .app_data(shared_state_clone)
            .wrap(cors)
            .wrap(tracing_actix_web::TracingLogger::default())
            .service(get_by_account)
            .default_service(actix_web::web::route().to(reject_unmapped_handler))
    })
    .bind("0.0.0.0:8686")?
    .run()
    .await?;

    Ok(())
}
