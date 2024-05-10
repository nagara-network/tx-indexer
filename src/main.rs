extern crate alloc;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub use tracing::{debug, error, info, warn};

mod config;
mod endpoints;
mod logger;
mod metadata;
mod processor;
mod wrapper;

fn exit_on_error<T, E>(result: core::result::Result<T, E>) -> T
where
    E: core::fmt::Display, {
    match result {
        | Err(err) => {
            error!("{err}");

            std::process::exit(1)
        },
        | Ok(inner) => inner,
    }
}

#[macro_export]
macro_rules! try_with_print {
    ($expr:expr $(,)?) => {
        match $expr {
            | core::result::Result::Ok(val) => val,
            | core::result::Result::Err(err) => {
                $crate::error!("{err}");
                return core::result::Result::Err(err.into());
            },
        }
    };
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    subxt::ext::sp_core::crypto::set_default_ss58_version(
        ss58_registry::Ss58AddressFormatRegistry::NagaraAccount.into(),
    );
    logger::init()?;
    let config = config::Config::default();
    let processor_handle = processor::run(config.clone());
    let endpoints_handle = endpoints::run(config);
    let _ = tokio::select! {
        _ = processor_handle => (),
        _ = endpoints_handle => (),
    };

    Ok(())
}
