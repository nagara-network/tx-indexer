use sqlx::{Executor, Row};

#[derive(Clone)]
pub(super) struct ProcessedBlocks {
    inner: sqlx::SqlitePool,
}

impl ProcessedBlocks {
    const FILENAME: &'static str = "processed_blocks.sqlite";
    const SQL_COMMIT: &'static str =
        include_str!("../../sql_scripts/processed_blocks/insert_row.sql");
    const SQL_CREATE: &'static str =
        include_str!("../../sql_scripts/processed_blocks/table_create.sql");
    const SQL_SELECT_NEXT: &'static str =
        include_str!("../../sql_scripts/processed_blocks/select_next_block.sql");

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

    pub(super) async fn select_next_block(&self) -> anyhow::Result<u32> {
        let query = sqlx::query(Self::SQL_SELECT_NEXT);

        let mut conn = self.inner.acquire().await?;

        match conn.fetch_one(query).await {
            Err(sqlx::Error::RowNotFound) => Ok(0),
            Err(err) => {
                crate::logger::error!("DB error because of bad program!");

                Err(err.into())
            }
            Ok(result_db) => {
                let next_block = result_db.try_get("next_block")?;

                Ok(next_block)
            }
        }
    }

    pub(super) async fn commit(&self, block_number: u32) -> anyhow::Result<()> {
        let current_timestamp = chrono::Utc::now().timestamp_millis();
        let query = sqlx::query(Self::SQL_COMMIT)
            .bind(block_number)
            .bind(current_timestamp);
        let mut conn = self.inner.acquire().await?;
        conn.execute(query).await?;

        Ok(())
    }
}
