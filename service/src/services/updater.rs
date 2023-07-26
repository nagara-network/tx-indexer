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
    let finalized_block_number = chain_api.rpc().header(Some(finalized_block_hash)).await;

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
    let block_hash_result = chain_api.rpc().block_hash(Some(block_number)).await;

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
    subxt::blocks::Block<subxt::PolkadotConfig, subxt::OnlineClient<subxt::PolkadotConfig>>,
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
    block: subxt::blocks::Block<subxt::PolkadotConfig, subxt::OnlineClient<subxt::PolkadotConfig>>,
) -> anyhow::Result<Vec<crate::entities::NewTransaction>> {
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
        let events_result = extrinsic.events().await;

        if events_result.is_err() {
            let err = events_result.err().unwrap();
            crate::logger::error!("{err}");

            return Err(err.into());
        }

        let root_extrinsic = extrinsic.as_root_extrinsic::<crate::metadata::goro::api::Call>()?;

        if let crate::metadata::goro::api::Call::Timestamp(
            crate::metadata::goro::api::timestamp::Call::set {
                now,
            },
        ) = root_extrinsic
        {
            block_timestamp = now as i64;

            continue;
        }

        if let crate::metadata::goro::api::Call::Balances(_) = root_extrinsic {
            let events = events_result.unwrap();
            let call_hash = hex::encode(events.extrinsic_hash().as_bytes());
            let events = events.iter().flatten();
            let mut new_transaction = crate::entities::NewTransaction {
                hash: call_hash,
                sender: "minter".to_owned(),
                receiver: "burner".to_owned(),
                amount: 0,
                fee: 0,
                blocknumber: block_number,
                unixtime: block_timestamp,
            };

            for event in events {
                let event = event.as_root_event::<crate::metadata::goro::api::Event>()?;

                if let crate::metadata::goro::api::Event::Balances(balance_event) = event {
                    match balance_event {
                        crate::metadata::goro::api::runtime_types::pallet_balances::pallet::Event::Endowed { account, free_balance } => {
                            new_transaction.receiver = crate::metadata::to_goro_ss58_string(account);
                            new_transaction.amount = free_balance;
                        }
                        crate::metadata::goro::api::runtime_types::pallet_balances::pallet::Event::Transfer { from, to, amount } => {
                            new_transaction.sender = crate::metadata::to_goro_ss58_string(from);
                            new_transaction.receiver = crate::metadata::to_goro_ss58_string(to);
                            new_transaction.amount = amount;
                        }
                        crate::metadata::goro::api::runtime_types::pallet_balances::pallet::Event::Withdraw { who, amount } => {
                            new_transaction.sender = crate::metadata::to_goro_ss58_string(who);
                            new_transaction.fee = amount;
                        }
                        _ => (),
                    }
                }
            }

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
    let waiting_for_reconnect = tokio::time::Duration::from_secs(5);
    let waiting_for_finalization = tokio::time::Duration::from_secs(1);

    while state.continue_running() {
        tokio::time::sleep(waiting_for_reconnect).await;

        let chain_api = subxt::OnlineClient::<subxt::PolkadotConfig>::from_url(&rpc_uri).await?;

        while state.continue_running() {
            let block_query_result = get_finalized_block_number(&chain_api).await;

            if block_query_result.is_err() {
                break;
            }

            let (_, finalized_block_number) = block_query_result.unwrap();
            let next_block_number = state.get_next_unprocessed_block().await?;

            if next_block_number > finalized_block_number {
                tokio::time::sleep(waiting_for_finalization).await;

                continue;
            }

            let target_block_hash = get_block_hash(&chain_api, next_block_number).await;

            if target_block_hash.is_err() {
                break;
            }

            let target_block_hash = target_block_hash.unwrap();
            let block_data = get_block_data(&chain_api, target_block_hash).await;

            if block_data.is_err() {
                break;
            }

            let block_data = block_data.unwrap();
            let new_transactions = get_transactions(next_block_number, block_data).await;

            if new_transactions.is_err() {
                break;
            }

            let new_transactions = new_transactions.unwrap();

            state.insert_new_transactions(new_transactions).await?;
            state.commit_processed_block(next_block_number).await?;
        }
    }

    Ok(())
}
