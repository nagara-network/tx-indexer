let blocknumber = subxt::rpc::types::BlockNumber::from(395_u32);
    let block_hash = api.rpc().block_hash(Some(blocknumber)).await?.unwrap();
    let block_data = api.blocks().at(block_hash).await?;
    let block_body = block_data.body().await?;
    let events = block_data.events().await?;

    crate::logger::info!("Block Hash: 0x{}", hex::encode(block_hash.to_fixed_bytes()));

    let block_body_vec: Vec<ExtrinsicDetails<PolkadotConfig, OnlineClient<PolkadotConfig>>> =
        block_body.extrinsics().iter().flatten().collect();

    for x in events.iter().flatten() {
        let event_index = x.index() as usize;
        let related_extrinsic = block_body_vec.get(event_index).unwrap();
        let call_hash = hex::encode(
            related_extrinsic
                .events()
                .await?
                .extrinsic_hash()
                .as_bytes(),
        );

        crate::logger::info!("Call Hash => 0x{call_hash}");

        if let goro::api::Event::Balances(balance_event) = x.as_root_event::<goro::api::Event>()? {
            match balance_event {
                BalanceEvent::BalanceSet {
                    who,
                    free,
                } => {
                    crate::logger::info!(
                        "BalanceSet => Who: {} | Amount: {free}",
                        to_goro_ss58(who)
                    );
                }
                BalanceEvent::Burned {
                    who,
                    amount,
                } => {
                    crate::logger::info!("Burned => Who: {} | Amount: {amount}", to_goro_ss58(who));
                }
                BalanceEvent::Deposit {
                    who,
                    amount,
                } => {
                    crate::logger::info!(
                        "Deposit => Who: {} | Amount: {amount}",
                        to_goro_ss58(who)
                    );
                }
                BalanceEvent::DustLost {
                    account,
                    amount,
                } => {
                    crate::logger::info!(
                        "DustLost => Who: {} | Amount: {amount}",
                        to_goro_ss58(account)
                    );
                }
                BalanceEvent::Endowed {
                    account,
                    free_balance,
                } => {
                    crate::logger::info!(
                        "Endowed => Who: {} | Amount: {free_balance}",
                        to_goro_ss58(account)
                    );
                }
                BalanceEvent::Frozen {
                    who,
                    amount,
                } => {
                    crate::logger::info!("Frozen => Who: {} | Amount: {amount}", to_goro_ss58(who));
                }
                BalanceEvent::Issued {
                    amount,
                } => {
                    crate::logger::info!("Issued => Amount: {amount}");
                }
                BalanceEvent::Locked {
                    who,
                    amount,
                } => {
                    crate::logger::info!("Locked => Who: {} | Amount: {amount}", to_goro_ss58(who));
                }
                BalanceEvent::Minted {
                    who,
                    amount,
                } => {
                    crate::logger::info!("Minted => Who: {} | Amount: {amount}", to_goro_ss58(who));
                }
                BalanceEvent::Rescinded {
                    amount,
                } => {
                    crate::logger::info!("Rescinded => Amount: {amount}");
                }
                BalanceEvent::ReserveRepatriated {
                    from,
                    to,
                    amount,
                    destination_status,
                } => {
                    crate::logger::info!(
                        "ReserveRepatriated => From: {} | To: {} | Amount: {amount} | Status: \
                         {destination_status:#?}",
                        to_goro_ss58(from),
                        to_goro_ss58(to)
                    );
                }
                BalanceEvent::Reserved {
                    who,
                    amount,
                } => {
                    crate::logger::info!(
                        "Reserved => Who: {} | Amount: {amount}",
                        to_goro_ss58(who)
                    );
                }
                BalanceEvent::Restored {
                    who,
                    amount,
                } => {
                    crate::logger::info!(
                        "Restored => Who: {} | Amount: {amount}",
                        to_goro_ss58(who)
                    );
                }
                BalanceEvent::Slashed {
                    who,
                    amount,
                } => {
                    crate::logger::info!(
                        "Slashed => Who: {} | Amount: {amount}",
                        to_goro_ss58(who)
                    );
                }
                BalanceEvent::Suspended {
                    who,
                    amount,
                } => {
                    crate::logger::info!(
                        "Slashed => Who: {} | Amount: {amount}",
                        to_goro_ss58(who)
                    );
                }
                BalanceEvent::Thawed {
                    who,
                    amount,
                } => {
                    crate::logger::info!("Thawed => Who: {} | Amount: {amount}", to_goro_ss58(who));
                }
                BalanceEvent::Transfer {
                    from,
                    to,
                    amount,
                } => {
                    crate::logger::info!(
                        "Minted => From: {} | To: {} | Amount: {amount}",
                        to_goro_ss58(from),
                        to_goro_ss58(to)
                    );
                }
                BalanceEvent::Unlocked {
                    who,
                    amount,
                } => {
                    crate::logger::info!(
                        "Unlocked => Who: {} | Amount: {amount}",
                        to_goro_ss58(who)
                    );
                }
                BalanceEvent::Unreserved {
                    who,
                    amount,
                } => {
                    crate::logger::info!(
                        "Unreserved => Who: {} | Amount: {amount}",
                        to_goro_ss58(who)
                    );
                }
                BalanceEvent::Upgraded {
                    who,
                } => {
                    crate::logger::info!("Upgraded => Who: {}", to_goro_ss58(who));
                }
                BalanceEvent::Withdraw {
                    who,
                    amount,
                } => {
                    crate::logger::info!(
                        "Withdraw => Who: {} | Amount: {amount}",
                        to_goro_ss58(who)
                    );
                }
            }
        }
    }
