pub use log::{error, info};

const ENVKEY_RUST_LOG: &str = "RUST_LOG";

pub fn init_logger() {
    if std::env::var(ENVKEY_RUST_LOG).is_err() {
        #[cfg(debug_assertions)]
        std::env::set_var(ENVKEY_RUST_LOG, "debug");
        #[cfg(not(debug_assertions))]
        std::env::set_var(ENVKEY_RUST_LOG, "info");
    }

    env_logger::builder()
        .default_format()
        .format_timestamp_nanos()
        .format_indent(None)
        .init();
}
