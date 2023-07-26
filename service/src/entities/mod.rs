mod processed_blocks;
mod transaction_histories;

use chrono::TimeZone;
use sqlx::Row;

#[derive(Clone)]
pub struct EntityConnector {
    conn_processed_blocks: processed_blocks::ProcessedBlocks,
    conn_transaction_histories: transaction_histories::TransactionHistories,
}

impl EntityConnector {
    pub const DIR_DATA: &'static str = "./data";

    pub async fn new() -> anyhow::Result<Self> {
        tokio::fs::create_dir_all(Self::DIR_DATA).await?;
        let conn_processed_blocks = processed_blocks::ProcessedBlocks::new().await?;
        let conn_transaction_histories = transaction_histories::TransactionHistories::new().await?;

        Ok(Self {
            conn_processed_blocks,
            conn_transaction_histories,
        })
    }

    pub fn is_connected(&self) -> bool {
        if self.conn_processed_blocks.is_closed() {
            return false;
        }

        if self.conn_transaction_histories.is_closed() {
            return false;
        }

        true
    }

    pub async fn close(&self) {
        self.conn_processed_blocks.close().await;
        self.conn_transaction_histories.close().await;
    }

    pub async fn insert_new_transactions(
        &self,
        new_transactions: Vec<NewTransaction>,
    ) -> anyhow::Result<()> {
        if new_transactions.is_empty() {
            return Ok(());
        }

        self.conn_transaction_histories
            .insert_row(new_transactions)
            .await?;

        Ok(())
    }

    pub async fn get_related_transactions(
        &self,
        actor: &str,
        from_inclusive: Option<chrono::DateTime<chrono::Utc>>,
        to_inclusive: Option<chrono::DateTime<chrono::Utc>>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<RelatedTransaction>> {
        self.conn_transaction_histories
            .get_related(actor, from_inclusive, to_inclusive, limit)
            .await
    }

    pub async fn get_next_unprocessed_block(&self) -> anyhow::Result<u32> {
        self.conn_processed_blocks.select_next_block().await
    }

    pub async fn commit_processed_block(&self, block_number: u32) -> anyhow::Result<()> {
        self.conn_processed_blocks.commit(block_number).await
    }
}

pub struct NewTransaction {
    pub hash: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u128,
    pub fee: u128,
    pub blocknumber: u32,
    pub unixtime: i64,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct RelatedTransaction {
    pub id: u32,
    pub hash: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u128,
    pub fee: u128,
    pub blocknumber: u32,
    pub unixtime: chrono::DateTime<chrono::Utc>,
}

impl RelatedTransaction {
    const LEN_U128: usize = u128::BITS as usize / 8;
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for RelatedTransaction {
    fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let id: u32 = row.try_get("id")?;
        let hash: String = row.try_get("hash")?;
        let sender: String = row.try_get("sender")?;
        let receiver: String = row.try_get("receiver")?;
        let amount_bytes: Vec<u8> = row.try_get("amount")?;
        let fee_bytes: Vec<u8> = row.try_get("fee")?;
        let blocknumber: u32 = row.try_get("at_block_time")?;
        let unixtime_ms: i64 = row.try_get("at_unix_time")?;
        let unixtime = chrono::Utc.timestamp_millis_opt(unixtime_ms).unwrap();

        if amount_bytes.len() != Self::LEN_U128 {
            crate::logger::error!("Bad `amount` at `id` => {}", id);
        }

        if fee_bytes.len() != Self::LEN_U128 {
            crate::logger::error!("Bad `fee` at `id` => {}", id);
        }

        let mut amount_bytes_array = [0u8; Self::LEN_U128];
        let mut fee_bytes_array = [0u8; Self::LEN_U128];
        amount_bytes_array.copy_from_slice(&amount_bytes[..]);
        fee_bytes_array.copy_from_slice(&fee_bytes[..]);
        let amount = u128::from_le_bytes(amount_bytes_array);
        let fee = u128::from_le_bytes(fee_bytes_array);

        Ok(Self {
            id,
            hash,
            sender,
            receiver,
            amount,
            fee,
            blocknumber,
            unixtime,
        })
    }
}
