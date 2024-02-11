pub(super) fn init() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    let filter = tracing::Level::DEBUG;
    #[cfg(not(debug_assertions))]
    let filter = tracing::Level::INFO;
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .compact()
        .with_ansi(true)
        .with_level(true)
        .with_file(false)
        .with_line_number(true)
        .with_max_level(filter)
        .with_target(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
