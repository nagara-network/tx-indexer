pub(crate) type AccountId32 = subxt::ext::sp_runtime::AccountId32;
pub(crate) type AddressRaw = [u8; 32];
pub(crate) type BlockHash = subxt::ext::sp_core::H256;
pub(crate) type BlockHashRaw = [u8; 32];
pub(crate) type U128Raw = [u8; 16];

struct DisplayableBalance(u128);

impl core::fmt::Display for DisplayableBalance {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (divider, unit) = match self.0 {
            | 0..=999_999_999 => {
                let divider =
                    <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(1_000_000)
                        .unwrap();
                let unit = "m";

                (divider, unit)
            },
            | 1_000_000_000..=999_999_999_999 => {
                let divider =
                    <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(1_000_000_000)
                        .unwrap();
                let unit = "";

                (divider, unit)
            },
            | 1_000_000_000_000..=999_999_999_999_999 => {
                let divider = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(
                    1_000_000_000_000,
                )
                .unwrap();
                let unit = "k";

                (divider, unit)
            },
            | 1_000_000_000_000_000..=999_999_999_999_999_999 => {
                let divider = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(
                    1_000_000_000_000_000,
                )
                .unwrap();
                let unit = "M";

                (divider, unit)
            },
            | 1_000_000_000_000_000_000..=999_999_999_999_999_999_999 => {
                let divider = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(
                    1_000_000_000_000_000_000,
                )
                .unwrap();
                let unit = "B";

                (divider, unit)
            },
            | 1_000_000_000_000_000_000_000..=999_999_999_999_999_999_999_999 => {
                let divider = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(
                    1_000_000_000_000_000_000_000,
                )
                .unwrap();
                let unit = "T";

                (divider, unit)
            },
            | 1_000_000_000_000_000_000_000_000..=999_999_999_999_999_999_999_999_999 => {
                let divider = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(
                    1_000_000_000_000_000_000_000_000,
                )
                .unwrap();
                let unit = "P";

                (divider, unit)
            },
            | 1_000_000_000_000_000_000_000_000_000..=999_999_999_999_999_999_999_999_999_999 => {
                let divider = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(
                    1_000_000_000_000_000_000_000_000_000,
                )
                .unwrap();
                let unit = "E";

                (divider, unit)
            },
            | 1_000_000_000_000_000_000_000_000_000_000
                ..=999_999_999_999_999_999_999_999_999_999_999 => {
                let divider = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(
                    1_000_000_000_000_000_000_000_000_000_000,
                )
                .unwrap();
                let unit = "Y";

                (divider, unit)
            },
            | 1_000_000_000_000_000_000_000_000_000_000_000..=u128::MAX => {
                let divider = <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(
                    1_000_000_000_000_000_000_000_000_000_000_000,
                )
                .unwrap();
                let unit = "Z";

                (divider, unit)
            },
        };
        let value =
            <bigdecimal::BigDecimal as bigdecimal::FromPrimitive>::from_u128(self.0).unwrap();
        let value = <bigdecimal::BigDecimal as core::ops::Div>::div(value, divider)
            .with_scale_round(4, bigdecimal::RoundingMode::Down);

        write!(f, "{value} {unit}NGR")
    }
}

#[derive(Debug)]
pub(crate) struct BalanceTransferRow {
    pub(crate) blocknumber: u32,
    pub(crate) sequence: u32,
    pub(crate) hash: BlockHashRaw,
    pub(crate) sender: AddressRaw,
    pub(crate) receiver: AddressRaw,
    pub(crate) amount: U128Raw,
    pub(crate) fee: U128Raw,
    pub(crate) unixtime: i64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct TransferHistory {
    pub(crate) id: u64,
    pub(crate) hash: String,
    pub(crate) sender: String,
    pub(crate) receiver: String,
    pub(crate) amount: u128,
    pub(crate) amount_str: String,
    pub(crate) fee: u128,
    pub(crate) fee_str: String,
    pub(crate) total_amount_str: String,
    pub(crate) blocknumber: u32,
    pub(crate) unixtime: chrono::DateTime<chrono::Utc>,
}

impl sqlx::FromRow<'_, sqlx::mysql::MySqlRow> for TransferHistory {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        // rows get
        let id: u64 = sqlx::Row::try_get(row, "id")?;
        let hash: Vec<u8> = sqlx::Row::try_get(row, "hash")?;
        let sender: Vec<u8> = sqlx::Row::try_get(row, "sender")?;
        let receiver: Vec<u8> = sqlx::Row::try_get(row, "receiver")?;
        let amount: Vec<u8> = sqlx::Row::try_get(row, "amount")?;
        let fee: Vec<u8> = sqlx::Row::try_get(row, "fee")?;
        let blocknumber: u32 = sqlx::Row::try_get(row, "blocknumber")?;
        let unixtime: i64 = sqlx::Row::try_get(row, "unixtime")?;
        // array conversion
        let mut amount_array = U128Raw::default();
        let mut fee_array = U128Raw::default();
        let mut sender_array = AddressRaw::default();
        let mut receiver_array = AddressRaw::default();
        amount_array.copy_from_slice(&amount);
        fee_array.copy_from_slice(&fee);
        sender_array.copy_from_slice(&sender);
        receiver_array.copy_from_slice(&receiver);
        // u128 conversions
        let amount = u128::from_le_bytes(amount_array);
        let fee = u128::from_le_bytes(fee_array);
        let amount_str = DisplayableBalance(amount).to_string();
        let fee_str = DisplayableBalance(fee).to_string();
        let total_amount_str = DisplayableBalance(amount + fee).to_string();
        // account conversions
        let sender = AccountId32::new(sender_array).to_string();
        let receiver = AccountId32::new(receiver_array).to_string();
        // others
        let hash = hex::encode(hash);
        let hash = format!("0x{hash}");
        let unixtime = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(unixtime).unwrap();

        Ok(Self {
            id,
            hash,
            sender,
            receiver,
            amount,
            amount_str,
            fee,
            fee_str,
            total_amount_str,
            blocknumber,
            unixtime,
        })
    }
}

pub(crate) struct RedisClient {
    inner: redis::aio::ConnectionManager,
}

impl RedisClient {
    const FIELD: &'static str = "last_processed_block_number";
    const KEY: &'static str = "tx-indexer";

    pub(crate) async fn new(mut inner: redis::aio::ConnectionManager) -> anyhow::Result<Self> {
        redis::AsyncCommands::hset_nx(&mut inner, Self::KEY, Self::FIELD, 0).await?;

        Ok(Self {
            inner,
        })
    }

    pub(crate) async fn get_last_processed_block(&mut self) -> anyhow::Result<u32> {
        let last_processed_block =
            redis::AsyncCommands::hget(&mut self.inner, Self::KEY, Self::FIELD).await?;

        Ok(last_processed_block)
    }

    pub(crate) async fn get_next_block_to_process(&mut self) -> anyhow::Result<u32> {
        Ok(self.get_last_processed_block().await? + 1)
    }

    pub(crate) async fn commit_last_processed_block(
        &mut self,
        block_number: u32,
    ) -> anyhow::Result<()> {
        redis::AsyncCommands::hset(&mut self.inner, Self::KEY, Self::FIELD, block_number).await?;

        Ok(())
    }
}

pub(crate) struct MysqlClient {
    inner: sqlx::MySqlPool,
}

impl MysqlClient {
    const MAX_UNIXTIME: i64 = 4_102_444_800_000;
    const MIN_UNIXTIME: i64 = 1_672_531_200_000;
    const SQL_CREATE_TABLE: &'static str = include_str!("../sql/create_table_if_not_exists.sql");
    const SQL_INSERT_ROW: &'static str = include_str!("../sql/insert_into.sql");
    const SQL_SELECT_BY_ACCOUNT: &'static str = include_str!("../sql/select_by_account.sql");

    pub(crate) async fn new(inner: sqlx::MySqlPool) -> anyhow::Result<Self> {
        sqlx::Executor::execute(&inner, Self::SQL_CREATE_TABLE).await?;

        Ok(Self {
            inner,
        })
    }

    pub(crate) async fn insert_rows(&mut self, rows: &[BalanceTransferRow]) -> anyhow::Result<()> {
        let mut batch_insert = self.inner.begin().await?;

        for row in rows {
            let query = sqlx::query(Self::SQL_INSERT_ROW)
                .bind(row.blocknumber)
                .bind(row.sequence)
                .bind(&row.hash[..])
                .bind(&row.sender[..])
                .bind(&row.receiver[..])
                .bind(&row.amount[..])
                .bind(&row.fee[..])
                .bind(row.unixtime);
            sqlx::Executor::execute(core::ops::DerefMut::deref_mut(&mut batch_insert), query)
                .await?;
        }

        batch_insert.commit().await?;

        Ok(())
    }

    pub(crate) async fn get_by_account(
        &self,
        account: AccountId32,
        from_inclusive: Option<chrono::DateTime<chrono::Utc>>,
        to_inclusive: Option<chrono::DateTime<chrono::Utc>>,
        limit: Option<u32>,
    ) -> anyhow::Result<Vec<TransferHistory>> {
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
        let account: [u8; 32] = account.into();
        let query = sqlx::query(Self::SQL_SELECT_BY_ACCOUNT)
            .bind(&account[..])
            .bind(&account[..])
            .bind(from_unixtime)
            .bind(to_unixtime)
            .bind(max_row);
        let result_db = sqlx::Executor::fetch_all(&self.inner, query).await?;
        let mut result = alloc::vec![];

        for row in result_db {
            let tx_history =
                <TransferHistory as sqlx::FromRow<'_, sqlx::mysql::MySqlRow>>::from_row(&row)?;
            result.push(tx_history);
        }

        Ok(result)
    }
}

pub(crate) struct ChainClient {
    extrinsic_client: subxt::OnlineClient<subxt::PolkadotConfig>,
    rpc_client: subxt::backend::legacy::LegacyRpcMethods<subxt::PolkadotConfig>,
}

impl ChainClient {
    pub(crate) fn new(
        extrinsic_client: subxt::OnlineClient<subxt::PolkadotConfig>,
        rpc_client: subxt::backend::legacy::LegacyRpcMethods<subxt::PolkadotConfig>,
    ) -> Self {
        Self {
            extrinsic_client,
            rpc_client,
        }
    }

    pub(crate) async fn get_latest_finalized_block(&self) -> anyhow::Result<u32> {
        loop {
            let maybe_has_last_finalized_block = self
                .extrinsic_client
                .blocks()
                .subscribe_finalized()
                .await?
                .next()
                .await;

            match maybe_has_last_finalized_block {
                | None => continue,
                | Some(last_finalized_block) => return Ok(last_finalized_block?.number()),
            }
        }
    }

    pub(crate) async fn get_block_hash(&self, block_number: u32) -> anyhow::Result<BlockHash> {
        let block_hash = self
            .rpc_client
            .chain_get_block_hash(Some(block_number.into()))
            .await?
            .ok_or(anyhow::anyhow!("Block {block_number} not found!"))?;

        Ok(block_hash)
    }

    pub(crate) async fn get_block_data(
        &self,
        block_number: u32,
    ) -> anyhow::Result<(
        BlockHash,
        subxt::blocks::Block<subxt::PolkadotConfig, subxt::OnlineClient<subxt::PolkadotConfig>>,
    )> {
        let block_hash = self.get_block_hash(block_number).await?;
        let block_data = self.extrinsic_client.blocks().at(block_hash).await?;

        Ok((block_hash, block_data))
    }
}
