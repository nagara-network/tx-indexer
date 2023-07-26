mod endpoints;
mod updater;

struct ServiceState {
    db_handle: crate::entities::EntityConnector,
    shutdown_flag: core::sync::atomic::AtomicBool,
}

impl ServiceState {
    async fn get_thread_safe_instance() -> anyhow::Result<actix_web::web::Data<Self>> {
        let db_handle = crate::entities::EntityConnector::new().await?;
        let shutdown_flag = core::sync::atomic::AtomicBool::new(false);
        let instance = Self {
            db_handle,
            shutdown_flag,
        };

        Ok(actix_web::web::Data::new(instance))
    }

    async fn shutdown(&self) {
        self.shutdown_flag
            .store(true, core::sync::atomic::Ordering::Relaxed);
        self.db_handle.close().await;
    }

    fn continue_running(&self) -> bool {
        if self
            .shutdown_flag
            .load(core::sync::atomic::Ordering::Relaxed)
        {
            return false;
        }

        if !self.db_handle.is_connected() {
            return false;
        }

        true
    }
}

impl core::ops::Deref for ServiceState {
    type Target = crate::entities::EntityConnector;

    fn deref(&self) -> &Self::Target {
        &self.db_handle
    }
}

pub(super) async fn run_services() -> anyhow::Result<()> {
    let state_0 = ServiceState::get_thread_safe_instance().await?;
    let state_1 = state_0.clone();
    let endpoints_handle = endpoints::run_endpoints(state_0);
    let updater_handle = updater::run_updater(state_1);

    let (endpoints_result, updater_result) = futures::join!(endpoints_handle, updater_handle);

    if endpoints_result.is_err() {
        crate::logger::error!("Endpoints Error: {}", endpoints_result.err().unwrap());
    }

    if updater_result.is_err() {
        crate::logger::error!("Updater Error: {}", updater_result.err().unwrap());
    }

    Ok(())
}
