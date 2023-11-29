pub(super) async fn transient_block_processor(
    state: actix_web::web::Data<super::super::ServiceState>,
    work_receiver: crossbeam::channel::Receiver<u32>,
) -> anyhow::Result<()> {
    while state.continue_running() {
        tokio::time::sleep(super::GRACE_TIME_RECONNECT).await;
        crate::logger::info!("Starting up Chain API Client...");
        let chain_api = super::create_rpc_client().await;

        if chain_api.is_err() {
            crate::logger::error!(
                "Error during API init: {}",
                chain_api.err().unwrap()
            );

            continue;
        }

        let chain_api = chain_api.unwrap();

        while state.continue_running() {
            tokio::time::sleep(super::GRACE_TIME_PROCESSOR).await;
            let next_block_number = match work_receiver.try_recv() {
                Ok(next_work) => next_work,
                Err(_) => {
                    crate::logger::info!("Finished...");

                    return anyhow::Result::Ok(());
                }
            };

            let next_block_hash =
                super::get_block_hash(&chain_api, next_block_number).await;

            if next_block_hash.is_err() {
                let err = next_block_hash.unwrap_err();
                crate::logger::error!(
                    "Cannot get block hash with error: {err}"
                );

                break;
            }

            let next_block_hash = next_block_hash.unwrap();
            let next_block_data =
                super::get_block_data(&chain_api, next_block_hash).await;

            if next_block_data.is_err() {
                let err = next_block_data.err().unwrap();
                crate::logger::error!(
                    "Cannot get block data with error: {err}"
                );

                break;
            }

            let next_block_data = next_block_data.unwrap();
            let new_transactions = super::get_transactions(
                next_block_number,
                next_block_hash,
                next_block_data,
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
