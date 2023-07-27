#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub(crate) mod entities;
pub(crate) mod logger;
pub(crate) mod metadata;
pub(crate) mod services;

pub(crate) const ENVKEY_ENDPOINT_SOCKET: &str = "ENDPOINT_SOCKET";
pub(crate) const ENVKEY_RPC_URI: &str = "RPC_URI";

fn get_socket_for_endpoint() -> String {
    match std::env::var(ENVKEY_ENDPOINT_SOCKET) {
        Err(_) => "0.0.0.0:8765".to_owned(),
        Ok(var) => var,
    }
}

fn get_rpc_uri() -> String {
    match std::env::var(ENVKEY_RPC_URI) {
        Err(_) => "wss://main-00.goro.network:443".to_owned(),
        Ok(var) => var,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_logger();
    sp_core::crypto::set_default_ss58_version(
        ss58_registry::Ss58AddressFormatRegistry::GoroAccount.into(),
    );

    services::run_services().await
}
