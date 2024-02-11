type BlockHash = subxt::ext::sp_core::H256;
type BTreeMap<K, V> = alloc::collections::BTreeMap<K, V>;
type CallBalance = crate::metadata::api::balances::Call;
type CallTimestamp = crate::metadata::api::timestamp::Call;
type EventExtrinsicSuccess = crate::metadata::api::system::events::ExtrinsicSuccess;
type EventFeePaid = crate::metadata::api::transaction_payment::events::TransactionFeePaid;
type EventTransfer = crate::metadata::api::balances::events::Transfer;
type PaysFee = crate::metadata::api::runtime_types::frame_support::dispatch::Pays;
type ProcessedBlocks = BTreeMap<u32, alloc::vec::Vec<crate::wrapper::BalanceTransferRow>>;
type RootExtrinsic = crate::metadata::api::Call;

struct ProcessedBlockUpdater {
    redis_client: crate::wrapper::RedisClient,
    chain_client: crate::wrapper::ChainClient,
    mysql_client: crate::wrapper::MysqlClient,
}

impl ProcessedBlockUpdater {
    async fn create(config: crate::config::Config) -> anyhow::Result<Self> {
        let redis_client = config.redis().into_connection().await?;
        let chain_client = config.chain().into_connection().await?;
        let mysql_client = config.mysql().into_connection().await?;

        Ok(Self {
            redis_client,
            chain_client,
            mysql_client,
        })
    }

    async fn get_next_blocks(&mut self) -> anyhow::Result<alloc::collections::BTreeSet<u32>> {
        let next_blocks_start = self.redis_client.get_next_block_to_process().await?;
        let next_blocks_end = self.chain_client.get_latest_finalized_block().await?;
        let mut next_blocks = alloc::collections::BTreeSet::new();

        if next_blocks_start > next_blocks_end {
            return Ok(next_blocks);
        }

        if next_blocks_start == next_blocks_end {
            next_blocks.insert(next_blocks_end);

            return Ok(next_blocks);
        }

        let result = (next_blocks_start..=next_blocks_end).collect();

        Ok(result)
    }

    async fn insert_processed_blocks(
        &mut self,
        processed_blocks: ProcessedBlocks,
    ) -> anyhow::Result<()> {
        for (block_number, block_rows) in processed_blocks {
            if !block_rows.is_empty() {
                self.mysql_client.insert_rows(&block_rows).await?;
            }

            self.redis_client
                .commit_last_processed_block(block_number)
                .await?;
        }

        Ok(())
    }
}

#[derive(core::fmt::Debug)]
struct BalanceTransferExtrinsic {
    sequence: u32,
    event_transfer: Option<EventTransfer>,
    event_fee_paid: Option<EventFeePaid>,
    event_extrinsic_success: Option<EventExtrinsicSuccess>,
}

impl BalanceTransferExtrinsic {
    fn is_irrelevant(&self) -> bool {
        self.event_transfer.is_none()
            || self.event_fee_paid.is_none()
            || self.event_extrinsic_success.is_none()
    }

    fn try_into_row(
        self,
        hash: [u8; 32],
        blocknumber: u32,
        unixtime: i64,
    ) -> Option<crate::wrapper::BalanceTransferRow> {
        if self.is_irrelevant() {
            return None;
        }

        let sequence = self.sequence;
        let event_transfer = self.event_transfer.unwrap();
        let event_fee_paid = self.event_fee_paid.unwrap();
        let event_extrinsic_success = self.event_extrinsic_success.unwrap();
        let sender = event_transfer.from.0;
        let receiver = event_transfer.to.0;
        let amount = event_transfer.amount.to_le_bytes();
        let fee = {
            let fee_integer = event_fee_paid.actual_fee.saturating_add(event_fee_paid.tip);

            match event_extrinsic_success.dispatch_info.pays_fee {
                | PaysFee::Yes => fee_integer.to_le_bytes(),
                | PaysFee::No => [0; 16],
            }
        };

        Some(crate::wrapper::BalanceTransferRow {
            blocknumber,
            sequence,
            hash,
            sender,
            receiver,
            amount,
            fee,
            unixtime,
        })
    }
}

#[derive(core::fmt::Debug)]
struct ProcessedBlock {
    number: u32,
    hash: BlockHash,
    timestamp_call: CallTimestamp,
    balance_transfer_extrinsics: Vec<BalanceTransferExtrinsic>,
}

impl ProcessedBlock {
    fn into_balance_transfer_rows(self) -> Vec<crate::wrapper::BalanceTransferRow> {
        let hash = self.hash.0;
        let blocknumber = self.number;
        let unixtime = {
            let CallTimestamp::set {
                now,
            } = self.timestamp_call;

            now as i64
        };
        let mut result = vec![];

        for extrinsic in self.balance_transfer_extrinsics {
            if let Some(row) = extrinsic.try_into_row(hash, blocknumber, unixtime) {
                result.push(row);
            }
        }

        result
    }

    async fn from_block_number_online(
        chain_client: &crate::wrapper::ChainClient,
        number: u32,
    ) -> anyhow::Result<Self> {
        let mut timestamp_call = None;
        let mut balance_transfer_extrinsics = vec![];
        let (hash, block_data) = chain_client.get_block_data(number).await?;
        let extrinsics = block_data.extrinsics().await?.iter().flatten();

        for (sequence, extrinsic) in extrinsics.enumerate() {
            let root_extrinsic = extrinsic.as_root_extrinsic::<RootExtrinsic>()?;

            match root_extrinsic {
                | RootExtrinsic::Timestamp(inner) => timestamp_call = Some(inner),
                | RootExtrinsic::Balances(inner) => {
                    let should_process = matches!(
                        inner,
                        CallBalance::force_transfer { .. }
                            | CallBalance::transfer_all { .. }
                            | CallBalance::transfer_allow_death { .. }
                            | CallBalance::transfer_keep_alive { .. }
                            | CallBalance::transfer { .. }
                    );

                    if !should_process {
                        continue;
                    }

                    let events = extrinsic.events().await?;
                    let sequence = sequence as u32;
                    let event_transfer = events.find_first::<EventTransfer>()?;
                    let event_fee_paid = events.find_first::<EventFeePaid>()?;
                    let event_extrinsic_success = events.find_first::<EventExtrinsicSuccess>()?;

                    balance_transfer_extrinsics.push(BalanceTransferExtrinsic {
                        sequence,
                        event_transfer,
                        event_fee_paid,
                        event_extrinsic_success,
                    });
                },
                | _ => continue,
            }
        }

        if timestamp_call.is_none() {
            return Err(anyhow::anyhow!("Invalid block decoder!"));
        }

        let timestamp_call = timestamp_call.unwrap();

        Ok(Self {
            number,
            hash,
            timestamp_call,
            balance_transfer_extrinsics,
        })
    }
}

async fn job_processor(
    chain_config: crate::config::ChainConfig,
    receiver: async_channel::Receiver<u32>,
    sender: async_channel::Sender<ProcessedBlock>,
) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let chain_client = crate::exit_on_error(chain_config.clone().into_connection().await);

        loop {
            let block_number = match receiver.recv().await {
                | Err(_) => {
                    crate::warn!("Job Channel is no more, shutting down...");
                    break;
                },
                | Ok(new_block_to_process) => new_block_to_process,
            };
            let processed_block = {
                let result =
                    ProcessedBlock::from_block_number_online(&chain_client, block_number).await;

                if result.is_err() {
                    let err = result.unwrap_err();
                    crate::error!("{err}");

                    return;
                }

                result.unwrap()
            };
            let send_job_result = sender.send(processed_block).await;

            if send_job_result.is_err() {
                crate::warn!("Job Channel is no more, shutting down...");
                return;
            }
        }
    }
}

pub(super) async fn run(config: crate::config::Config) -> anyhow::Result<()> {
    let (job_sender, job_receiver) = async_channel::unbounded();
    let (processed_block_sender, processed_block_receiver) = async_channel::unbounded();

    for _ in 0..config.worker_count() {
        let chain_config = config.chain();
        let receiver = job_receiver.clone();
        let sender = processed_block_sender.clone();
        let _ = tokio::spawn(async move {
            job_processor(chain_config, receiver, sender).await;
        });
    }

    let mut inner_block_processor =
        crate::exit_on_error(ProcessedBlockUpdater::create(config).await);

    loop {
        let mut pending_inserts = ProcessedBlocks::new();
        let next_blocks = crate::exit_on_error(inner_block_processor.get_next_blocks().await);
        let target_length = next_blocks.len();

        if target_length == 0 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            continue;
        }

        for next_block in next_blocks {
            crate::exit_on_error(job_sender.send(next_block).await);
        }

        while pending_inserts.len() != target_length {
            let processed_block = crate::exit_on_error(processed_block_receiver.recv().await);
            let block_number = processed_block.number;
            let transactions = processed_block.into_balance_transfer_rows();

            if !transactions.is_empty() {
                crate::info!(
                    "Block {block_number} has {:#?} transactions",
                    transactions.len()
                );
            }

            pending_inserts.insert(block_number, transactions);
        }

        crate::exit_on_error(
            inner_block_processor
                .insert_processed_blocks(pending_inserts)
                .await,
        );
    }
}
