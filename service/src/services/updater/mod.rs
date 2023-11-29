mod transient_worker;

const GRACE_TIME_GFB: tokio::time::Duration =
    tokio::time::Duration::from_secs(1);
const GRACE_TIME_PROCESSOR: tokio::time::Duration =
    tokio::time::Duration::from_millis(100);
const GRACE_TIME_RECONNECT: tokio::time::Duration =
    tokio::time::Duration::from_secs(5);
const GRACE_TIME_CATCHUP: tokio::time::Duration =
    tokio::time::Duration::from_secs(10);

async fn create_rpc_client(
) -> anyhow::Result<subxt::OnlineClient<subxt::PolkadotConfig>> {
    let rpc_uri = crate::get_rpc_uri();

    let chain_api =
        subxt::OnlineClient::<subxt::PolkadotConfig>::from_url(&rpc_uri)
            .await?;

    anyhow::Result::Ok(chain_api)
}

async fn get_finalized_block_number(
    chain_api: &subxt::OnlineClient<subxt::PolkadotConfig>,
) -> anyhow::Result<(sp_core::H256, u32)> {
    let finalized_block_hash = chain_api.rpc().finalized_head().await;

    if finalized_block_hash.is_err() {
        let err = finalized_block_hash.err().unwrap();
        crate::logger::error!("{err}");

        return Err(err.into());
    }

    let finalized_block_hash = finalized_block_hash.unwrap();
    let finalized_block_number =
        chain_api.rpc().header(Some(finalized_block_hash)).await;

    if finalized_block_number.is_err() {
        let err = finalized_block_number.err().unwrap();
        crate::logger::error!("{err}");

        return Err(err.into());
    }

    let finalized_block_number = finalized_block_number.unwrap();

    if finalized_block_number.is_none() {
        crate::logger::error!("Chain's Network is unavailable...");

        return Err(anyhow::anyhow!("Cannot parse block number"));
    }

    Ok((finalized_block_hash, finalized_block_number.unwrap().number))
}

async fn get_block_hash(
    chain_api: &subxt::OnlineClient<subxt::PolkadotConfig>,
    block_number: u32,
) -> anyhow::Result<sp_core::H256> {
    let block_number = subxt::rpc::types::BlockNumber::from(block_number);
    let block_hash_result =
        chain_api.rpc().block_hash(Some(block_number)).await;

    if block_hash_result.is_err() {
        let err = block_hash_result.err().unwrap();
        crate::logger::error!("{err}");

        return Err(err.into());
    }

    let block_hash_maybe = block_hash_result.unwrap();

    if block_hash_maybe.is_none() {
        crate::logger::error!("Chain's Network is unavailable...");

        return Err(anyhow::anyhow!("Cannot parse block hash"));
    }

    Ok(block_hash_maybe.unwrap())
}

async fn get_block_data(
    chain_api: &subxt::OnlineClient<subxt::PolkadotConfig>,
    block_hash: sp_core::H256,
) -> anyhow::Result<
    subxt::blocks::Block<
        subxt::PolkadotConfig,
        subxt::OnlineClient<subxt::PolkadotConfig>,
    >,
> {
    let block_data_result = chain_api.blocks().at(block_hash).await;

    if block_data_result.is_err() {
        crate::logger::error!("Chain's Network is unavailable...");

        return Err(block_data_result.err().unwrap().into());
    }

    let block_data = block_data_result.unwrap();

    Ok(block_data)
}

async fn get_transactions(
    block_number: u32,
    block_hash: sp_core::H256,
    block: subxt::blocks::Block<
        subxt::PolkadotConfig,
        subxt::OnlineClient<subxt::PolkadotConfig>,
    >,
) -> anyhow::Result<Vec<crate::entities::NewTransaction>> {
    let block_hash = format!("0x{}", hex::encode(block_hash.0));
    let mut result = vec![];
    let mut block_timestamp = 0;

    let block_body_result = block.body().await;

    if block_body_result.is_err() {
        let err = block_body_result.err().unwrap();
        crate::logger::error!("{err}");

        return Err(err.into());
    }

    let extrinsics = block_body_result.unwrap().extrinsics().iter().flatten();

    for extrinsic in extrinsics {
        let extrinsic_index = extrinsic.index();
        let its_not_balances_activity =
            extrinsic.pallet_name()?.to_lowercase().ne("balances");
        let events_result = extrinsic.events().await;

        if events_result.is_err() {
            let err = events_result.err().unwrap();
            crate::logger::error!("{err}");

            return Err(err.into());
        }

        let events = events_result.unwrap();
        let event_count = events.iter().fold(0usize, |accumulator, event| {
            if event.is_ok() {
                accumulator + 1
            } else {
                accumulator
            }
        });

        if event_count > 1 {
            crate::logger::info!(
                "{block_number}-{extrinsic_index} contains {event_count} \
                 events"
            );
        }

        let root_extrinsic = extrinsic
            .as_root_extrinsic::<crate::metadata::nagara::api::Call>()?;

        if let crate::metadata::nagara::api::Call::Timestamp(
            crate::metadata::nagara::api::timestamp::Call::set {
                now,
            },
        ) = root_extrinsic
        {
            block_timestamp = now as i64;

            continue;
        }

        let mut balance_transfer_events = Vec::new();
        let mut gas_fee_events = Vec::new();

        events
            .find::<crate::metadata::nagara::api::balances::events::Transfer>()
            .for_each(|x| {
                if let Ok(balance_transfer_event) = x {
                    balance_transfer_events.push(balance_transfer_event);
                }
            });
        events
            .find::<crate::metadata::nagara::api::transaction_payment::events::TransactionFeePaid>()
            .for_each(|x| {
                if let Ok(gas_fee_event) = x {
                    gas_fee_events.push(gas_fee_event);
                }
            });

        if gas_fee_events.is_empty() {
            continue;
        }

        let (sender, mut sender_paid_fee) = {
            let caller = gas_fee_events.first().unwrap().who.to_owned();
            let mut total_gas_fee = 0;

            for gas_fee_event in gas_fee_events {
                total_gas_fee += gas_fee_event.actual_fee;
            }

            (caller, total_gas_fee)
        };

        if its_not_balances_activity {
            let new_transaction = crate::entities::NewTransaction {
                hash: block_hash.clone(),
                sender: crate::metadata::to_nagara_ss58_string(sender),
                receiver: "TheVoid".to_owned(),
                amount: 0,
                fee: sender_paid_fee,
                blocknumber: block_number,
                unixtime: block_timestamp,
            };
            result.push(new_transaction);
            sender_paid_fee = 0;
        }

        for balance_transfer_event in balance_transfer_events {
            let new_transaction = crate::entities::NewTransaction {
                hash: block_hash.clone(),
                sender: crate::metadata::to_nagara_ss58_string(
                    balance_transfer_event.from,
                ),
                receiver: crate::metadata::to_nagara_ss58_string(
                    balance_transfer_event.to,
                ),
                amount: balance_transfer_event.amount,
                fee: sender_paid_fee,
                blocknumber: block_number,
                unixtime: block_timestamp,
            };
            result.push(new_transaction);
        }
    }

    if !result.is_empty() {
        crate::logger::info!(
            "[Block Number({block_number})] Got {} transactions",
            result.len()
        );
    }

    Ok(result)
}

pub(super) async fn run_updater(
    state: actix_web::web::Data<super::ServiceState>,
) -> anyhow::Result<()> {
    let rpc_uri = crate::get_rpc_uri();

    while state.continue_running() {
        tokio::time::sleep(GRACE_TIME_RECONNECT).await;

        crate::logger::info!("Starting up Chain API Client...");
        let chain_api =
            subxt::OnlineClient::<subxt::PolkadotConfig>::from_url(&rpc_uri)
                .await;

        if chain_api.is_err() {
            crate::logger::error!(
                "Error during API init: {}",
                chain_api.err().unwrap()
            );

            continue;
        }

        let chain_api = chain_api.unwrap();
        let block_query_result = get_finalized_block_number(&chain_api).await;

        if block_query_result.is_err() {
            crate::logger::error!(
                "Error during Block Query: {}",
                block_query_result.err().unwrap()
            );

            break;
        }

        let (_, finalized_block_number) = block_query_result.unwrap();

        loop {
            let next_block_number = state.get_next_unprocessed_block().await?;
            let block_delta =
                finalized_block_number.saturating_sub(next_block_number);

            if block_delta >= crate::WORKER_LIMIT {
                // catch-up phase: start
                crate::logger::info!("Catching up {block_delta} blocks");
                let (work_sender, work_receiver) =
                    crossbeam::channel::unbounded();
                let mut workers = vec![];

                for _ in 0..crate::WORKER_LIMIT {
                    let future = transient_worker::transient_block_processor(
                        state.clone(),
                        work_receiver.clone(),
                    );
                    let worker_join_handle = tokio::spawn(future);
                    workers.push(worker_join_handle);
                }

                for block_number_to_process in
                    next_block_number..finalized_block_number
                {
                    let _ = work_sender.try_send(block_number_to_process);
                }

                loop {
                    tokio::time::sleep(GRACE_TIME_CATCHUP).await;
                    let indexed_blocks =
                        state.get_next_unprocessed_block().await?;
                    let catchup_progress =
                        block_delta - (finalized_block_number - indexed_blocks);
                    let catchup_progress_percent =
                        catchup_progress as f32 * 100.0 / block_delta as f32;
                    crate::logger::info!(
                        "Catching up progress: {catchup_progress} \
                         ({catchup_progress_percent:.2} %)"
                    );

                    if indexed_blocks == finalized_block_number {
                        break;
                    }
                }

                for worker in workers {
                    let _ = worker.await;
                }
                // catch-up phase: finished
            } else {
                break;
            }
        }

        while state.continue_running() {
            tokio::time::sleep(GRACE_TIME_GFB).await;
            let block_query_result =
                get_finalized_block_number(&chain_api).await;

            if block_query_result.is_err() {
                crate::logger::error!(
                    "Error during Block Query: {}",
                    block_query_result.err().unwrap()
                );

                break;
            }

            let (_, finalized_block_number) = block_query_result.unwrap();
            let next_block_number = state.get_next_unprocessed_block().await?;

            if next_block_number > finalized_block_number {
                continue;
            }

            let target_block_hash =
                get_block_hash(&chain_api, next_block_number).await;

            if target_block_hash.is_err() {
                crate::logger::error!(
                    "Error fetching BlockHash[{next_block_number}]: {}",
                    target_block_hash.err().unwrap()
                );

                break;
            }

            let target_block_hash = target_block_hash.unwrap();
            let block_data =
                get_block_data(&chain_api, target_block_hash).await;

            if block_data.is_err() {
                crate::logger::error!(
                    "Error fetching BlockData[{next_block_number}]: {}",
                    block_data.err().unwrap()
                );

                break;
            }

            let block_data = block_data.unwrap();
            let new_transactions = get_transactions(
                next_block_number,
                target_block_hash,
                block_data,
            )
            .await;

            if new_transactions.is_err() {
                crate::logger::error!(
                    "Error fetching TransactionData[{next_block_number}]: {}",
                    new_transactions.err().unwrap()
                );

                break;
            }

            let new_transactions = new_transactions.unwrap();

            state.insert_new_transactions(new_transactions).await?;
            state.commit_processed_block(next_block_number).await?;
        }
    }

    Ok(())
}
