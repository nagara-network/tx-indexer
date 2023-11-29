use sqlx::{ConnectOptions, Connection};
use sqlx::{Executor, FromRow};

#[derive(Clone)]
pub(super) struct TransactionHistories {
    inner: sqlx::SqlitePool,
}

impl TransactionHistories {
    const FILENAME: &'static str = "transaction_histories.sqlite";
    const MAX_UNIXTIME: i64 = 4_102_444_800_000;
    const MIN_UNIXTIME: i64 = 1_672_531_200_000;
    const SQL_CREATE: &'static str = include_str!(
        "../../sql_scripts/transaction_histories/table_create.sql"
    );
    const SQL_INSERT_ROW: &'static str =
        include_str!("../../sql_scripts/transaction_histories/insert_row.sql");
    const SQL_SELECT_BY_ACCOUNT: &'static str = include_str!(
        "../../sql_scripts/transaction_histories/select_by_account.sql"
    );

    pub(super) async fn new() -> anyhow::Result<Self> {
        let db_path =
            format!("{}/{}", super::EntityConnector::DIR_DATA, Self::FILENAME);
        let connection_option = sqlx::sqlite::SqliteConnectOptions::new()
            .auto_vacuum(sqlx::sqlite::SqliteAutoVacuum::Incremental)
            .busy_timeout(tokio::time::Duration::from_secs(5))
            .create_if_missing(true)
            .filename(&db_path)
            .foreign_keys(false)
            .immutable(false)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Persist)
            .locking_mode(sqlx::sqlite::SqliteLockingMode::Normal)
            .log_slow_statements(
                log::LevelFilter::Warn,
                tokio::time::Duration::from_secs(3),
            )
            .log_statements(log::LevelFilter::Debug)
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
        let mut conn: sqlx::pool::PoolConnection<sqlx::Sqlite> =
            self.inner.acquire().await?;
        conn.execute(Self::SQL_CREATE).await?;

        Ok(())
    }

    pub(super) fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    pub(super) async fn close(&self) {
        self.inner.close().await;
    }

    pub(super) async fn insert_row(
        &self,
        new_transaction: Vec<super::NewTransaction>,
    ) -> anyhow::Result<()> {
        let mut conn = self.inner.acquire().await?;
        let mut batch_insert = conn.begin().await?;

        for row in new_transaction {
            let amount_bytes = row.amount.to_le_bytes().to_vec();
            let fee_bytes = row.fee.to_le_bytes().to_vec();
            let query = sqlx::query(Self::SQL_INSERT_ROW)
                .bind(row.hash)
                .bind(row.sender)
                .bind(row.receiver)
                .bind(amount_bytes)
                .bind(fee_bytes)
                .bind(row.blocknumber)
                .bind(row.unixtime);
            batch_insert.execute(query).await?;
        }

        batch_insert.commit().await?;

        Ok(())
    }

    pub(super) async fn get_by_account(
        &self,
        account: &str,
        from_inclusive: Option<chrono::DateTime<chrono::Utc>>,
        to_inclusive: Option<chrono::DateTime<chrono::Utc>>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<super::RelatedTransaction>> {
        let from_unixtime = if let Some(from_utc) = from_inclusive {
            from_utc.timestamp()
        } else {
            Self::MIN_UNIXTIME
        };
        let to_unixtime = if let Some(to_utc) = to_inclusive {
            to_utc.timestamp()
        } else {
            Self::MAX_UNIXTIME
        };
        let max_row = if let Some(row_limit) = limit {
            row_limit
        } else {
            u32::MAX - 1
        };

        let query = sqlx::query(Self::SQL_SELECT_BY_ACCOUNT)
            .bind(account)
            .bind(from_unixtime)
            .bind(to_unixtime)
            .bind(max_row);

        let mut conn = self.inner.acquire().await?;
        let result_db = conn.fetch_all(query).await?;
        let mut result = vec![];

        for row in result_db {
            let tx_data = super::RelatedTransaction::from_row(&row)?;

            result.push(tx_data);
        }

        Ok(result)
    }
}
