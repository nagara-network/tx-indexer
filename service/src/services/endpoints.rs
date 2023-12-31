#[derive(serde::Deserialize)]
struct TxQuery {
    from_utc: Option<chrono::DateTime<chrono::Utc>>,
    to_utc: Option<chrono::DateTime<chrono::Utc>>,
    limit: Option<u32>,
}

async fn reject_unmapped_handler() -> impl actix_web::Responder {
    actix_web::HttpResponse::Forbidden().finish()
}

#[actix_web::get("/account/{account}")]
async fn get_by_actor(
    state: actix_web::web::Data<crate::services::ServiceState>,
    account: actix_web::web::Path<String>,
    query: actix_web::web::Query<TxQuery>,
) -> impl actix_web::Responder {
    let related_tx = state
        .get_transactions_by_account(
            &account,
            query.from_utc,
            query.to_utc,
            query.limit,
        )
        .await;

    if related_tx.is_err() {
        crate::logger::error!("{}", related_tx.err().unwrap());

        return actix_web::HttpResponse::Forbidden().finish();
    }

    let related_tx = related_tx.unwrap();
    let related_tx_json = serde_json::to_string_pretty(&related_tx).unwrap();

    actix_web::HttpResponse::Ok()
        .content_type("application/json")
        .body(related_tx_json)
}

fn get_cors_config() -> actix_cors::Cors {
    actix_cors::Cors::default().allowed_origin_fn(|origin, _| {
        origin.as_bytes().ends_with(b".goro.network")
            || origin.as_bytes().ends_with(b".krigan.network")
            || origin.as_bytes().ends_with(b".nagara.network")
    })
}

pub(super) async fn run_endpoints(
    state: actix_web::web::Data<super::ServiceState>,
) -> anyhow::Result<()> {
    let cloned_state = state.clone();
    let endpoint_socket = crate::get_socket_for_endpoint();

    actix_web::HttpServer::new(move || {
        let cors = get_cors_config();

        actix_web::App::new()
            .app_data(state.clone())
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .service(get_by_actor)
            .default_service(
                actix_web::web::route().to(reject_unmapped_handler),
            )
    })
    .bind(endpoint_socket)?
    .run()
    .await?;

    cloned_state.shutdown().await;

    Ok(())
}
