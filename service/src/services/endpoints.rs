#[actix_web::get("/")]
async fn hello() -> impl actix_web::Responder {
    "Hello!"
}

pub(super) async fn run_endpoints(
    state: actix_web::web::Data<super::ServiceState>,
) -> anyhow::Result<()> {
    let cloned_state = state.clone();
    let endpoint_socket = crate::get_socket_for_endpoint();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(state.clone())
            .wrap(actix_web::middleware::Logger::default())
            .service(hello)
    })
    .bind(endpoint_socket)?
    .run()
    .await?;

    cloned_state.shutdown().await;

    Ok(())
}
