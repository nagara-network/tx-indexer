use sqlx::Executor;

#[derive(Clone)]
pub(super) struct ProcessedBlocks {
    inner: sqlx::SqlitePool,
}

impl ProcessedBlocks {
    const FILENAME: &'static str = "processed_blocks.sqlite";
    const SQL_CREATE: &'static str =
        include_str!("../../sql_scripts/processed_blocks/table_create.sql");

    pub(super) async fn new() -> anyhow::Result<Self> {
        let db_path = format!("{}/{}", super::EntityConnector::DIR_DATA, Self::FILENAME);
        let connection_option = sqlx::sqlite::SqliteConnectOptions::new()
            .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental)
            .busy_timeout(tokio::time::Duration::from_secs(1))
            .create_if_missing(true)
            .filename(&db_path)
            .foreign_keys(false)
            .immutable(false)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Persist)
            .locking_mode(sqlx::sqlite::SqliteLockingMode::Normal)
            .optimize_on_close(true, None)
            .shared_cache(false)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Extra);
        let inner = sqlx::SqlitePool::connect_with(connection_option).await?;
        let instance = Self {
            inner,
        };
        instance.table_create_if_not_exist().await?;

        Ok(instance)
    }

    async fn table_create_if_not_exist(&self) -> anyhow::Result<()> {
        let mut conn: sqlx::pool::PoolConnection<sqlx::Sqlite> = self.inner.acquire().await?;
        conn.execute(Self::SQL_CREATE).await?;

        Ok(())
    }

    pub(super) fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    pub(super) async fn close(&self) {
        self.inner.close().await;
    }
}
