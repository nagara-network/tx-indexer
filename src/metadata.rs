#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
pub mod api {
    #[allow(unused_imports)]
    mod root_mod {
        pub use super::*;
    }
    pub static PALLETS: [&str; 17usize] = [
        "System",
        "Timestamp",
        "Balances",
        "ValidatorSet",
        "Session",
        "Aura",
        "Grandpa",
        "TransactionPayment",
        "Sudo",
        "Utility",
        "RandomnessCollectiveFlip",
        "Contracts",
        "Assets",
        "Multisig",
        "Identity",
        "BigBrotherCouncil",
        "ServicerRegistry",
    ];
    pub static RUNTIME_APIS: [&str; 12usize] = [
        "Core",
        "Metadata",
        "BlockBuilder",
        "TaggedTransactionQueue",
        "OffchainWorkerApi",
        "AuraApi",
        "SessionKeys",
        "GrandpaApi",
        "AccountNonceApi",
        "TransactionPaymentApi",
        "TransactionPaymentCallApi",
        "ContractsApi",
    ];
    /// The error type returned when there is a runtime issue.
    pub type DispatchError = runtime_types::sp_runtime::DispatchError;
    /// The outer event enum.
    pub type Event = runtime_types::nagara_core_runtime::RuntimeEvent;
    /// The outer extrinsic enum.
    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
    /// The outer error enum representing the DispatchError's Module variant.
    pub type Error = runtime_types::nagara_core_runtime::RuntimeError;
    pub fn constants() -> ConstantsApi {
        ConstantsApi
    }
    pub fn storage() -> StorageApi {
        StorageApi
    }
    pub fn tx() -> TransactionApi {
        TransactionApi
    }
    pub fn apis() -> runtime_apis::RuntimeApi {
        runtime_apis::RuntimeApi
    }
    pub mod runtime_apis {
        use super::root_mod;
        use super::runtime_types;
        use ::subxt::ext::codec::Encode;
        pub struct RuntimeApi;
        impl RuntimeApi {
            pub fn core(&self) -> core::Core {
                core::Core
            }

            pub fn metadata(&self) -> metadata::Metadata {
                metadata::Metadata
            }

            pub fn block_builder(&self) -> block_builder::BlockBuilder {
                block_builder::BlockBuilder
            }

            pub fn tagged_transaction_queue(
                &self,
            ) -> tagged_transaction_queue::TaggedTransactionQueue {
                tagged_transaction_queue::TaggedTransactionQueue
            }

            pub fn offchain_worker_api(&self) -> offchain_worker_api::OffchainWorkerApi {
                offchain_worker_api::OffchainWorkerApi
            }

            pub fn aura_api(&self) -> aura_api::AuraApi {
                aura_api::AuraApi
            }

            pub fn session_keys(&self) -> session_keys::SessionKeys {
                session_keys::SessionKeys
            }

            pub fn grandpa_api(&self) -> grandpa_api::GrandpaApi {
                grandpa_api::GrandpaApi
            }

            pub fn account_nonce_api(&self) -> account_nonce_api::AccountNonceApi {
                account_nonce_api::AccountNonceApi
            }

            pub fn transaction_payment_api(
                &self,
            ) -> transaction_payment_api::TransactionPaymentApi {
                transaction_payment_api::TransactionPaymentApi
            }

            pub fn transaction_payment_call_api(
                &self,
            ) -> transaction_payment_call_api::TransactionPaymentCallApi {
                transaction_payment_call_api::TransactionPaymentCallApi
            }

            pub fn contracts_api(&self) -> contracts_api::ContractsApi {
                contracts_api::ContractsApi
            }
        }
        pub mod core {
            use super::root_mod;
            use super::runtime_types;
            /// The `Core` runtime api that every Substrate runtime needs to
            /// implement.
            pub struct Core;
            impl Core {
                /// Returns the version of the runtime.
                pub fn version(
                    &self,
                ) -> ::subxt::runtime_api::Payload<types::Version, types::version::output::Output>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "Core",
                        "version",
                        types::Version {},
                        [
                            76u8, 202u8, 17u8, 117u8, 189u8, 237u8, 239u8, 237u8, 151u8, 17u8,
                            125u8, 159u8, 218u8, 92u8, 57u8, 238u8, 64u8, 147u8, 40u8, 72u8, 157u8,
                            116u8, 37u8, 195u8, 156u8, 27u8, 123u8, 173u8, 178u8, 102u8, 136u8,
                            6u8,
                        ],
                    )
                }

                /// Execute the given block.
                pub fn execute_block(
                    &self,
                    block: types::execute_block::Block,
                ) -> ::subxt::runtime_api::Payload<
                    types::ExecuteBlock,
                    types::execute_block::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Core",
                        "execute_block",
                        types::ExecuteBlock {
                            block,
                        },
                        [
                            133u8, 135u8, 228u8, 65u8, 106u8, 27u8, 85u8, 158u8, 112u8, 254u8,
                            93u8, 26u8, 102u8, 201u8, 118u8, 216u8, 249u8, 247u8, 91u8, 74u8, 56u8,
                            208u8, 231u8, 115u8, 131u8, 29u8, 209u8, 6u8, 65u8, 57u8, 214u8, 125u8,
                        ],
                    )
                }

                /// Initialize a block with the given header.
                pub fn initialize_block(
                    &self,
                    header: types::initialize_block::Header,
                ) -> ::subxt::runtime_api::Payload<
                    types::InitializeBlock,
                    types::initialize_block::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Core",
                        "initialize_block",
                        types::InitializeBlock {
                            header,
                        },
                        [
                            146u8, 138u8, 72u8, 240u8, 63u8, 96u8, 110u8, 189u8, 77u8, 92u8, 96u8,
                            232u8, 41u8, 217u8, 105u8, 148u8, 83u8, 190u8, 152u8, 219u8, 19u8,
                            87u8, 163u8, 1u8, 232u8, 25u8, 221u8, 74u8, 224u8, 67u8, 223u8, 34u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod version {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_version::RuntimeVersion;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Version {}
                pub mod execute_block {
                    use super::runtime_types;
                    pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: nagara_core_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ();
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExecuteBlock {
                    pub block: execute_block::Block,
                }
                pub mod initialize_block {
                    use super::runtime_types;
                    pub type Header =
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ();
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InitializeBlock {
                    pub header: initialize_block::Header,
                }
            }
        }
        pub mod metadata {
            use super::root_mod;
            use super::runtime_types;
            /// The `Metadata` api trait that returns metadata for the runtime.
            pub struct Metadata;
            impl Metadata {
                /// Returns the metadata of a runtime.
                pub fn metadata(
                    &self,
                ) -> ::subxt::runtime_api::Payload<types::Metadata, types::metadata::output::Output>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "Metadata",
                        "metadata",
                        types::Metadata {},
                        [
                            231u8, 24u8, 67u8, 152u8, 23u8, 26u8, 188u8, 82u8, 229u8, 6u8, 185u8,
                            27u8, 175u8, 68u8, 83u8, 122u8, 69u8, 89u8, 185u8, 74u8, 248u8, 87u8,
                            217u8, 124u8, 193u8, 252u8, 199u8, 186u8, 196u8, 179u8, 179u8, 96u8,
                        ],
                    )
                }

                /// Returns the metadata at a given version.
                ///
                /// If the given `version` isn't supported, this will return
                /// `None`.
                /// Use [`Self::metadata_versions`] to find out about supported
                /// metadata version of the runtime.
                pub fn metadata_at_version(
                    &self,
                    version: types::metadata_at_version::Version,
                ) -> ::subxt::runtime_api::Payload<
                    types::MetadataAtVersion,
                    types::metadata_at_version::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Metadata",
                        "metadata_at_version",
                        types::MetadataAtVersion {
                            version,
                        },
                        [
                            131u8, 53u8, 212u8, 234u8, 16u8, 25u8, 120u8, 252u8, 153u8, 153u8,
                            216u8, 28u8, 54u8, 113u8, 52u8, 236u8, 146u8, 68u8, 142u8, 8u8, 10u8,
                            169u8, 131u8, 142u8, 204u8, 38u8, 48u8, 108u8, 134u8, 86u8, 226u8,
                            61u8,
                        ],
                    )
                }

                /// Returns the supported metadata versions.
                ///
                /// This can be used to call `metadata_at_version`.
                pub fn metadata_versions(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::MetadataVersions,
                    types::metadata_versions::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "Metadata",
                        "metadata_versions",
                        types::MetadataVersions {},
                        [
                            23u8, 144u8, 137u8, 91u8, 188u8, 39u8, 231u8, 208u8, 252u8, 218u8,
                            224u8, 176u8, 77u8, 32u8, 130u8, 212u8, 223u8, 76u8, 100u8, 190u8,
                            82u8, 94u8, 190u8, 8u8, 82u8, 244u8, 225u8, 179u8, 85u8, 176u8, 56u8,
                            16u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod metadata {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_core::OpaqueMetadata;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Metadata {}
                pub mod metadata_at_version {
                    use super::runtime_types;
                    pub type Version = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            ::core::option::Option<runtime_types::sp_core::OpaqueMetadata>;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MetadataAtVersion {
                    pub version: metadata_at_version::Version,
                }
                pub mod metadata_versions {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::std::vec::Vec<::core::primitive::u32>;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MetadataVersions {}
            }
        }
        pub mod block_builder {
            use super::root_mod;
            use super::runtime_types;
            /// The `BlockBuilder` api trait that provides the required
            /// functionality for building a block.
            pub struct BlockBuilder;
            impl BlockBuilder {
                /// Apply the given extrinsic.
                ///
                /// Returns an inclusion outcome which specifies if this
                /// extrinsic is included in
                /// this block or not.
                pub fn apply_extrinsic(
                    &self,
                    extrinsic: types::apply_extrinsic::Extrinsic,
                ) -> ::subxt::runtime_api::Payload<
                    types::ApplyExtrinsic,
                    types::apply_extrinsic::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "apply_extrinsic",
                        types::ApplyExtrinsic {
                            extrinsic,
                        },
                        [
                            72u8, 54u8, 139u8, 3u8, 118u8, 136u8, 65u8, 47u8, 6u8, 105u8, 125u8,
                            223u8, 160u8, 29u8, 103u8, 74u8, 79u8, 149u8, 48u8, 90u8, 237u8, 2u8,
                            97u8, 201u8, 123u8, 34u8, 167u8, 37u8, 187u8, 35u8, 176u8, 97u8,
                        ],
                    )
                }

                /// Finish the current block.
                pub fn finalize_block(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::FinalizeBlock,
                    types::finalize_block::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "finalize_block",
                        types::FinalizeBlock {},
                        [
                            244u8, 207u8, 24u8, 33u8, 13u8, 69u8, 9u8, 249u8, 145u8, 143u8, 122u8,
                            96u8, 197u8, 55u8, 64u8, 111u8, 238u8, 224u8, 34u8, 201u8, 27u8, 146u8,
                            232u8, 99u8, 191u8, 30u8, 114u8, 16u8, 32u8, 220u8, 58u8, 62u8,
                        ],
                    )
                }

                /// Generate inherent extrinsics. The inherent data will vary
                /// from chain to chain.
                pub fn inherent_extrinsics(
                    &self,
                    inherent: types::inherent_extrinsics::Inherent,
                ) -> ::subxt::runtime_api::Payload<
                    types::InherentExtrinsics,
                    types::inherent_extrinsics::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "inherent_extrinsics",
                        types::InherentExtrinsics {
                            inherent,
                        },
                        [
                            254u8, 110u8, 245u8, 201u8, 250u8, 192u8, 27u8, 228u8, 151u8, 213u8,
                            166u8, 89u8, 94u8, 81u8, 189u8, 234u8, 64u8, 18u8, 245u8, 80u8, 29u8,
                            18u8, 140u8, 129u8, 113u8, 236u8, 135u8, 55u8, 79u8, 159u8, 175u8,
                            183u8,
                        ],
                    )
                }

                /// Check that the inherents are valid. The inherent data will
                /// vary from chain to chain.
                pub fn check_inherents(
                    &self,
                    block: types::check_inherents::Block,
                    data: types::check_inherents::Data,
                ) -> ::subxt::runtime_api::Payload<
                    types::CheckInherents,
                    types::check_inherents::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "BlockBuilder",
                        "check_inherents",
                        types::CheckInherents {
                            block,
                            data,
                        },
                        [
                            153u8, 134u8, 1u8, 215u8, 139u8, 11u8, 53u8, 51u8, 210u8, 175u8, 197u8,
                            28u8, 38u8, 209u8, 175u8, 247u8, 142u8, 157u8, 50u8, 151u8, 164u8,
                            191u8, 181u8, 118u8, 80u8, 97u8, 160u8, 248u8, 110u8, 217u8, 181u8,
                            234u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod apply_extrinsic {
                    use super::runtime_types;
                    pub type Extrinsic = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: nagara_core_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: result :: Result < :: core :: result :: Result < () , runtime_types :: sp_runtime :: DispatchError > , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ApplyExtrinsic {
                    pub extrinsic: apply_extrinsic::Extrinsic,
                }
                pub mod finalize_block {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                        >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FinalizeBlock {}
                pub mod inherent_extrinsics {
                    use super::runtime_types;
                    pub type Inherent = runtime_types::sp_inherents::InherentData;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: std :: vec :: Vec < :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: nagara_core_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InherentExtrinsics {
                    pub inherent: inherent_extrinsics::Inherent,
                }
                pub mod check_inherents {
                    use super::runtime_types;
                    pub type Block = runtime_types :: sp_runtime :: generic :: block :: Block < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 > , :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: nagara_core_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > > ;
                    pub type Data = runtime_types::sp_inherents::InherentData;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_inherents::CheckInherentsResult;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckInherents {
                    pub block: check_inherents::Block,
                    pub data: check_inherents::Data,
                }
            }
        }
        pub mod tagged_transaction_queue {
            use super::root_mod;
            use super::runtime_types;
            /// The `TaggedTransactionQueue` api trait for interfering with the
            /// transaction queue.
            pub struct TaggedTransactionQueue;
            impl TaggedTransactionQueue {
                /// Validate the transaction.
                ///
                /// This method is invoked by the transaction pool to learn
                /// details about given transaction.
                /// The implementation should make sure to verify the
                /// correctness of the transaction
                /// against current state. The given `block_hash` corresponds to
                /// the hash of the block
                /// that is used as current state.
                ///
                /// Note that this call may be performed by the pool multiple
                /// times and transactions
                /// might be verified in any possible order.
                pub fn validate_transaction(
                    &self,
                    source: types::validate_transaction::Source,
                    tx: types::validate_transaction::Tx,
                    block_hash: types::validate_transaction::BlockHash,
                ) -> ::subxt::runtime_api::Payload<
                    types::ValidateTransaction,
                    types::validate_transaction::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TaggedTransactionQueue",
                        "validate_transaction",
                        types::ValidateTransaction {
                            source,
                            tx,
                            block_hash,
                        },
                        [
                            196u8, 50u8, 90u8, 49u8, 109u8, 251u8, 200u8, 35u8, 23u8, 150u8, 140u8,
                            143u8, 232u8, 164u8, 133u8, 89u8, 32u8, 240u8, 115u8, 39u8, 95u8, 70u8,
                            162u8, 76u8, 122u8, 73u8, 151u8, 144u8, 234u8, 120u8, 100u8, 29u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod validate_transaction {
                    use super::runtime_types;
                    pub type Source =
                        runtime_types::sp_runtime::transaction_validity::TransactionSource;
                    pub type Tx = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: nagara_core_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub type BlockHash = ::subxt::utils::H256;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = :: core :: result :: Result < runtime_types :: sp_runtime :: transaction_validity :: ValidTransaction , runtime_types :: sp_runtime :: transaction_validity :: TransactionValidityError > ;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidateTransaction {
                    pub source: validate_transaction::Source,
                    pub tx: validate_transaction::Tx,
                    pub block_hash: validate_transaction::BlockHash,
                }
            }
        }
        pub mod offchain_worker_api {
            use super::root_mod;
            use super::runtime_types;
            /// The offchain worker api.
            pub struct OffchainWorkerApi;
            impl OffchainWorkerApi {
                /// Starts the off-chain task for given block header.
                pub fn offchain_worker(
                    &self,
                    header: types::offchain_worker::Header,
                ) -> ::subxt::runtime_api::Payload<
                    types::OffchainWorker,
                    types::offchain_worker::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "OffchainWorkerApi",
                        "offchain_worker",
                        types::OffchainWorker {
                            header,
                        },
                        [
                            10u8, 135u8, 19u8, 153u8, 33u8, 216u8, 18u8, 242u8, 33u8, 140u8, 4u8,
                            223u8, 200u8, 130u8, 103u8, 118u8, 137u8, 24u8, 19u8, 127u8, 161u8,
                            29u8, 184u8, 111u8, 222u8, 111u8, 253u8, 73u8, 45u8, 31u8, 79u8, 60u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod offchain_worker {
                    use super::runtime_types;
                    pub type Header =
                        runtime_types::sp_runtime::generic::header::Header<::core::primitive::u32>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ();
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct OffchainWorker {
                    pub header: offchain_worker::Header,
                }
            }
        }
        pub mod aura_api {
            use super::root_mod;
            use super::runtime_types;
            /// API necessary for block authorship with aura.
            pub struct AuraApi;
            impl AuraApi {
                /// Returns the slot duration for Aura.
                ///
                /// Currently, only the value provided by this type at genesis
                /// will be used.
                pub fn slot_duration(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::SlotDuration,
                    types::slot_duration::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "AuraApi",
                        "slot_duration",
                        types::SlotDuration {},
                        [
                            233u8, 210u8, 132u8, 172u8, 100u8, 125u8, 239u8, 92u8, 114u8, 82u8,
                            7u8, 110u8, 179u8, 196u8, 10u8, 19u8, 211u8, 15u8, 174u8, 2u8, 91u8,
                            73u8, 133u8, 100u8, 205u8, 201u8, 191u8, 60u8, 163u8, 122u8, 215u8,
                            10u8,
                        ],
                    )
                }

                /// Return the current set of authorities.
                pub fn authorities(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::Authorities,
                    types::authorities::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "AuraApi",
                        "authorities",
                        types::Authorities {},
                        [
                            96u8, 136u8, 226u8, 244u8, 105u8, 189u8, 8u8, 250u8, 71u8, 230u8, 37u8,
                            123u8, 218u8, 47u8, 179u8, 16u8, 170u8, 181u8, 165u8, 77u8, 102u8,
                            51u8, 43u8, 51u8, 186u8, 84u8, 49u8, 15u8, 208u8, 226u8, 129u8, 230u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod slot_duration {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types::sp_consensus_slots::SlotDuration;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SlotDuration {}
                pub mod authorities {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::std::vec::Vec<
                            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                        >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Authorities {}
            }
        }
        pub mod session_keys {
            use super::root_mod;
            use super::runtime_types;
            /// Session keys runtime api.
            pub struct SessionKeys;
            impl SessionKeys {
                /// Generate a set of session keys with optionally using the
                /// given seed.
                /// The keys should be stored within the keystore exposed via
                /// runtime
                /// externalities.
                ///
                /// The seed needs to be a valid `utf8` string.
                ///
                /// Returns the concatenated SCALE encoded public keys.
                pub fn generate_session_keys(
                    &self,
                    seed: types::generate_session_keys::Seed,
                ) -> ::subxt::runtime_api::Payload<
                    types::GenerateSessionKeys,
                    types::generate_session_keys::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "SessionKeys",
                        "generate_session_keys",
                        types::GenerateSessionKeys {
                            seed,
                        },
                        [
                            96u8, 171u8, 164u8, 166u8, 175u8, 102u8, 101u8, 47u8, 133u8, 95u8,
                            102u8, 202u8, 83u8, 26u8, 238u8, 47u8, 126u8, 132u8, 22u8, 11u8, 33u8,
                            190u8, 175u8, 94u8, 58u8, 245u8, 46u8, 80u8, 195u8, 184u8, 107u8, 65u8,
                        ],
                    )
                }

                /// Decode the given public session keys.
                ///
                /// Returns the list of public raw public keys + key type.
                pub fn decode_session_keys(
                    &self,
                    encoded: types::decode_session_keys::Encoded,
                ) -> ::subxt::runtime_api::Payload<
                    types::DecodeSessionKeys,
                    types::decode_session_keys::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "SessionKeys",
                        "decode_session_keys",
                        types::DecodeSessionKeys {
                            encoded,
                        },
                        [
                            57u8, 242u8, 18u8, 51u8, 132u8, 110u8, 238u8, 255u8, 39u8, 194u8, 8u8,
                            54u8, 198u8, 178u8, 75u8, 151u8, 148u8, 176u8, 144u8, 197u8, 87u8,
                            29u8, 179u8, 235u8, 176u8, 78u8, 252u8, 103u8, 72u8, 203u8, 151u8,
                            248u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod generate_session_keys {
                    use super::runtime_types;
                    pub type Seed = ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::std::vec::Vec<::core::primitive::u8>;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GenerateSessionKeys {
                    pub seed: generate_session_keys::Seed,
                }
                pub mod decode_session_keys {
                    use super::runtime_types;
                    pub type Encoded = ::std::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::option::Option<
                            ::std::vec::Vec<(
                                ::std::vec::Vec<::core::primitive::u8>,
                                runtime_types::sp_core::crypto::KeyTypeId,
                            )>,
                        >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DecodeSessionKeys {
                    pub encoded: decode_session_keys::Encoded,
                }
            }
        }
        pub mod grandpa_api {
            use super::root_mod;
            use super::runtime_types;
            /// APIs for integrating the GRANDPA finality gadget into runtimes.
            /// This should be implemented on the runtime side.
            ///
            /// This is primarily used for negotiating authority-set changes for
            /// the
            /// gadget. GRANDPA uses a signaling model of changing authority
            /// sets:
            /// changes should be signaled with a delay of N blocks, and then
            /// automatically
            /// applied in the runtime after those N blocks have passed.
            ///
            /// The consensus protocol will coordinate the handoff externally.
            pub struct GrandpaApi;
            impl GrandpaApi {
                /// Get the current GRANDPA authorities and weights. This should
                /// not change except
                /// for when changes are scheduled and the corresponding delay
                /// has passed.
                ///
                /// When called at block B, it will return the set of
                /// authorities that should be
                /// used to finalize descendants of this block (B+1, B+2, ...).
                /// The block B itself
                /// is finalized by the authorities from block B-1.
                pub fn grandpa_authorities(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::GrandpaAuthorities,
                    types::grandpa_authorities::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "grandpa_authorities",
                        types::GrandpaAuthorities {},
                        [
                            166u8, 76u8, 160u8, 101u8, 242u8, 145u8, 213u8, 10u8, 16u8, 130u8,
                            230u8, 196u8, 125u8, 152u8, 92u8, 143u8, 119u8, 223u8, 140u8, 189u8,
                            203u8, 95u8, 52u8, 105u8, 147u8, 107u8, 135u8, 228u8, 62u8, 178u8,
                            128u8, 33u8,
                        ],
                    )
                }

                /// Submits an unsigned extrinsic to report an equivocation. The
                /// caller
                /// must provide the equivocation proof and a key ownership
                /// proof
                /// (should be obtained using `generate_key_ownership_proof`).
                /// The
                /// extrinsic will be unsigned and should only be accepted for
                /// local
                /// authorship (not to be broadcast to the network). This method
                /// returns
                /// `None` when creation of the extrinsic fails, e.g. if
                /// equivocation
                /// reporting is disabled for the given runtime (i.e. this
                /// method is
                /// hardcoded to return `None`). Only useful in an offchain
                /// context.
                pub fn submit_report_equivocation_unsigned_extrinsic(
                    &self,
                    equivocation_proof : types :: submit_report_equivocation_unsigned_extrinsic :: EquivocationProof,
                    key_owner_proof : types :: submit_report_equivocation_unsigned_extrinsic :: KeyOwnerProof,
                ) -> ::subxt::runtime_api::Payload<
                    types::SubmitReportEquivocationUnsignedExtrinsic,
                    types::submit_report_equivocation_unsigned_extrinsic::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "submit_report_equivocation_unsigned_extrinsic",
                        types::SubmitReportEquivocationUnsignedExtrinsic {
                            equivocation_proof,
                            key_owner_proof,
                        },
                        [
                            112u8, 94u8, 150u8, 250u8, 132u8, 127u8, 185u8, 24u8, 113u8, 62u8,
                            28u8, 171u8, 83u8, 9u8, 41u8, 228u8, 92u8, 137u8, 29u8, 190u8, 214u8,
                            232u8, 100u8, 66u8, 100u8, 168u8, 149u8, 122u8, 93u8, 17u8, 236u8,
                            104u8,
                        ],
                    )
                }

                /// Generates a proof of key ownership for the given authority
                /// in the
                /// given set. An example usage of this module is coupled with
                /// the
                /// session historical module to prove that a given authority
                /// key is
                /// tied to a given staking identity during a specific session.
                /// Proofs
                /// of key ownership are necessary for submitting equivocation
                /// reports.
                /// NOTE: even though the API takes a `set_id` as parameter the
                /// current
                /// implementations ignore this parameter and instead rely on
                /// this
                /// method being called at the correct block height, i.e. any
                /// point at
                /// which the given set id is live on-chain. Future
                /// implementations will
                /// instead use indexed data through an offchain worker, not
                /// requiring
                /// older states to be available.
                pub fn generate_key_ownership_proof(
                    &self,
                    set_id: types::generate_key_ownership_proof::SetId,
                    authority_id: types::generate_key_ownership_proof::AuthorityId,
                ) -> ::subxt::runtime_api::Payload<
                    types::GenerateKeyOwnershipProof,
                    types::generate_key_ownership_proof::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "generate_key_ownership_proof",
                        types::GenerateKeyOwnershipProof {
                            set_id,
                            authority_id,
                        },
                        [
                            40u8, 126u8, 113u8, 27u8, 245u8, 45u8, 123u8, 138u8, 12u8, 3u8, 125u8,
                            186u8, 151u8, 53u8, 186u8, 93u8, 13u8, 150u8, 163u8, 176u8, 206u8,
                            89u8, 244u8, 127u8, 182u8, 85u8, 203u8, 41u8, 101u8, 183u8, 209u8,
                            179u8,
                        ],
                    )
                }

                /// Get current GRANDPA authority set id.
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::runtime_api::Payload<
                    types::CurrentSetId,
                    types::current_set_id::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "GrandpaApi",
                        "current_set_id",
                        types::CurrentSetId {},
                        [
                            42u8, 230u8, 120u8, 211u8, 156u8, 245u8, 109u8, 86u8, 100u8, 146u8,
                            234u8, 205u8, 41u8, 183u8, 109u8, 42u8, 17u8, 33u8, 156u8, 25u8, 139u8,
                            84u8, 101u8, 75u8, 232u8, 198u8, 87u8, 136u8, 218u8, 233u8, 103u8,
                            156u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod grandpa_authorities {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::std::vec::Vec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GrandpaAuthorities {}
                pub mod submit_report_equivocation_unsigned_extrinsic {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >;
                    pub type KeyOwnerProof =
                        runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::option::Option<()>;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SubmitReportEquivocationUnsignedExtrinsic {
                    pub equivocation_proof:
                        submit_report_equivocation_unsigned_extrinsic::EquivocationProof,
                    pub key_owner_proof:
                        submit_report_equivocation_unsigned_extrinsic::KeyOwnerProof,
                }
                pub mod generate_key_ownership_proof {
                    use super::runtime_types;
                    pub type SetId = ::core::primitive::u64;
                    pub type AuthorityId = runtime_types::sp_consensus_grandpa::app::Public;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::option::Option<
                            runtime_types::sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                        >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GenerateKeyOwnershipProof {
                    pub set_id: generate_key_ownership_proof::SetId,
                    pub authority_id: generate_key_ownership_proof::AuthorityId,
                }
                pub mod current_set_id {
                    use super::runtime_types;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u64;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CurrentSetId {}
            }
        }
        pub mod account_nonce_api {
            use super::root_mod;
            use super::runtime_types;
            /// The API to query account nonce.
            pub struct AccountNonceApi;
            impl AccountNonceApi {
                /// Get current account nonce of given `AccountId`.
                pub fn account_nonce(
                    &self,
                    account: types::account_nonce::Account,
                ) -> ::subxt::runtime_api::Payload<
                    types::AccountNonce,
                    types::account_nonce::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "AccountNonceApi",
                        "account_nonce",
                        types::AccountNonce {
                            account,
                        },
                        [
                            231u8, 82u8, 7u8, 227u8, 131u8, 2u8, 215u8, 252u8, 173u8, 82u8, 11u8,
                            103u8, 200u8, 25u8, 114u8, 116u8, 79u8, 229u8, 152u8, 150u8, 236u8,
                            37u8, 101u8, 26u8, 220u8, 146u8, 182u8, 101u8, 73u8, 55u8, 191u8,
                            171u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod account_nonce {
                    use super::runtime_types;
                    pub type Account = ::subxt::utils::AccountId32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u32;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountNonce {
                    pub account: account_nonce::Account,
                }
            }
        }
        pub mod transaction_payment_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentApi;
            impl TransactionPaymentApi {
                pub fn query_info(
                    &self,
                    uxt: types::query_info::Uxt,
                    len: types::query_info::Len,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryInfo,
                    types::query_info::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_info",
                        types::QueryInfo {
                            uxt,
                            len,
                        },
                        [
                            56u8, 30u8, 174u8, 34u8, 202u8, 24u8, 177u8, 189u8, 145u8, 36u8, 1u8,
                            156u8, 98u8, 209u8, 178u8, 49u8, 198u8, 23u8, 150u8, 173u8, 35u8,
                            205u8, 147u8, 129u8, 42u8, 22u8, 69u8, 3u8, 129u8, 8u8, 196u8, 139u8,
                        ],
                    )
                }

                pub fn query_fee_details(
                    &self,
                    uxt: types::query_fee_details::Uxt,
                    len: types::query_fee_details::Len,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryFeeDetails,
                    types::query_fee_details::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_fee_details",
                        types::QueryFeeDetails {
                            uxt,
                            len,
                        },
                        [
                            117u8, 60u8, 137u8, 159u8, 237u8, 252u8, 216u8, 238u8, 232u8, 1u8,
                            100u8, 152u8, 26u8, 185u8, 145u8, 125u8, 68u8, 189u8, 4u8, 30u8, 125u8,
                            7u8, 196u8, 153u8, 235u8, 51u8, 219u8, 108u8, 185u8, 254u8, 100u8,
                            201u8,
                        ],
                    )
                }

                pub fn query_weight_to_fee(
                    &self,
                    weight: types::query_weight_to_fee::Weight,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryWeightToFee,
                    types::query_weight_to_fee::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_weight_to_fee",
                        types::QueryWeightToFee {
                            weight,
                        },
                        [
                            206u8, 243u8, 189u8, 83u8, 231u8, 244u8, 247u8, 52u8, 126u8, 208u8,
                            224u8, 5u8, 163u8, 108u8, 254u8, 114u8, 214u8, 156u8, 227u8, 217u8,
                            211u8, 198u8, 121u8, 164u8, 110u8, 54u8, 181u8, 146u8, 50u8, 146u8,
                            146u8, 23u8,
                        ],
                    )
                }

                pub fn query_length_to_fee(
                    &self,
                    length: types::query_length_to_fee::Length,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryLengthToFee,
                    types::query_length_to_fee::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentApi",
                        "query_length_to_fee",
                        types::QueryLengthToFee {
                            length,
                        },
                        [
                            92u8, 132u8, 29u8, 119u8, 66u8, 11u8, 196u8, 224u8, 129u8, 23u8, 249u8,
                            12u8, 32u8, 28u8, 92u8, 50u8, 188u8, 101u8, 203u8, 229u8, 248u8, 216u8,
                            130u8, 150u8, 212u8, 161u8, 81u8, 254u8, 116u8, 89u8, 162u8, 48u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod query_info {
                    use super::runtime_types;
                    pub type Uxt = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: nagara_core_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                                ::core::primitive::u128,
                                runtime_types::sp_weights::weight_v2::Weight,
                            >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryInfo {
                    pub uxt: query_info::Uxt,
                    pub len: query_info::Len,
                }
                pub mod query_fee_details {
                    use super::runtime_types;
                    pub type Uxt = :: subxt :: utils :: UncheckedExtrinsic < :: subxt :: utils :: MultiAddress < :: subxt :: utils :: AccountId32 , () > , runtime_types :: nagara_core_runtime :: RuntimeCall , runtime_types :: sp_runtime :: MultiSignature , (runtime_types :: frame_system :: extensions :: check_non_zero_sender :: CheckNonZeroSender , runtime_types :: frame_system :: extensions :: check_spec_version :: CheckSpecVersion , runtime_types :: frame_system :: extensions :: check_tx_version :: CheckTxVersion , runtime_types :: frame_system :: extensions :: check_genesis :: CheckGenesis , runtime_types :: frame_system :: extensions :: check_mortality :: CheckMortality , runtime_types :: frame_system :: extensions :: check_nonce :: CheckNonce , runtime_types :: frame_system :: extensions :: check_weight :: CheckWeight , runtime_types :: pallet_transaction_payment :: ChargeTransactionPayment ,) > ;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::FeeDetails<
                                ::core::primitive::u128,
                            >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryFeeDetails {
                    pub uxt: query_fee_details::Uxt,
                    pub len: query_fee_details::Len,
                }
                pub mod query_weight_to_fee {
                    use super::runtime_types;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u128;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryWeightToFee {
                    pub weight: query_weight_to_fee::Weight,
                }
                pub mod query_length_to_fee {
                    use super::runtime_types;
                    pub type Length = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u128;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryLengthToFee {
                    pub length: query_length_to_fee::Length,
                }
            }
        }
        pub mod transaction_payment_call_api {
            use super::root_mod;
            use super::runtime_types;
            pub struct TransactionPaymentCallApi;
            impl TransactionPaymentCallApi {
                /// Query information of a dispatch class, weight, and fee of a
                /// given encoded `Call`.
                pub fn query_call_info(
                    &self,
                    call: types::query_call_info::Call,
                    len: types::query_call_info::Len,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryCallInfo,
                    types::query_call_info::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_info",
                        types::QueryCallInfo {
                            call,
                            len,
                        },
                        [
                            163u8, 9u8, 97u8, 75u8, 149u8, 253u8, 250u8, 112u8, 177u8, 108u8, 7u8,
                            214u8, 196u8, 215u8, 100u8, 253u8, 73u8, 9u8, 238u8, 180u8, 234u8,
                            225u8, 139u8, 71u8, 50u8, 21u8, 101u8, 92u8, 16u8, 83u8, 6u8, 219u8,
                        ],
                    )
                }

                /// Query fee details of a given encoded `Call`.
                pub fn query_call_fee_details(
                    &self,
                    call: types::query_call_fee_details::Call,
                    len: types::query_call_fee_details::Len,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryCallFeeDetails,
                    types::query_call_fee_details::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_call_fee_details",
                        types::QueryCallFeeDetails {
                            call,
                            len,
                        },
                        [
                            227u8, 28u8, 221u8, 10u8, 107u8, 163u8, 87u8, 96u8, 168u8, 6u8, 16u8,
                            254u8, 92u8, 149u8, 99u8, 202u8, 167u8, 46u8, 191u8, 177u8, 71u8,
                            118u8, 134u8, 17u8, 4u8, 200u8, 207u8, 225u8, 203u8, 150u8, 27u8, 62u8,
                        ],
                    )
                }

                /// Query the output of the current `WeightToFee` given some
                /// input.
                pub fn query_weight_to_fee(
                    &self,
                    weight: types::query_weight_to_fee::Weight,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryWeightToFee,
                    types::query_weight_to_fee::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_weight_to_fee",
                        types::QueryWeightToFee {
                            weight,
                        },
                        [
                            117u8, 91u8, 94u8, 22u8, 248u8, 212u8, 15u8, 23u8, 97u8, 116u8, 64u8,
                            228u8, 83u8, 123u8, 87u8, 77u8, 97u8, 7u8, 98u8, 181u8, 6u8, 165u8,
                            114u8, 141u8, 164u8, 113u8, 126u8, 88u8, 174u8, 171u8, 224u8, 35u8,
                        ],
                    )
                }

                /// Query the output of the current `LengthToFee` given some
                /// input.
                pub fn query_length_to_fee(
                    &self,
                    length: types::query_length_to_fee::Length,
                ) -> ::subxt::runtime_api::Payload<
                    types::QueryLengthToFee,
                    types::query_length_to_fee::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "TransactionPaymentCallApi",
                        "query_length_to_fee",
                        types::QueryLengthToFee {
                            length,
                        },
                        [
                            246u8, 40u8, 4u8, 160u8, 152u8, 94u8, 170u8, 53u8, 205u8, 122u8, 5u8,
                            69u8, 70u8, 25u8, 128u8, 156u8, 119u8, 134u8, 116u8, 147u8, 14u8,
                            164u8, 65u8, 140u8, 86u8, 13u8, 250u8, 218u8, 89u8, 95u8, 234u8, 228u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod query_call_info {
                    use super::runtime_types;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::RuntimeDispatchInfo<
                                ::core::primitive::u128,
                                runtime_types::sp_weights::weight_v2::Weight,
                            >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryCallInfo {
                    pub call: query_call_info::Call,
                    pub len: query_call_info::Len,
                }
                pub mod query_call_fee_details {
                    use super::runtime_types;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                    pub type Len = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_transaction_payment::types::FeeDetails<
                                ::core::primitive::u128,
                            >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryCallFeeDetails {
                    pub call: query_call_fee_details::Call,
                    pub len: query_call_fee_details::Len,
                }
                pub mod query_weight_to_fee {
                    use super::runtime_types;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u128;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryWeightToFee {
                    pub weight: query_weight_to_fee::Weight,
                }
                pub mod query_length_to_fee {
                    use super::runtime_types;
                    pub type Length = ::core::primitive::u32;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::primitive::u128;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct QueryLengthToFee {
                    pub length: query_length_to_fee::Length,
                }
            }
        }
        pub mod contracts_api {
            use super::root_mod;
            use super::runtime_types;
            /// The API used to dry-run contract interactions.
            pub struct ContractsApi;
            impl ContractsApi {
                /// Perform a call from a specified account to a given contract.
                ///
                /// See [`crate::Pallet::bare_call`].
                pub fn call(
                    &self,
                    origin: types::call::Origin,
                    dest: types::call::Dest,
                    value: types::call::Value,
                    gas_limit: types::call::GasLimit,
                    storage_deposit_limit: types::call::StorageDepositLimit,
                    input_data: types::call::InputData,
                ) -> ::subxt::runtime_api::Payload<types::Call, types::call::output::Output>
                {
                    ::subxt::runtime_api::Payload::new_static(
                        "ContractsApi",
                        "call",
                        types::Call {
                            origin,
                            dest,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            input_data,
                        },
                        [
                            167u8, 161u8, 196u8, 16u8, 59u8, 82u8, 23u8, 96u8, 163u8, 10u8, 149u8,
                            16u8, 113u8, 2u8, 109u8, 216u8, 107u8, 21u8, 237u8, 175u8, 105u8, 73u8,
                            201u8, 186u8, 220u8, 208u8, 214u8, 163u8, 134u8, 223u8, 96u8, 109u8,
                        ],
                    )
                }

                /// Instantiate a new contract.
                ///
                /// See `[crate::Pallet::bare_instantiate]`.
                pub fn instantiate(
                    &self,
                    origin: types::instantiate::Origin,
                    value: types::instantiate::Value,
                    gas_limit: types::instantiate::GasLimit,
                    storage_deposit_limit: types::instantiate::StorageDepositLimit,
                    code: types::instantiate::Code,
                    data: types::instantiate::Data,
                    salt: types::instantiate::Salt,
                ) -> ::subxt::runtime_api::Payload<
                    types::Instantiate,
                    types::instantiate::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ContractsApi",
                        "instantiate",
                        types::Instantiate {
                            origin,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code,
                            data,
                            salt,
                        },
                        [
                            151u8, 57u8, 2u8, 236u8, 9u8, 253u8, 18u8, 219u8, 238u8, 143u8, 197u8,
                            169u8, 248u8, 44u8, 167u8, 91u8, 50u8, 183u8, 171u8, 191u8, 53u8,
                            123u8, 40u8, 87u8, 190u8, 52u8, 123u8, 164u8, 165u8, 87u8, 35u8, 52u8,
                        ],
                    )
                }

                /// Upload new code without instantiating a contract from it.
                ///
                /// See [`crate::Pallet::bare_upload_code`].
                pub fn upload_code(
                    &self,
                    origin: types::upload_code::Origin,
                    code: types::upload_code::Code,
                    storage_deposit_limit: types::upload_code::StorageDepositLimit,
                    determinism: types::upload_code::Determinism,
                ) -> ::subxt::runtime_api::Payload<
                    types::UploadCode,
                    types::upload_code::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ContractsApi",
                        "upload_code",
                        types::UploadCode {
                            origin,
                            code,
                            storage_deposit_limit,
                            determinism,
                        },
                        [
                            231u8, 114u8, 110u8, 91u8, 142u8, 108u8, 124u8, 161u8, 13u8, 8u8,
                            127u8, 134u8, 133u8, 152u8, 137u8, 67u8, 59u8, 78u8, 120u8, 75u8,
                            172u8, 211u8, 23u8, 227u8, 90u8, 203u8, 204u8, 129u8, 142u8, 226u8,
                            32u8, 213u8,
                        ],
                    )
                }

                /// Query a given storage key in a given contract.
                ///
                /// Returns `Ok(Some(Vec<u8>))` if the storage value exists
                /// under the given key in the
                /// specified account and `Ok(None)` if it doesn't. If the
                /// account specified by the address
                /// doesn't exist, or doesn't have a contract then `Err` is
                /// returned.
                pub fn get_storage(
                    &self,
                    address: types::get_storage::Address,
                    key: types::get_storage::Key,
                ) -> ::subxt::runtime_api::Payload<
                    types::GetStorage,
                    types::get_storage::output::Output,
                > {
                    ::subxt::runtime_api::Payload::new_static(
                        "ContractsApi",
                        "get_storage",
                        types::GetStorage {
                            address,
                            key,
                        },
                        [
                            49u8, 103u8, 100u8, 132u8, 135u8, 193u8, 145u8, 48u8, 154u8, 78u8,
                            41u8, 43u8, 81u8, 109u8, 146u8, 199u8, 6u8, 111u8, 184u8, 102u8, 46u8,
                            76u8, 174u8, 148u8, 106u8, 184u8, 131u8, 137u8, 194u8, 98u8, 179u8,
                            45u8,
                        ],
                    )
                }
            }
            pub mod types {
                use super::runtime_types;
                pub mod call {
                    use super::runtime_types;
                    pub type Origin = ::subxt::utils::AccountId32;
                    pub type Dest = ::subxt::utils::AccountId32;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit =
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>;
                    pub type StorageDepositLimit = ::core::option::Option<::core::primitive::u128>;
                    pub type InputData = ::std::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output =
                            runtime_types::pallet_contracts_primitives::ContractResult<
                                ::core::result::Result<
                                    runtime_types::pallet_contracts_primitives::ExecReturnValue,
                                    runtime_types::sp_runtime::DispatchError,
                                >,
                                ::core::primitive::u128,
                                runtime_types::frame_system::EventRecord<
                                    runtime_types::nagara_core_runtime::RuntimeEvent,
                                    ::subxt::utils::H256,
                                >,
                            >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Call {
                    pub origin: call::Origin,
                    pub dest: call::Dest,
                    pub value: call::Value,
                    pub gas_limit: call::GasLimit,
                    pub storage_deposit_limit: call::StorageDepositLimit,
                    pub input_data: call::InputData,
                }
                pub mod instantiate {
                    use super::runtime_types;
                    pub type Origin = ::subxt::utils::AccountId32;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit =
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>;
                    pub type StorageDepositLimit = ::core::option::Option<::core::primitive::u128>;
                    pub type Code =
                        runtime_types::pallet_contracts_primitives::Code<::subxt::utils::H256>;
                    pub type Data = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Salt = ::std::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = runtime_types :: pallet_contracts_primitives :: ContractResult < :: core :: result :: Result < runtime_types :: pallet_contracts_primitives :: InstantiateReturnValue < :: subxt :: utils :: AccountId32 > , runtime_types :: sp_runtime :: DispatchError > , :: core :: primitive :: u128 , runtime_types :: frame_system :: EventRecord < runtime_types :: nagara_core_runtime :: RuntimeEvent , :: subxt :: utils :: H256 > > ;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Instantiate {
                    pub origin: instantiate::Origin,
                    pub value: instantiate::Value,
                    pub gas_limit: instantiate::GasLimit,
                    pub storage_deposit_limit: instantiate::StorageDepositLimit,
                    pub code: instantiate::Code,
                    pub data: instantiate::Data,
                    pub salt: instantiate::Salt,
                }
                pub mod upload_code {
                    use super::runtime_types;
                    pub type Origin = ::subxt::utils::AccountId32;
                    pub type Code = ::std::vec::Vec<::core::primitive::u8>;
                    pub type StorageDepositLimit = ::core::option::Option<::core::primitive::u128>;
                    pub type Determinism = runtime_types::pallet_contracts::wasm::Determinism;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::result::Result<
                            runtime_types::pallet_contracts_primitives::CodeUploadReturnValue<
                                ::subxt::utils::H256,
                                ::core::primitive::u128,
                            >,
                            runtime_types::sp_runtime::DispatchError,
                        >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UploadCode {
                    pub origin: upload_code::Origin,
                    pub code: upload_code::Code,
                    pub storage_deposit_limit: upload_code::StorageDepositLimit,
                    pub determinism: upload_code::Determinism,
                }
                pub mod get_storage {
                    use super::runtime_types;
                    pub type Address = ::subxt::utils::AccountId32;
                    pub type Key = ::std::vec::Vec<::core::primitive::u8>;
                    pub mod output {
                        use super::runtime_types;
                        pub type Output = ::core::result::Result<
                            ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                            runtime_types::pallet_contracts_primitives::ContractAccessError,
                        >;
                    }
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct GetStorage {
                    pub address: get_storage::Address,
                    pub key: get_storage::Key,
                }
            }
        }
    }
    pub fn custom() -> CustomValuesApi {
        CustomValuesApi
    }
    pub struct CustomValuesApi;
    impl CustomValuesApi {}
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }

        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }

        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }

        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }

        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }

        pub fn utility(&self) -> utility::constants::ConstantsApi {
            utility::constants::ConstantsApi
        }

        pub fn contracts(&self) -> contracts::constants::ConstantsApi {
            contracts::constants::ConstantsApi
        }

        pub fn assets(&self) -> assets::constants::ConstantsApi {
            assets::constants::ConstantsApi
        }

        pub fn multisig(&self) -> multisig::constants::ConstantsApi {
            multisig::constants::ConstantsApi
        }

        pub fn identity(&self) -> identity::constants::ConstantsApi {
            identity::constants::ConstantsApi
        }

        pub fn big_brother_council(&self) -> big_brother_council::constants::ConstantsApi {
            big_brother_council::constants::ConstantsApi
        }

        pub fn servicer_registry(&self) -> servicer_registry::constants::ConstantsApi {
            servicer_registry::constants::ConstantsApi
        }
    }
    pub struct StorageApi;
    impl StorageApi {
        pub fn system(&self) -> system::storage::StorageApi {
            system::storage::StorageApi
        }

        pub fn timestamp(&self) -> timestamp::storage::StorageApi {
            timestamp::storage::StorageApi
        }

        pub fn balances(&self) -> balances::storage::StorageApi {
            balances::storage::StorageApi
        }

        pub fn validator_set(&self) -> validator_set::storage::StorageApi {
            validator_set::storage::StorageApi
        }

        pub fn session(&self) -> session::storage::StorageApi {
            session::storage::StorageApi
        }

        pub fn aura(&self) -> aura::storage::StorageApi {
            aura::storage::StorageApi
        }

        pub fn grandpa(&self) -> grandpa::storage::StorageApi {
            grandpa::storage::StorageApi
        }

        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi {
            transaction_payment::storage::StorageApi
        }

        pub fn sudo(&self) -> sudo::storage::StorageApi {
            sudo::storage::StorageApi
        }

        pub fn randomness_collective_flip(
            &self,
        ) -> randomness_collective_flip::storage::StorageApi {
            randomness_collective_flip::storage::StorageApi
        }

        pub fn contracts(&self) -> contracts::storage::StorageApi {
            contracts::storage::StorageApi
        }

        pub fn assets(&self) -> assets::storage::StorageApi {
            assets::storage::StorageApi
        }

        pub fn multisig(&self) -> multisig::storage::StorageApi {
            multisig::storage::StorageApi
        }

        pub fn identity(&self) -> identity::storage::StorageApi {
            identity::storage::StorageApi
        }

        pub fn big_brother_council(&self) -> big_brother_council::storage::StorageApi {
            big_brother_council::storage::StorageApi
        }

        pub fn servicer_registry(&self) -> servicer_registry::storage::StorageApi {
            servicer_registry::storage::StorageApi
        }
    }
    pub struct TransactionApi;
    impl TransactionApi {
        pub fn system(&self) -> system::calls::TransactionApi {
            system::calls::TransactionApi
        }

        pub fn timestamp(&self) -> timestamp::calls::TransactionApi {
            timestamp::calls::TransactionApi
        }

        pub fn balances(&self) -> balances::calls::TransactionApi {
            balances::calls::TransactionApi
        }

        pub fn validator_set(&self) -> validator_set::calls::TransactionApi {
            validator_set::calls::TransactionApi
        }

        pub fn session(&self) -> session::calls::TransactionApi {
            session::calls::TransactionApi
        }

        pub fn grandpa(&self) -> grandpa::calls::TransactionApi {
            grandpa::calls::TransactionApi
        }

        pub fn sudo(&self) -> sudo::calls::TransactionApi {
            sudo::calls::TransactionApi
        }

        pub fn utility(&self) -> utility::calls::TransactionApi {
            utility::calls::TransactionApi
        }

        pub fn contracts(&self) -> contracts::calls::TransactionApi {
            contracts::calls::TransactionApi
        }

        pub fn assets(&self) -> assets::calls::TransactionApi {
            assets::calls::TransactionApi
        }

        pub fn multisig(&self) -> multisig::calls::TransactionApi {
            multisig::calls::TransactionApi
        }

        pub fn identity(&self) -> identity::calls::TransactionApi {
            identity::calls::TransactionApi
        }

        pub fn big_brother_council(&self) -> big_brother_council::calls::TransactionApi {
            big_brother_council::calls::TransactionApi
        }

        pub fn servicer_registry(&self) -> servicer_registry::calls::TransactionApi {
            servicer_registry::calls::TransactionApi
        }
    }
    /// check whether the metadata provided is aligned with this statically
    /// generated code.
    pub fn is_codegen_valid_for(metadata: &::subxt::Metadata) -> bool {
        let runtime_metadata_hash = metadata
            .hasher()
            .only_these_pallets(&PALLETS)
            .only_these_runtime_apis(&RUNTIME_APIS)
            .hash();
        runtime_metadata_hash
            == [
                182u8, 85u8, 61u8, 255u8, 34u8, 17u8, 54u8, 39u8, 40u8, 13u8, 133u8, 117u8, 107u8,
                103u8, 40u8, 219u8, 142u8, 88u8, 32u8, 203u8, 63u8, 68u8, 101u8, 189u8, 52u8, 49u8,
                0u8, 100u8, 181u8, 124u8, 238u8, 78u8,
            ]
    }
    pub mod system {
        use super::root_mod;
        use super::runtime_types;
        /// Error for the System pallet
        pub type Error = runtime_types::frame_system::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::frame_system::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::remark`].
                pub struct Remark {
                    pub remark: remark::Remark,
                }
                pub mod remark {
                    use super::runtime_types;
                    pub type Remark = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for Remark {
                    const CALL: &'static str = "remark";
                    const PALLET: &'static str = "System";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_heap_pages`].
                pub struct SetHeapPages {
                    pub pages: set_heap_pages::Pages,
                }
                pub mod set_heap_pages {
                    use super::runtime_types;
                    pub type Pages = ::core::primitive::u64;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetHeapPages {
                    const CALL: &'static str = "set_heap_pages";
                    const PALLET: &'static str = "System";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_code`].
                pub struct SetCode {
                    pub code: set_code::Code,
                }
                pub mod set_code {
                    use super::runtime_types;
                    pub type Code = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCode {
                    const CALL: &'static str = "set_code";
                    const PALLET: &'static str = "System";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_code_without_checks`].
                pub struct SetCodeWithoutChecks {
                    pub code: set_code_without_checks::Code,
                }
                pub mod set_code_without_checks {
                    use super::runtime_types;
                    pub type Code = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCodeWithoutChecks {
                    const CALL: &'static str = "set_code_without_checks";
                    const PALLET: &'static str = "System";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_storage`].
                pub struct SetStorage {
                    pub items: set_storage::Items,
                }
                pub mod set_storage {
                    use super::runtime_types;
                    pub type Items = ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetStorage {
                    const CALL: &'static str = "set_storage";
                    const PALLET: &'static str = "System";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::kill_storage`].
                pub struct KillStorage {
                    pub keys: kill_storage::Keys,
                }
                pub mod kill_storage {
                    use super::runtime_types;
                    pub type Keys = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                }
                impl ::subxt::blocks::StaticExtrinsic for KillStorage {
                    const CALL: &'static str = "kill_storage";
                    const PALLET: &'static str = "System";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::kill_prefix`].
                pub struct KillPrefix {
                    pub prefix: kill_prefix::Prefix,
                    pub subkeys: kill_prefix::Subkeys,
                }
                pub mod kill_prefix {
                    use super::runtime_types;
                    pub type Prefix = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Subkeys = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for KillPrefix {
                    const CALL: &'static str = "kill_prefix";
                    const PALLET: &'static str = "System";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::remark_with_event`].
                pub struct RemarkWithEvent {
                    pub remark: remark_with_event::Remark,
                }
                pub mod remark_with_event {
                    use super::runtime_types;
                    pub type Remark = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for RemarkWithEvent {
                    const CALL: &'static str = "remark_with_event";
                    const PALLET: &'static str = "System";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::remark`].
                pub fn remark(
                    &self,
                    remark: types::remark::Remark,
                ) -> ::subxt::tx::Payload<types::Remark> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark",
                        types::Remark {
                            remark,
                        },
                        [
                            43u8, 126u8, 180u8, 174u8, 141u8, 48u8, 52u8, 125u8, 166u8, 212u8,
                            216u8, 98u8, 100u8, 24u8, 132u8, 71u8, 101u8, 64u8, 246u8, 169u8, 33u8,
                            250u8, 147u8, 208u8, 2u8, 40u8, 129u8, 209u8, 232u8, 207u8, 207u8,
                            13u8,
                        ],
                    )
                }

                /// See [`Pallet::set_heap_pages`].
                pub fn set_heap_pages(
                    &self,
                    pages: types::set_heap_pages::Pages,
                ) -> ::subxt::tx::Payload<types::SetHeapPages> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_heap_pages",
                        types::SetHeapPages {
                            pages,
                        },
                        [
                            188u8, 191u8, 99u8, 216u8, 219u8, 109u8, 141u8, 50u8, 78u8, 235u8,
                            215u8, 242u8, 195u8, 24u8, 111u8, 76u8, 229u8, 64u8, 99u8, 225u8,
                            134u8, 121u8, 81u8, 209u8, 127u8, 223u8, 98u8, 215u8, 150u8, 70u8,
                            57u8, 147u8,
                        ],
                    )
                }

                /// See [`Pallet::set_code`].
                pub fn set_code(
                    &self,
                    code: types::set_code::Code,
                ) -> ::subxt::tx::Payload<types::SetCode> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code",
                        types::SetCode {
                            code,
                        },
                        [
                            233u8, 248u8, 88u8, 245u8, 28u8, 65u8, 25u8, 169u8, 35u8, 237u8, 19u8,
                            203u8, 136u8, 160u8, 18u8, 3u8, 20u8, 197u8, 81u8, 169u8, 244u8, 188u8,
                            27u8, 147u8, 147u8, 236u8, 65u8, 25u8, 3u8, 143u8, 182u8, 22u8,
                        ],
                    )
                }

                /// See [`Pallet::set_code_without_checks`].
                pub fn set_code_without_checks(
                    &self,
                    code: types::set_code_without_checks::Code,
                ) -> ::subxt::tx::Payload<types::SetCodeWithoutChecks> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_code_without_checks",
                        types::SetCodeWithoutChecks {
                            code,
                        },
                        [
                            82u8, 212u8, 157u8, 44u8, 70u8, 0u8, 143u8, 15u8, 109u8, 109u8, 107u8,
                            157u8, 141u8, 42u8, 169u8, 11u8, 15u8, 186u8, 252u8, 138u8, 10u8,
                            147u8, 15u8, 178u8, 247u8, 229u8, 213u8, 98u8, 207u8, 231u8, 119u8,
                            115u8,
                        ],
                    )
                }

                /// See [`Pallet::set_storage`].
                pub fn set_storage(
                    &self,
                    items: types::set_storage::Items,
                ) -> ::subxt::tx::Payload<types::SetStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "set_storage",
                        types::SetStorage {
                            items,
                        },
                        [
                            141u8, 216u8, 52u8, 222u8, 223u8, 136u8, 123u8, 181u8, 19u8, 75u8,
                            163u8, 102u8, 229u8, 189u8, 158u8, 142u8, 95u8, 235u8, 240u8, 49u8,
                            150u8, 76u8, 78u8, 137u8, 126u8, 88u8, 183u8, 88u8, 231u8, 146u8,
                            234u8, 43u8,
                        ],
                    )
                }

                /// See [`Pallet::kill_storage`].
                pub fn kill_storage(
                    &self,
                    keys: types::kill_storage::Keys,
                ) -> ::subxt::tx::Payload<types::KillStorage> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_storage",
                        types::KillStorage {
                            keys,
                        },
                        [
                            73u8, 63u8, 196u8, 36u8, 144u8, 114u8, 34u8, 213u8, 108u8, 93u8, 209u8,
                            234u8, 153u8, 185u8, 33u8, 91u8, 187u8, 195u8, 223u8, 130u8, 58u8,
                            156u8, 63u8, 47u8, 228u8, 249u8, 216u8, 139u8, 143u8, 177u8, 41u8,
                            35u8,
                        ],
                    )
                }

                /// See [`Pallet::kill_prefix`].
                pub fn kill_prefix(
                    &self,
                    prefix: types::kill_prefix::Prefix,
                    subkeys: types::kill_prefix::Subkeys,
                ) -> ::subxt::tx::Payload<types::KillPrefix> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "kill_prefix",
                        types::KillPrefix {
                            prefix,
                            subkeys,
                        },
                        [
                            184u8, 57u8, 139u8, 24u8, 208u8, 87u8, 108u8, 215u8, 198u8, 189u8,
                            175u8, 242u8, 167u8, 215u8, 97u8, 63u8, 110u8, 166u8, 238u8, 98u8,
                            67u8, 236u8, 111u8, 110u8, 234u8, 81u8, 102u8, 5u8, 182u8, 5u8, 214u8,
                            85u8,
                        ],
                    )
                }

                /// See [`Pallet::remark_with_event`].
                pub fn remark_with_event(
                    &self,
                    remark: types::remark_with_event::Remark,
                ) -> ::subxt::tx::Payload<types::RemarkWithEvent> {
                    ::subxt::tx::Payload::new_static(
                        "System",
                        "remark_with_event",
                        types::RemarkWithEvent {
                            remark,
                        },
                        [
                            120u8, 120u8, 153u8, 92u8, 184u8, 85u8, 34u8, 2u8, 174u8, 206u8, 105u8,
                            228u8, 233u8, 130u8, 80u8, 246u8, 228u8, 59u8, 234u8, 240u8, 4u8, 49u8,
                            147u8, 170u8, 115u8, 91u8, 149u8, 200u8, 228u8, 181u8, 8u8, 154u8,
                        ],
                    )
                }
            }
        }
        /// Event for the System pallet.
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An extrinsic completed successfully.
            pub struct ExtrinsicSuccess {
                pub dispatch_info: extrinsic_success::DispatchInfo,
            }
            pub mod extrinsic_success {
                use super::runtime_types;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::events::StaticEvent for ExtrinsicSuccess {
                const EVENT: &'static str = "ExtrinsicSuccess";
                const PALLET: &'static str = "System";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An extrinsic failed.
            pub struct ExtrinsicFailed {
                pub dispatch_error: extrinsic_failed::DispatchError,
                pub dispatch_info: extrinsic_failed::DispatchInfo,
            }
            pub mod extrinsic_failed {
                use super::runtime_types;
                pub type DispatchError = runtime_types::sp_runtime::DispatchError;
                pub type DispatchInfo = runtime_types::frame_support::dispatch::DispatchInfo;
            }
            impl ::subxt::events::StaticEvent for ExtrinsicFailed {
                const EVENT: &'static str = "ExtrinsicFailed";
                const PALLET: &'static str = "System";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// `:code` was updated.
            pub struct CodeUpdated;
            impl ::subxt::events::StaticEvent for CodeUpdated {
                const EVENT: &'static str = "CodeUpdated";
                const PALLET: &'static str = "System";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A new account was created.
            pub struct NewAccount {
                pub account: new_account::Account,
            }
            pub mod new_account {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for NewAccount {
                const EVENT: &'static str = "NewAccount";
                const PALLET: &'static str = "System";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An account was reaped.
            pub struct KilledAccount {
                pub account: killed_account::Account,
            }
            pub mod killed_account {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for KilledAccount {
                const EVENT: &'static str = "KilledAccount";
                const PALLET: &'static str = "System";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// On on-chain remark happened.
            pub struct Remarked {
                pub sender: remarked::Sender,
                pub hash: remarked::Hash,
            }
            pub mod remarked {
                use super::runtime_types;
                pub type Sender = ::subxt::utils::AccountId32;
                pub type Hash = ::subxt::utils::H256;
            }
            impl ::subxt::events::StaticEvent for Remarked {
                const EVENT: &'static str = "Remarked";
                const PALLET: &'static str = "System";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod extrinsic_count {
                    use super::runtime_types;
                    pub type ExtrinsicCount = ::core::primitive::u32;
                }
                pub mod block_weight {
                    use super::runtime_types;
                    pub type BlockWeight = runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::sp_weights::weight_v2::Weight,
                    >;
                }
                pub mod all_extrinsics_len {
                    use super::runtime_types;
                    pub type AllExtrinsicsLen = ::core::primitive::u32;
                }
                pub mod block_hash {
                    use super::runtime_types;
                    pub type BlockHash = ::subxt::utils::H256;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod extrinsic_data {
                    use super::runtime_types;
                    pub type ExtrinsicData = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod number {
                    use super::runtime_types;
                    pub type Number = ::core::primitive::u32;
                }
                pub mod parent_hash {
                    use super::runtime_types;
                    pub type ParentHash = ::subxt::utils::H256;
                }
                pub mod digest {
                    use super::runtime_types;
                    pub type Digest = runtime_types::sp_runtime::generic::digest::Digest;
                }
                pub mod events {
                    use super::runtime_types;
                    pub type Events = ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::nagara_core_runtime::RuntimeEvent,
                            ::subxt::utils::H256,
                        >,
                    >;
                }
                pub mod event_count {
                    use super::runtime_types;
                    pub type EventCount = ::core::primitive::u32;
                }
                pub mod event_topics {
                    use super::runtime_types;
                    pub type EventTopics =
                        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                    pub type Param0 = ::subxt::utils::H256;
                }
                pub mod last_runtime_upgrade {
                    use super::runtime_types;
                    pub type LastRuntimeUpgrade =
                        runtime_types::frame_system::LastRuntimeUpgradeInfo;
                }
                pub mod upgraded_to_u32_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToU32RefCount = ::core::primitive::bool;
                }
                pub mod upgraded_to_triple_ref_count {
                    use super::runtime_types;
                    pub type UpgradedToTripleRefCount = ::core::primitive::bool;
                }
                pub mod execution_phase {
                    use super::runtime_types;
                    pub type ExecutionPhase = runtime_types::frame_system::Phase;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// The full account information for a particular account ID.
                pub fn account_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        vec![],
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }

                /// The full account information for a particular account ID.
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            14u8, 233u8, 115u8, 214u8, 0u8, 109u8, 222u8, 121u8, 162u8, 65u8, 60u8,
                            175u8, 209u8, 79u8, 222u8, 124u8, 22u8, 235u8, 138u8, 176u8, 133u8,
                            124u8, 90u8, 158u8, 85u8, 45u8, 37u8, 174u8, 47u8, 79u8, 47u8, 166u8,
                        ],
                    )
                }

                /// Total extrinsics count for the current block.
                pub fn extrinsic_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::extrinsic_count::ExtrinsicCount,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicCount",
                        vec![],
                        [
                            102u8, 76u8, 236u8, 42u8, 40u8, 231u8, 33u8, 222u8, 123u8, 147u8,
                            153u8, 148u8, 234u8, 203u8, 181u8, 119u8, 6u8, 187u8, 177u8, 199u8,
                            120u8, 47u8, 137u8, 254u8, 96u8, 100u8, 165u8, 182u8, 249u8, 230u8,
                            159u8, 79u8,
                        ],
                    )
                }

                /// The current weight for the block.
                pub fn block_weight(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::block_weight::BlockWeight,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockWeight",
                        vec![],
                        [
                            158u8, 46u8, 228u8, 89u8, 210u8, 214u8, 84u8, 154u8, 50u8, 68u8, 63u8,
                            62u8, 43u8, 42u8, 99u8, 27u8, 54u8, 42u8, 146u8, 44u8, 241u8, 216u8,
                            229u8, 30u8, 216u8, 255u8, 165u8, 238u8, 181u8, 130u8, 36u8, 102u8,
                        ],
                    )
                }

                /// Total length (in bytes) for all extrinsics put together, for
                /// the current block.
                pub fn all_extrinsics_len(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::all_extrinsics_len::AllExtrinsicsLen,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "AllExtrinsicsLen",
                        vec![],
                        [
                            117u8, 86u8, 61u8, 243u8, 41u8, 51u8, 102u8, 214u8, 137u8, 100u8,
                            243u8, 185u8, 122u8, 174u8, 187u8, 117u8, 86u8, 189u8, 63u8, 135u8,
                            101u8, 218u8, 203u8, 201u8, 237u8, 254u8, 128u8, 183u8, 169u8, 221u8,
                            242u8, 65u8,
                        ],
                    )
                }

                /// Map of block numbers to block hashes.
                pub fn block_hash_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::block_hash::BlockHash,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        vec![],
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }

                /// Map of block numbers to block hashes.
                pub fn block_hash(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::block_hash::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::block_hash::BlockHash,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "BlockHash",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            217u8, 32u8, 215u8, 253u8, 24u8, 182u8, 207u8, 178u8, 157u8, 24u8,
                            103u8, 100u8, 195u8, 165u8, 69u8, 152u8, 112u8, 181u8, 56u8, 192u8,
                            164u8, 16u8, 20u8, 222u8, 28u8, 214u8, 144u8, 142u8, 146u8, 69u8,
                            202u8, 118u8,
                        ],
                    )
                }

                /// Extrinsics data for the current block (maps an extrinsic's
                /// index to its data).
                pub fn extrinsic_data_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::extrinsic_data::ExtrinsicData,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        vec![],
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }

                /// Extrinsics data for the current block (maps an extrinsic's
                /// index to its data).
                pub fn extrinsic_data(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::extrinsic_data::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::extrinsic_data::ExtrinsicData,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExtrinsicData",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            160u8, 180u8, 122u8, 18u8, 196u8, 26u8, 2u8, 37u8, 115u8, 232u8, 133u8,
                            220u8, 106u8, 245u8, 4u8, 129u8, 42u8, 84u8, 241u8, 45u8, 199u8, 179u8,
                            128u8, 61u8, 170u8, 137u8, 231u8, 156u8, 247u8, 57u8, 47u8, 38u8,
                        ],
                    )
                }

                /// The current block number being processed. Set by
                /// `execute_block`.
                pub fn number(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::number::Number,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Number",
                        vec![],
                        [
                            30u8, 194u8, 177u8, 90u8, 194u8, 232u8, 46u8, 180u8, 85u8, 129u8, 14u8,
                            9u8, 8u8, 8u8, 23u8, 95u8, 230u8, 5u8, 13u8, 105u8, 125u8, 2u8, 22u8,
                            200u8, 78u8, 93u8, 115u8, 28u8, 150u8, 113u8, 48u8, 53u8,
                        ],
                    )
                }

                /// Hash of the previous block.
                pub fn parent_hash(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::parent_hash::ParentHash,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ParentHash",
                        vec![],
                        [
                            26u8, 130u8, 11u8, 216u8, 155u8, 71u8, 128u8, 170u8, 30u8, 153u8, 21u8,
                            192u8, 62u8, 93u8, 137u8, 80u8, 120u8, 81u8, 202u8, 94u8, 248u8, 125u8,
                            71u8, 82u8, 141u8, 229u8, 32u8, 56u8, 73u8, 50u8, 101u8, 78u8,
                        ],
                    )
                }

                /// Digest of the current block, also part of the block header.
                pub fn digest(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::digest::Digest,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Digest",
                        vec![],
                        [
                            61u8, 64u8, 237u8, 91u8, 145u8, 232u8, 17u8, 254u8, 181u8, 16u8, 234u8,
                            91u8, 51u8, 140u8, 254u8, 131u8, 98u8, 135u8, 21u8, 37u8, 251u8, 20u8,
                            58u8, 92u8, 123u8, 141u8, 14u8, 227u8, 146u8, 46u8, 222u8, 117u8,
                        ],
                    )
                }

                /// Events deposited for the current block.
                ///
                /// NOTE: The item is unbound and should therefore never be read
                /// on chain.
                /// It could otherwise inflate the PoV size of a block.
                ///
                /// Events have a large in-memory size. Box the events to not go
                /// out-of-memory
                /// just in case someone still reads them from within the
                /// runtime.
                pub fn events(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::events::Events,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "Events",
                        vec![],
                        [
                            135u8, 106u8, 90u8, 104u8, 182u8, 189u8, 111u8, 132u8, 1u8, 156u8,
                            28u8, 204u8, 164u8, 128u8, 142u8, 118u8, 35u8, 52u8, 99u8, 6u8, 111u8,
                            236u8, 132u8, 32u8, 134u8, 180u8, 214u8, 221u8, 168u8, 25u8, 137u8,
                            10u8,
                        ],
                    )
                }

                /// The number of events in the `Events<T>` list.
                pub fn event_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::event_count::EventCount,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventCount",
                        vec![],
                        [
                            175u8, 24u8, 252u8, 184u8, 210u8, 167u8, 146u8, 143u8, 164u8, 80u8,
                            151u8, 205u8, 189u8, 189u8, 55u8, 220u8, 47u8, 101u8, 181u8, 33u8,
                            254u8, 131u8, 13u8, 143u8, 3u8, 244u8, 245u8, 45u8, 2u8, 210u8, 79u8,
                            133u8,
                        ],
                    )
                }

                /// Mapping between a topic (represented by T::Hash) and a
                /// vector of indexes
                /// of events in the `<Events<T>>` list.
                ///
                /// All topic vectors have deterministic storage locations
                /// depending on the topic. This
                /// allows light-clients to leverage the changes trie storage
                /// tracking mechanism and
                /// in case of changes fetch the list of events of interest.
                ///
                /// The value has the type `(BlockNumberFor<T>, EventIndex)`
                /// because if we used only just
                /// the `EventIndex` then in case if the topic has the same
                /// contents on the next block
                /// no notification will be triggered thus the event might be
                /// lost.
                pub fn event_topics_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::event_topics::EventTopics,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        vec![],
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }

                /// Mapping between a topic (represented by T::Hash) and a
                /// vector of indexes
                /// of events in the `<Events<T>>` list.
                ///
                /// All topic vectors have deterministic storage locations
                /// depending on the topic. This
                /// allows light-clients to leverage the changes trie storage
                /// tracking mechanism and
                /// in case of changes fetch the list of events of interest.
                ///
                /// The value has the type `(BlockNumberFor<T>, EventIndex)`
                /// because if we used only just
                /// the `EventIndex` then in case if the topic has the same
                /// contents on the next block
                /// no notification will be triggered thus the event might be
                /// lost.
                pub fn event_topics(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::event_topics::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::event_topics::EventTopics,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "EventTopics",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            40u8, 225u8, 14u8, 75u8, 44u8, 176u8, 76u8, 34u8, 143u8, 107u8, 69u8,
                            133u8, 114u8, 13u8, 172u8, 250u8, 141u8, 73u8, 12u8, 65u8, 217u8, 63u8,
                            120u8, 241u8, 48u8, 106u8, 143u8, 161u8, 128u8, 100u8, 166u8, 59u8,
                        ],
                    )
                }

                /// Stores the `spec_version` and `spec_name` of when the last
                /// runtime upgrade happened.
                pub fn last_runtime_upgrade(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::last_runtime_upgrade::LastRuntimeUpgrade,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "LastRuntimeUpgrade",
                        vec![],
                        [
                            137u8, 29u8, 175u8, 75u8, 197u8, 208u8, 91u8, 207u8, 156u8, 87u8,
                            148u8, 68u8, 91u8, 140u8, 22u8, 233u8, 1u8, 229u8, 56u8, 34u8, 40u8,
                            194u8, 253u8, 30u8, 163u8, 39u8, 54u8, 209u8, 13u8, 27u8, 139u8, 184u8,
                        ],
                    )
                }

                /// True if we have upgraded so that `type RefCount` is `u32`.
                /// False (default) if not.
                pub fn upgraded_to_u32_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::upgraded_to_u32_ref_count::UpgradedToU32RefCount,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToU32RefCount",
                        vec![],
                        [
                            229u8, 73u8, 9u8, 132u8, 186u8, 116u8, 151u8, 171u8, 145u8, 29u8, 34u8,
                            130u8, 52u8, 146u8, 124u8, 175u8, 79u8, 189u8, 147u8, 230u8, 234u8,
                            107u8, 124u8, 31u8, 2u8, 22u8, 86u8, 190u8, 4u8, 147u8, 50u8, 245u8,
                        ],
                    )
                }

                /// True if we have upgraded so that AccountInfo contains three
                /// types of `RefCount`. False
                /// (default) if not.
                pub fn upgraded_to_triple_ref_count(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::upgraded_to_triple_ref_count::UpgradedToTripleRefCount,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "UpgradedToTripleRefCount",
                        vec![],
                        [
                            97u8, 66u8, 124u8, 243u8, 27u8, 167u8, 147u8, 81u8, 254u8, 201u8,
                            101u8, 24u8, 40u8, 231u8, 14u8, 179u8, 154u8, 163u8, 71u8, 81u8, 185u8,
                            167u8, 82u8, 254u8, 189u8, 3u8, 101u8, 207u8, 206u8, 194u8, 155u8,
                            151u8,
                        ],
                    )
                }

                /// The execution phase of the block.
                pub fn execution_phase(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::execution_phase::ExecutionPhase,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "System",
                        "ExecutionPhase",
                        vec![],
                        [
                            191u8, 129u8, 100u8, 134u8, 126u8, 116u8, 154u8, 203u8, 220u8, 200u8,
                            0u8, 26u8, 161u8, 250u8, 133u8, 205u8, 146u8, 24u8, 5u8, 156u8, 158u8,
                            35u8, 36u8, 253u8, 52u8, 235u8, 86u8, 167u8, 35u8, 100u8, 119u8, 27u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// Block & extrinsics weights: base values and limits.
                pub fn block_weights(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockWeights>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockWeights",
                        [
                            176u8, 124u8, 225u8, 136u8, 25u8, 73u8, 247u8, 33u8, 82u8, 206u8, 85u8,
                            190u8, 127u8, 102u8, 71u8, 11u8, 185u8, 8u8, 58u8, 0u8, 94u8, 55u8,
                            163u8, 177u8, 104u8, 59u8, 60u8, 136u8, 246u8, 116u8, 0u8, 239u8,
                        ],
                    )
                }

                /// The maximum length of a block (in bytes).
                pub fn block_length(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::frame_system::limits::BlockLength>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockLength",
                        [
                            23u8, 242u8, 225u8, 39u8, 225u8, 67u8, 152u8, 41u8, 155u8, 104u8, 68u8,
                            229u8, 185u8, 133u8, 10u8, 143u8, 184u8, 152u8, 234u8, 44u8, 140u8,
                            96u8, 166u8, 235u8, 162u8, 160u8, 72u8, 7u8, 35u8, 194u8, 3u8, 37u8,
                        ],
                    )
                }

                /// Maximum number of block number to block hash mappings to
                /// keep (oldest pruned first).
                pub fn block_hash_count(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "BlockHashCount",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// The weight of runtime database operations the runtime can
                /// invoke.
                pub fn db_weight(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_weights::RuntimeDbWeight>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "DbWeight",
                        [
                            42u8, 43u8, 178u8, 142u8, 243u8, 203u8, 60u8, 173u8, 118u8, 111u8,
                            200u8, 170u8, 102u8, 70u8, 237u8, 187u8, 198u8, 120u8, 153u8, 232u8,
                            183u8, 76u8, 74u8, 10u8, 70u8, 243u8, 14u8, 218u8, 213u8, 126u8, 29u8,
                            177u8,
                        ],
                    )
                }

                /// Get the chain's current version.
                pub fn version(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::sp_version::RuntimeVersion>
                {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "Version",
                        [
                            219u8, 45u8, 162u8, 245u8, 177u8, 246u8, 48u8, 126u8, 191u8, 157u8,
                            228u8, 83u8, 111u8, 133u8, 183u8, 13u8, 148u8, 108u8, 92u8, 102u8,
                            72u8, 205u8, 74u8, 242u8, 233u8, 79u8, 20u8, 170u8, 72u8, 202u8, 158u8,
                            165u8,
                        ],
                    )
                }

                /// The designated SS58 prefix of this chain.
                ///
                /// This replaces the "ss58Format" property declared in the
                /// chain spec. Reason is
                /// that the runtime should know about the prefix in order to
                /// make use of it as
                /// an identifier of the chain.
                pub fn ss58_prefix(&self) -> ::subxt::constants::Address<::core::primitive::u16> {
                    ::subxt::constants::Address::new_static(
                        "System",
                        "SS58Prefix",
                        [
                            116u8, 33u8, 2u8, 170u8, 181u8, 147u8, 171u8, 169u8, 167u8, 227u8,
                            41u8, 144u8, 11u8, 236u8, 82u8, 100u8, 74u8, 60u8, 184u8, 72u8, 169u8,
                            90u8, 208u8, 135u8, 15u8, 117u8, 10u8, 123u8, 128u8, 193u8, 29u8, 70u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod timestamp {
        use super::root_mod;
        use super::runtime_types;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::pallet_timestamp::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set`].
                pub struct Set {
                    #[codec(compact)]
                    pub now: set::Now,
                }
                pub mod set {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                impl ::subxt::blocks::StaticExtrinsic for Set {
                    const CALL: &'static str = "set";
                    const PALLET: &'static str = "Timestamp";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::set`].
                pub fn set(&self, now: types::set::Now) -> ::subxt::tx::Payload<types::Set> {
                    ::subxt::tx::Payload::new_static(
                        "Timestamp",
                        "set",
                        types::Set {
                            now,
                        },
                        [
                            37u8, 95u8, 49u8, 218u8, 24u8, 22u8, 0u8, 95u8, 72u8, 35u8, 155u8,
                            199u8, 213u8, 54u8, 207u8, 22u8, 185u8, 193u8, 221u8, 70u8, 18u8,
                            200u8, 4u8, 231u8, 195u8, 173u8, 6u8, 122u8, 11u8, 203u8, 231u8, 227u8,
                        ],
                    )
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod now {
                    use super::runtime_types;
                    pub type Now = ::core::primitive::u64;
                }
                pub mod did_update {
                    use super::runtime_types;
                    pub type DidUpdate = ::core::primitive::bool;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// Current time for the current block.
                pub fn now(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::now::Now,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "Now",
                        vec![],
                        [
                            44u8, 50u8, 80u8, 30u8, 195u8, 146u8, 123u8, 238u8, 8u8, 163u8, 187u8,
                            92u8, 61u8, 39u8, 51u8, 29u8, 173u8, 169u8, 217u8, 158u8, 85u8, 187u8,
                            141u8, 26u8, 12u8, 115u8, 51u8, 11u8, 200u8, 244u8, 138u8, 152u8,
                        ],
                    )
                }

                /// Did the timestamp get updated in this block?
                pub fn did_update(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::did_update::DidUpdate,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Timestamp",
                        "DidUpdate",
                        vec![],
                        [
                            229u8, 175u8, 246u8, 102u8, 237u8, 158u8, 212u8, 229u8, 238u8, 214u8,
                            205u8, 160u8, 164u8, 252u8, 195u8, 75u8, 139u8, 110u8, 22u8, 34u8,
                            248u8, 204u8, 107u8, 46u8, 20u8, 200u8, 238u8, 167u8, 71u8, 41u8,
                            214u8, 140u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// The minimum period between blocks. Beware that this is
                /// different to the *expected*
                /// period that the block production apparatus provides. Your
                /// chosen consensus system will
                /// generally work with this to determine a sensible block time.
                /// e.g. For Aura, it will be
                /// double this period on default settings.
                pub fn minimum_period(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Timestamp",
                        "MinimumPeriod",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod balances {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::pallet_balances::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::pallet_balances::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::transfer_allow_death`].
                pub struct TransferAllowDeath {
                    pub dest: transfer_allow_death::Dest,
                    #[codec(compact)]
                    pub value: transfer_allow_death::Value,
                }
                pub mod transfer_allow_death {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAllowDeath {
                    const CALL: &'static str = "transfer_allow_death";
                    const PALLET: &'static str = "Balances";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_balance_deprecated`].
                pub struct SetBalanceDeprecated {
                    pub who: set_balance_deprecated::Who,
                    #[codec(compact)]
                    pub new_free: set_balance_deprecated::NewFree,
                    #[codec(compact)]
                    pub old_reserved: set_balance_deprecated::OldReserved,
                }
                pub mod set_balance_deprecated {
                    use super::runtime_types;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type NewFree = ::core::primitive::u128;
                    pub type OldReserved = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetBalanceDeprecated {
                    const CALL: &'static str = "set_balance_deprecated";
                    const PALLET: &'static str = "Balances";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_transfer`].
                pub struct ForceTransfer {
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub value: force_transfer::Value,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Source = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
                    const CALL: &'static str = "force_transfer";
                    const PALLET: &'static str = "Balances";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::transfer_keep_alive`].
                pub struct TransferKeepAlive {
                    pub dest: transfer_keep_alive::Dest,
                    #[codec(compact)]
                    pub value: transfer_keep_alive::Value,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
                    const CALL: &'static str = "transfer_keep_alive";
                    const PALLET: &'static str = "Balances";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::transfer_all`].
                pub struct TransferAll {
                    pub dest: transfer_all::Dest,
                    pub keep_alive: transfer_all::KeepAlive,
                }
                pub mod transfer_all {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type KeepAlive = ::core::primitive::bool;
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferAll {
                    const CALL: &'static str = "transfer_all";
                    const PALLET: &'static str = "Balances";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_unreserve`].
                pub struct ForceUnreserve {
                    pub who: force_unreserve::Who,
                    pub amount: force_unreserve::Amount,
                }
                pub mod force_unreserve {
                    use super::runtime_types;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceUnreserve {
                    const CALL: &'static str = "force_unreserve";
                    const PALLET: &'static str = "Balances";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::upgrade_accounts`].
                pub struct UpgradeAccounts {
                    pub who: upgrade_accounts::Who,
                }
                pub mod upgrade_accounts {
                    use super::runtime_types;
                    pub type Who = ::std::vec::Vec<::subxt::utils::AccountId32>;
                }
                impl ::subxt::blocks::StaticExtrinsic for UpgradeAccounts {
                    const CALL: &'static str = "upgrade_accounts";
                    const PALLET: &'static str = "Balances";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::transfer`].
                pub struct Transfer {
                    pub dest: transfer::Dest,
                    #[codec(compact)]
                    pub value: transfer::Value,
                }
                pub mod transfer {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Value = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for Transfer {
                    const CALL: &'static str = "transfer";
                    const PALLET: &'static str = "Balances";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_set_balance`].
                pub struct ForceSetBalance {
                    pub who: force_set_balance::Who,
                    #[codec(compact)]
                    pub new_free: force_set_balance::NewFree,
                }
                pub mod force_set_balance {
                    use super::runtime_types;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type NewFree = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceSetBalance {
                    const CALL: &'static str = "force_set_balance";
                    const PALLET: &'static str = "Balances";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::transfer_allow_death`].
                pub fn transfer_allow_death(
                    &self,
                    dest: types::transfer_allow_death::Dest,
                    value: types::transfer_allow_death::Value,
                ) -> ::subxt::tx::Payload<types::TransferAllowDeath> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_allow_death",
                        types::TransferAllowDeath {
                            dest,
                            value,
                        },
                        [
                            51u8, 166u8, 195u8, 10u8, 139u8, 218u8, 55u8, 130u8, 6u8, 194u8, 35u8,
                            140u8, 27u8, 205u8, 214u8, 222u8, 102u8, 43u8, 143u8, 145u8, 86u8,
                            219u8, 210u8, 147u8, 13u8, 39u8, 51u8, 21u8, 237u8, 179u8, 132u8,
                            130u8,
                        ],
                    )
                }

                /// See [`Pallet::set_balance_deprecated`].
                pub fn set_balance_deprecated(
                    &self,
                    who: types::set_balance_deprecated::Who,
                    new_free: types::set_balance_deprecated::NewFree,
                    old_reserved: types::set_balance_deprecated::OldReserved,
                ) -> ::subxt::tx::Payload<types::SetBalanceDeprecated> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "set_balance_deprecated",
                        types::SetBalanceDeprecated {
                            who,
                            new_free,
                            old_reserved,
                        },
                        [
                            125u8, 171u8, 21u8, 186u8, 108u8, 185u8, 241u8, 145u8, 125u8, 8u8,
                            12u8, 42u8, 96u8, 114u8, 80u8, 80u8, 227u8, 76u8, 20u8, 208u8, 93u8,
                            219u8, 36u8, 50u8, 209u8, 155u8, 70u8, 45u8, 6u8, 57u8, 156u8, 77u8,
                        ],
                    )
                }

                /// See [`Pallet::force_transfer`].
                pub fn force_transfer(
                    &self,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    value: types::force_transfer::Value,
                ) -> ::subxt::tx::Payload<types::ForceTransfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_transfer",
                        types::ForceTransfer {
                            source,
                            dest,
                            value,
                        },
                        [
                            154u8, 93u8, 222u8, 27u8, 12u8, 248u8, 63u8, 213u8, 224u8, 86u8, 250u8,
                            153u8, 249u8, 102u8, 83u8, 160u8, 79u8, 125u8, 105u8, 222u8, 77u8,
                            180u8, 90u8, 105u8, 81u8, 217u8, 60u8, 25u8, 213u8, 51u8, 185u8, 96u8,
                        ],
                    )
                }

                /// See [`Pallet::transfer_keep_alive`].
                pub fn transfer_keep_alive(
                    &self,
                    dest: types::transfer_keep_alive::Dest,
                    value: types::transfer_keep_alive::Value,
                ) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_keep_alive",
                        types::TransferKeepAlive {
                            dest,
                            value,
                        },
                        [
                            245u8, 14u8, 190u8, 193u8, 32u8, 210u8, 74u8, 92u8, 25u8, 182u8, 76u8,
                            55u8, 247u8, 83u8, 114u8, 75u8, 143u8, 236u8, 117u8, 25u8, 54u8, 157u8,
                            208u8, 207u8, 233u8, 89u8, 70u8, 161u8, 235u8, 242u8, 222u8, 59u8,
                        ],
                    )
                }

                /// See [`Pallet::transfer_all`].
                pub fn transfer_all(
                    &self,
                    dest: types::transfer_all::Dest,
                    keep_alive: types::transfer_all::KeepAlive,
                ) -> ::subxt::tx::Payload<types::TransferAll> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer_all",
                        types::TransferAll {
                            dest,
                            keep_alive,
                        },
                        [
                            105u8, 132u8, 49u8, 144u8, 195u8, 250u8, 34u8, 46u8, 213u8, 248u8,
                            112u8, 188u8, 81u8, 228u8, 136u8, 18u8, 67u8, 172u8, 37u8, 38u8, 238u8,
                            9u8, 34u8, 15u8, 67u8, 34u8, 148u8, 195u8, 223u8, 29u8, 154u8, 6u8,
                        ],
                    )
                }

                /// See [`Pallet::force_unreserve`].
                pub fn force_unreserve(
                    &self,
                    who: types::force_unreserve::Who,
                    amount: types::force_unreserve::Amount,
                ) -> ::subxt::tx::Payload<types::ForceUnreserve> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_unreserve",
                        types::ForceUnreserve {
                            who,
                            amount,
                        },
                        [
                            142u8, 151u8, 64u8, 205u8, 46u8, 64u8, 62u8, 122u8, 108u8, 49u8, 223u8,
                            140u8, 120u8, 153u8, 35u8, 165u8, 187u8, 38u8, 157u8, 200u8, 123u8,
                            199u8, 198u8, 168u8, 208u8, 159u8, 39u8, 134u8, 92u8, 103u8, 84u8,
                            171u8,
                        ],
                    )
                }

                /// See [`Pallet::upgrade_accounts`].
                pub fn upgrade_accounts(
                    &self,
                    who: types::upgrade_accounts::Who,
                ) -> ::subxt::tx::Payload<types::UpgradeAccounts> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "upgrade_accounts",
                        types::UpgradeAccounts {
                            who,
                        },
                        [
                            66u8, 200u8, 179u8, 104u8, 65u8, 2u8, 101u8, 56u8, 130u8, 161u8, 224u8,
                            233u8, 255u8, 124u8, 70u8, 122u8, 8u8, 49u8, 103u8, 178u8, 68u8, 47u8,
                            214u8, 166u8, 217u8, 116u8, 178u8, 50u8, 212u8, 164u8, 98u8, 226u8,
                        ],
                    )
                }

                /// See [`Pallet::transfer`].
                pub fn transfer(
                    &self,
                    dest: types::transfer::Dest,
                    value: types::transfer::Value,
                ) -> ::subxt::tx::Payload<types::Transfer> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "transfer",
                        types::Transfer {
                            dest,
                            value,
                        },
                        [
                            154u8, 145u8, 140u8, 54u8, 50u8, 123u8, 225u8, 249u8, 200u8, 217u8,
                            172u8, 110u8, 233u8, 198u8, 77u8, 198u8, 211u8, 89u8, 8u8, 13u8, 240u8,
                            94u8, 28u8, 13u8, 242u8, 217u8, 168u8, 23u8, 106u8, 254u8, 249u8,
                            120u8,
                        ],
                    )
                }

                /// See [`Pallet::force_set_balance`].
                pub fn force_set_balance(
                    &self,
                    who: types::force_set_balance::Who,
                    new_free: types::force_set_balance::NewFree,
                ) -> ::subxt::tx::Payload<types::ForceSetBalance> {
                    ::subxt::tx::Payload::new_static(
                        "Balances",
                        "force_set_balance",
                        types::ForceSetBalance {
                            who,
                            new_free,
                        },
                        [
                            114u8, 229u8, 59u8, 204u8, 180u8, 83u8, 17u8, 4u8, 59u8, 4u8, 55u8,
                            39u8, 151u8, 196u8, 124u8, 60u8, 209u8, 65u8, 193u8, 11u8, 44u8, 164u8,
                            116u8, 93u8, 169u8, 30u8, 199u8, 165u8, 55u8, 231u8, 223u8, 43u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An account was created with some free balance.
            pub struct Endowed {
                pub account: endowed::Account,
                pub free_balance: endowed::FreeBalance,
            }
            pub mod endowed {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
                pub type FreeBalance = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Endowed {
                const EVENT: &'static str = "Endowed";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An account was removed whose balance was non-zero but below
            /// ExistentialDeposit,
            /// resulting in an outright loss.
            pub struct DustLost {
                pub account: dust_lost::Account,
                pub amount: dust_lost::Amount,
            }
            pub mod dust_lost {
                use super::runtime_types;
                pub type Account = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for DustLost {
                const EVENT: &'static str = "DustLost";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Transfer succeeded.
            pub struct Transfer {
                pub from: transfer::From,
                pub to: transfer::To,
                pub amount: transfer::Amount,
            }
            pub mod transfer {
                use super::runtime_types;
                pub type From = ::subxt::utils::AccountId32;
                pub type To = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Transfer {
                const EVENT: &'static str = "Transfer";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A balance was set by root.
            pub struct BalanceSet {
                pub who: balance_set::Who,
                pub free: balance_set::Free,
            }
            pub mod balance_set {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Free = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for BalanceSet {
                const EVENT: &'static str = "BalanceSet";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some balance was reserved (moved from free to reserved).
            pub struct Reserved {
                pub who: reserved::Who,
                pub amount: reserved::Amount,
            }
            pub mod reserved {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Reserved {
                const EVENT: &'static str = "Reserved";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some balance was unreserved (moved from reserved to free).
            pub struct Unreserved {
                pub who: unreserved::Who,
                pub amount: unreserved::Amount,
            }
            pub mod unreserved {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Unreserved {
                const EVENT: &'static str = "Unreserved";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some balance was moved from the reserve of the first account to
            /// the second account.
            /// Final argument indicates the destination balance type.
            pub struct ReserveRepatriated {
                pub from: reserve_repatriated::From,
                pub to: reserve_repatriated::To,
                pub amount: reserve_repatriated::Amount,
                pub destination_status: reserve_repatriated::DestinationStatus,
            }
            pub mod reserve_repatriated {
                use super::runtime_types;
                pub type From = ::subxt::utils::AccountId32;
                pub type To = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
                pub type DestinationStatus =
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus;
            }
            impl ::subxt::events::StaticEvent for ReserveRepatriated {
                const EVENT: &'static str = "ReserveRepatriated";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some amount was deposited (e.g. for transaction fees).
            pub struct Deposit {
                pub who: deposit::Who,
                pub amount: deposit::Amount,
            }
            pub mod deposit {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Deposit {
                const EVENT: &'static str = "Deposit";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some amount was withdrawn from the account (e.g. for transaction
            /// fees).
            pub struct Withdraw {
                pub who: withdraw::Who,
                pub amount: withdraw::Amount,
            }
            pub mod withdraw {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Withdraw {
                const EVENT: &'static str = "Withdraw";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some amount was removed from the account (e.g. for misbehavior).
            pub struct Slashed {
                pub who: slashed::Who,
                pub amount: slashed::Amount,
            }
            pub mod slashed {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Slashed {
                const EVENT: &'static str = "Slashed";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some amount was minted into an account.
            pub struct Minted {
                pub who: minted::Who,
                pub amount: minted::Amount,
            }
            pub mod minted {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Minted {
                const EVENT: &'static str = "Minted";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some amount was burned from an account.
            pub struct Burned {
                pub who: burned::Who,
                pub amount: burned::Amount,
            }
            pub mod burned {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Burned {
                const EVENT: &'static str = "Burned";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some amount was suspended from an account (it can be restored
            /// later).
            pub struct Suspended {
                pub who: suspended::Who,
                pub amount: suspended::Amount,
            }
            pub mod suspended {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Suspended {
                const EVENT: &'static str = "Suspended";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some amount was restored into an account.
            pub struct Restored {
                pub who: restored::Who,
                pub amount: restored::Amount,
            }
            pub mod restored {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Restored {
                const EVENT: &'static str = "Restored";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An account was upgraded.
            pub struct Upgraded {
                pub who: upgraded::Who,
            }
            pub mod upgraded {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Upgraded {
                const EVENT: &'static str = "Upgraded";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Total issuance was increased by `amount`, creating a credit to
            /// be balanced.
            pub struct Issued {
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Issued {
                const EVENT: &'static str = "Issued";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Total issuance was decreased by `amount`, creating a debt to be
            /// balanced.
            pub struct Rescinded {
                pub amount: rescinded::Amount,
            }
            pub mod rescinded {
                use super::runtime_types;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Rescinded {
                const EVENT: &'static str = "Rescinded";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some balance was locked.
            pub struct Locked {
                pub who: locked::Who,
                pub amount: locked::Amount,
            }
            pub mod locked {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Locked {
                const EVENT: &'static str = "Locked";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some balance was unlocked.
            pub struct Unlocked {
                pub who: unlocked::Who,
                pub amount: unlocked::Amount,
            }
            pub mod unlocked {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Unlocked {
                const EVENT: &'static str = "Unlocked";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some balance was frozen.
            pub struct Frozen {
                pub who: frozen::Who,
                pub amount: frozen::Amount,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Frozen {
                const EVENT: &'static str = "Frozen";
                const PALLET: &'static str = "Balances";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some balance was thawed.
            pub struct Thawed {
                pub who: thawed::Who,
                pub amount: thawed::Amount,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Thawed {
                const EVENT: &'static str = "Thawed";
                const PALLET: &'static str = "Balances";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod total_issuance {
                    use super::runtime_types;
                    pub type TotalIssuance = ::core::primitive::u128;
                }
                pub mod inactive_issuance {
                    use super::runtime_types;
                    pub type InactiveIssuance = ::core::primitive::u128;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account =
                        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod locks {
                    use super::runtime_types;
                    pub type Locks =
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::pallet_balances::types::BalanceLock<
                                ::core::primitive::u128,
                            >,
                        >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod reserves {
                    use super::runtime_types;
                    pub type Reserves = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::ReserveData<
                            [::core::primitive::u8; 24usize],
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod holds {
                    use super::runtime_types;
                    pub type Holds = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            runtime_types::nagara_core_runtime::RuntimeHoldReason,
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod freezes {
                    use super::runtime_types;
                    pub type Freezes = runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::pallet_balances::types::IdAmount<
                            runtime_types::nagara_core_runtime::RuntimeFreezeReason,
                            ::core::primitive::u128,
                        >,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// The total units issued in the system.
                pub fn total_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::total_issuance::TotalIssuance,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "TotalIssuance",
                        vec![],
                        [
                            116u8, 70u8, 119u8, 194u8, 69u8, 37u8, 116u8, 206u8, 171u8, 70u8,
                            171u8, 210u8, 226u8, 111u8, 184u8, 204u8, 206u8, 11u8, 68u8, 72u8,
                            255u8, 19u8, 194u8, 11u8, 27u8, 194u8, 81u8, 204u8, 59u8, 224u8, 202u8,
                            185u8,
                        ],
                    )
                }

                /// The total units of outstanding deactivated balance in the
                /// system.
                pub fn inactive_issuance(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::inactive_issuance::InactiveIssuance,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "InactiveIssuance",
                        vec![],
                        [
                            212u8, 185u8, 19u8, 50u8, 250u8, 72u8, 173u8, 50u8, 4u8, 104u8, 161u8,
                            249u8, 77u8, 247u8, 204u8, 248u8, 11u8, 18u8, 57u8, 4u8, 82u8, 110u8,
                            30u8, 216u8, 16u8, 37u8, 87u8, 67u8, 189u8, 235u8, 214u8, 155u8,
                        ],
                    )
                }

                /// The Balances pallet example of storing the balance of an
                /// account.
                ///
                /// # Example
                ///
                /// ```nocompile
                ///  impl pallet_balances::Config for Runtime {
                ///    type AccountStore =
                /// StorageMapShim<Self::Account<Runtime>,
                /// frame_system::Provider<Runtime>, AccountId,
                /// Self::AccountData<Balance>>
                ///  }
                /// ```
                ///
                /// You can also store the balance of an account in the `System`
                /// pallet.
                ///
                /// # Example
                ///
                /// ```nocompile
                ///  impl pallet_balances::Config for Runtime {
                ///   type AccountStore = System
                ///  }
                /// ```
                ///
                /// But this comes with tradeoffs, storing account balances in
                /// the system pallet stores
                /// `frame_system` data alongside the account data contrary to
                /// storing account balances in the
                /// `Balances` pallet, which uses a `StorageMap` to store
                /// balances data only.
                /// NOTE: This is only used in the case that this pallet is used
                /// to store balances.
                pub fn account_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        vec![],
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }

                /// The Balances pallet example of storing the balance of an
                /// account.
                ///
                /// # Example
                ///
                /// ```nocompile
                ///  impl pallet_balances::Config for Runtime {
                ///    type AccountStore =
                /// StorageMapShim<Self::Account<Runtime>,
                /// frame_system::Provider<Runtime>, AccountId,
                /// Self::AccountData<Balance>>
                ///  }
                /// ```
                ///
                /// You can also store the balance of an account in the `System`
                /// pallet.
                ///
                /// # Example
                ///
                /// ```nocompile
                ///  impl pallet_balances::Config for Runtime {
                ///   type AccountStore = System
                ///  }
                /// ```
                ///
                /// But this comes with tradeoffs, storing account balances in
                /// the system pallet stores
                /// `frame_system` data alongside the account data contrary to
                /// storing account balances in the
                /// `Balances` pallet, which uses a `StorageMap` to store
                /// balances data only.
                /// NOTE: This is only used in the case that this pallet is used
                /// to store balances.
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            213u8, 38u8, 200u8, 69u8, 218u8, 0u8, 112u8, 181u8, 160u8, 23u8, 96u8,
                            90u8, 3u8, 88u8, 126u8, 22u8, 103u8, 74u8, 64u8, 69u8, 29u8, 247u8,
                            18u8, 17u8, 234u8, 143u8, 189u8, 22u8, 247u8, 194u8, 154u8, 249u8,
                        ],
                    )
                }

                /// Any liquidity locks on some account balances.
                /// NOTE: Should only be accessed when setting, changing and
                /// freeing a lock.
                pub fn locks_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::locks::Locks,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        vec![],
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }

                /// Any liquidity locks on some account balances.
                /// NOTE: Should only be accessed when setting, changing and
                /// freeing a lock.
                pub fn locks(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::locks::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::locks::Locks,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Locks",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            10u8, 223u8, 55u8, 0u8, 249u8, 69u8, 168u8, 41u8, 75u8, 35u8, 120u8,
                            167u8, 18u8, 132u8, 9u8, 20u8, 91u8, 51u8, 27u8, 69u8, 136u8, 187u8,
                            13u8, 220u8, 163u8, 122u8, 26u8, 141u8, 174u8, 249u8, 85u8, 37u8,
                        ],
                    )
                }

                /// Named reserves on some account balances.
                pub fn reserves_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::reserves::Reserves,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        vec![],
                        [
                            128u8, 51u8, 44u8, 200u8, 179u8, 217u8, 252u8, 46u8, 37u8, 136u8,
                            237u8, 119u8, 242u8, 111u8, 86u8, 134u8, 224u8, 130u8, 166u8, 115u8,
                            177u8, 11u8, 118u8, 210u8, 190u8, 253u8, 157u8, 239u8, 33u8, 70u8, 4u8,
                            188u8,
                        ],
                    )
                }

                /// Named reserves on some account balances.
                pub fn reserves(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::reserves::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::reserves::Reserves,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Reserves",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            128u8, 51u8, 44u8, 200u8, 179u8, 217u8, 252u8, 46u8, 37u8, 136u8,
                            237u8, 119u8, 242u8, 111u8, 86u8, 134u8, 224u8, 130u8, 166u8, 115u8,
                            177u8, 11u8, 118u8, 210u8, 190u8, 253u8, 157u8, 239u8, 33u8, 70u8, 4u8,
                            188u8,
                        ],
                    )
                }

                /// Holds on account balances.
                pub fn holds_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::holds::Holds,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        vec![],
                        [
                            146u8, 63u8, 22u8, 212u8, 190u8, 232u8, 81u8, 227u8, 170u8, 90u8, 14u8,
                            70u8, 129u8, 96u8, 96u8, 234u8, 217u8, 208u8, 246u8, 69u8, 121u8, 42u8,
                            143u8, 218u8, 100u8, 115u8, 157u8, 130u8, 210u8, 138u8, 67u8, 137u8,
                        ],
                    )
                }

                /// Holds on account balances.
                pub fn holds(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::holds::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::holds::Holds,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Holds",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            146u8, 63u8, 22u8, 212u8, 190u8, 232u8, 81u8, 227u8, 170u8, 90u8, 14u8,
                            70u8, 129u8, 96u8, 96u8, 234u8, 217u8, 208u8, 246u8, 69u8, 121u8, 42u8,
                            143u8, 218u8, 100u8, 115u8, 157u8, 130u8, 210u8, 138u8, 67u8, 137u8,
                        ],
                    )
                }

                /// Freeze locks on account balances.
                pub fn freezes_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::freezes::Freezes,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        vec![],
                        [
                            170u8, 69u8, 116u8, 92u8, 165u8, 14u8, 129u8, 179u8, 165u8, 6u8, 123u8,
                            156u8, 4u8, 30u8, 25u8, 181u8, 191u8, 29u8, 3u8, 92u8, 96u8, 167u8,
                            102u8, 38u8, 128u8, 140u8, 85u8, 248u8, 114u8, 127u8, 128u8, 40u8,
                        ],
                    )
                }

                /// Freeze locks on account balances.
                pub fn freezes(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::freezes::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::freezes::Freezes,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Balances",
                        "Freezes",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            170u8, 69u8, 116u8, 92u8, 165u8, 14u8, 129u8, 179u8, 165u8, 6u8, 123u8,
                            156u8, 4u8, 30u8, 25u8, 181u8, 191u8, 29u8, 3u8, 92u8, 96u8, 167u8,
                            102u8, 38u8, 128u8, 140u8, 85u8, 248u8, 114u8, 127u8, 128u8, 40u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// The minimum amount required to keep an account open. MUST BE
                /// GREATER THAN ZERO!
                ///
                /// If you *really* need it to be zero, you can enable the
                /// feature `insecure_zero_ed` for
                /// this pallet. However, you do so at your own risk: this will
                /// open up a major DoS vector.
                /// In case you have multiple sources of provider references,
                /// you may also get unexpected
                /// behaviour if you set this to zero.
                ///
                /// Bottom line: Do yourself a favour and make it at least one!
                pub fn existential_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "ExistentialDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The maximum number of locks that should exist on an account.
                /// Not strictly enforced, but used for weight estimation.
                pub fn max_locks(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxLocks",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// The maximum number of named reserves that can exist on an
                /// account.
                pub fn max_reserves(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxReserves",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// The maximum number of holds that can exist on an account at
                /// any time.
                pub fn max_holds(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxHolds",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// The maximum number of individual freeze locks that can exist
                /// on an account at any time.
                pub fn max_freezes(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Balances",
                        "MaxFreezes",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod validator_set {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::substrate_validator_set::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::substrate_validator_set::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::add_validator`].
                pub struct AddValidator {
                    pub validator_id: add_validator::ValidatorId,
                }
                pub mod add_validator {
                    use super::runtime_types;
                    pub type ValidatorId = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for AddValidator {
                    const CALL: &'static str = "add_validator";
                    const PALLET: &'static str = "ValidatorSet";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::remove_validator`].
                pub struct RemoveValidator {
                    pub validator_id: remove_validator::ValidatorId,
                }
                pub mod remove_validator {
                    use super::runtime_types;
                    pub type ValidatorId = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveValidator {
                    const CALL: &'static str = "remove_validator";
                    const PALLET: &'static str = "ValidatorSet";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::add_validator`].
                pub fn add_validator(
                    &self,
                    validator_id: types::add_validator::ValidatorId,
                ) -> ::subxt::tx::Payload<types::AddValidator> {
                    ::subxt::tx::Payload::new_static(
                        "ValidatorSet",
                        "add_validator",
                        types::AddValidator {
                            validator_id,
                        },
                        [
                            126u8, 146u8, 187u8, 183u8, 159u8, 201u8, 223u8, 160u8, 19u8, 193u8,
                            112u8, 251u8, 195u8, 206u8, 104u8, 98u8, 246u8, 104u8, 40u8, 122u8,
                            86u8, 94u8, 97u8, 190u8, 47u8, 207u8, 221u8, 71u8, 119u8, 50u8, 79u8,
                            68u8,
                        ],
                    )
                }

                /// See [`Pallet::remove_validator`].
                pub fn remove_validator(
                    &self,
                    validator_id: types::remove_validator::ValidatorId,
                ) -> ::subxt::tx::Payload<types::RemoveValidator> {
                    ::subxt::tx::Payload::new_static(
                        "ValidatorSet",
                        "remove_validator",
                        types::RemoveValidator {
                            validator_id,
                        },
                        [
                            83u8, 26u8, 215u8, 89u8, 19u8, 91u8, 208u8, 249u8, 201u8, 127u8, 230u8,
                            207u8, 191u8, 215u8, 193u8, 206u8, 55u8, 139u8, 121u8, 133u8, 42u8,
                            97u8, 68u8, 90u8, 210u8, 81u8, 74u8, 126u8, 60u8, 80u8, 242u8, 132u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::substrate_validator_set::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// New validator addition initiated. Effective in ~2 sessions.
            pub struct ValidatorAdditionInitiated(pub validator_addition_initiated::Field0);
            pub mod validator_addition_initiated {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for ValidatorAdditionInitiated {
                const EVENT: &'static str = "ValidatorAdditionInitiated";
                const PALLET: &'static str = "ValidatorSet";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Validator removal initiated. Effective in ~2 sessions.
            pub struct ValidatorRemovalInitiated(pub validator_removal_initiated::Field0);
            pub mod validator_removal_initiated {
                use super::runtime_types;
                pub type Field0 = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for ValidatorRemovalInitiated {
                const EVENT: &'static str = "ValidatorRemovalInitiated";
                const PALLET: &'static str = "ValidatorSet";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod validators {
                    use super::runtime_types;
                    pub type Validators = ::std::vec::Vec<::subxt::utils::AccountId32>;
                }
                pub mod offline_validators {
                    use super::runtime_types;
                    pub type OfflineValidators = ::std::vec::Vec<::subxt::utils::AccountId32>;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::validators::Validators,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ValidatorSet",
                        "Validators",
                        vec![],
                        [
                            50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
                            133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
                            115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
                            86u8,
                        ],
                    )
                }

                pub fn offline_validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::offline_validators::OfflineValidators,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ValidatorSet",
                        "OfflineValidators",
                        vec![],
                        [
                            127u8, 204u8, 6u8, 22u8, 44u8, 16u8, 58u8, 220u8, 168u8, 214u8, 92u8,
                            60u8, 104u8, 206u8, 237u8, 231u8, 39u8, 59u8, 255u8, 82u8, 209u8,
                            236u8, 195u8, 135u8, 128u8, 185u8, 103u8, 140u8, 216u8, 213u8, 249u8,
                            2u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod session {
        use super::root_mod;
        use super::runtime_types;
        /// Error for the session pallet.
        pub type Error = runtime_types::pallet_session::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::pallet_session::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_keys`].
                pub struct SetKeys {
                    pub keys: set_keys::Keys,
                    pub proof: set_keys::Proof,
                }
                pub mod set_keys {
                    use super::runtime_types;
                    pub type Keys = runtime_types::nagara_core_runtime::opaque::SessionKeys;
                    pub type Proof = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetKeys {
                    const CALL: &'static str = "set_keys";
                    const PALLET: &'static str = "Session";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::purge_keys`].
                pub struct PurgeKeys;
                impl ::subxt::blocks::StaticExtrinsic for PurgeKeys {
                    const CALL: &'static str = "purge_keys";
                    const PALLET: &'static str = "Session";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::set_keys`].
                pub fn set_keys(
                    &self,
                    keys: types::set_keys::Keys,
                    proof: types::set_keys::Proof,
                ) -> ::subxt::tx::Payload<types::SetKeys> {
                    ::subxt::tx::Payload::new_static(
                        "Session",
                        "set_keys",
                        types::SetKeys {
                            keys,
                            proof,
                        },
                        [
                            47u8, 127u8, 163u8, 217u8, 206u8, 187u8, 133u8, 242u8, 41u8, 220u8,
                            161u8, 23u8, 104u8, 81u8, 53u8, 96u8, 129u8, 183u8, 37u8, 129u8, 178u8,
                            48u8, 192u8, 123u8, 194u8, 58u8, 193u8, 238u8, 114u8, 250u8, 81u8,
                            252u8,
                        ],
                    )
                }

                /// See [`Pallet::purge_keys`].
                pub fn purge_keys(&self) -> ::subxt::tx::Payload<types::PurgeKeys> {
                    ::subxt::tx::Payload::new_static(
                        "Session",
                        "purge_keys",
                        types::PurgeKeys {},
                        [
                            215u8, 204u8, 146u8, 236u8, 32u8, 78u8, 198u8, 79u8, 85u8, 214u8, 15u8,
                            151u8, 158u8, 31u8, 146u8, 119u8, 119u8, 204u8, 151u8, 169u8, 226u8,
                            67u8, 217u8, 39u8, 241u8, 245u8, 203u8, 240u8, 203u8, 172u8, 16u8,
                            209u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// New session has happened. Note that the argument is the session
            /// index, not the
            /// block number as the type might suggest.
            pub struct NewSession {
                pub session_index: new_session::SessionIndex,
            }
            pub mod new_session {
                use super::runtime_types;
                pub type SessionIndex = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for NewSession {
                const EVENT: &'static str = "NewSession";
                const PALLET: &'static str = "Session";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod validators {
                    use super::runtime_types;
                    pub type Validators = ::std::vec::Vec<::subxt::utils::AccountId32>;
                }
                pub mod current_index {
                    use super::runtime_types;
                    pub type CurrentIndex = ::core::primitive::u32;
                }
                pub mod queued_changed {
                    use super::runtime_types;
                    pub type QueuedChanged = ::core::primitive::bool;
                }
                pub mod queued_keys {
                    use super::runtime_types;
                    pub type QueuedKeys = ::std::vec::Vec<(
                        ::subxt::utils::AccountId32,
                        runtime_types::nagara_core_runtime::opaque::SessionKeys,
                    )>;
                }
                pub mod disabled_validators {
                    use super::runtime_types;
                    pub type DisabledValidators = ::std::vec::Vec<::core::primitive::u32>;
                }
                pub mod next_keys {
                    use super::runtime_types;
                    pub type NextKeys = runtime_types::nagara_core_runtime::opaque::SessionKeys;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod key_owner {
                    use super::runtime_types;
                    pub type KeyOwner = ::subxt::utils::AccountId32;
                    pub type Param0 = runtime_types::sp_core::crypto::KeyTypeId;
                    pub type Param1 = [::core::primitive::u8];
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// The current set of validators.
                pub fn validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::validators::Validators,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "Validators",
                        vec![],
                        [
                            50u8, 86u8, 154u8, 222u8, 249u8, 209u8, 156u8, 22u8, 155u8, 25u8,
                            133u8, 194u8, 210u8, 50u8, 38u8, 28u8, 139u8, 201u8, 90u8, 139u8,
                            115u8, 12u8, 12u8, 141u8, 4u8, 178u8, 201u8, 241u8, 223u8, 234u8, 6u8,
                            86u8,
                        ],
                    )
                }

                /// Current index of the session.
                pub fn current_index(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::current_index::CurrentIndex,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "CurrentIndex",
                        vec![],
                        [
                            167u8, 151u8, 125u8, 150u8, 159u8, 21u8, 78u8, 217u8, 237u8, 183u8,
                            135u8, 65u8, 187u8, 114u8, 188u8, 206u8, 16u8, 32u8, 69u8, 208u8,
                            134u8, 159u8, 232u8, 224u8, 243u8, 27u8, 31u8, 166u8, 145u8, 44u8,
                            221u8, 230u8,
                        ],
                    )
                }

                /// True if the underlying economic identities or weighting
                /// behind the validators
                /// has changed in the queued validator set.
                pub fn queued_changed(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::queued_changed::QueuedChanged,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "QueuedChanged",
                        vec![],
                        [
                            184u8, 137u8, 224u8, 137u8, 31u8, 236u8, 95u8, 164u8, 102u8, 225u8,
                            198u8, 227u8, 140u8, 37u8, 113u8, 57u8, 59u8, 4u8, 202u8, 102u8, 117u8,
                            36u8, 226u8, 64u8, 113u8, 141u8, 199u8, 111u8, 99u8, 144u8, 198u8,
                            153u8,
                        ],
                    )
                }

                /// The queued keys for the next session. When the next session
                /// begins, these keys
                /// will be used to determine the validator's session keys.
                pub fn queued_keys(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::queued_keys::QueuedKeys,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "QueuedKeys",
                        vec![],
                        [
                            193u8, 50u8, 14u8, 97u8, 30u8, 205u8, 136u8, 133u8, 46u8, 201u8, 62u8,
                            178u8, 135u8, 253u8, 69u8, 73u8, 216u8, 251u8, 72u8, 93u8, 167u8, 96u8,
                            92u8, 5u8, 183u8, 74u8, 191u8, 28u8, 39u8, 217u8, 60u8, 80u8,
                        ],
                    )
                }

                /// Indices of disabled validators.
                ///
                /// The vec is always kept sorted so that we can find whether a
                /// given validator is
                /// disabled using binary search. It gets cleared when
                /// `on_session_ending` returns
                /// a new set of identities.
                pub fn disabled_validators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::disabled_validators::DisabledValidators,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "DisabledValidators",
                        vec![],
                        [
                            213u8, 19u8, 168u8, 234u8, 187u8, 200u8, 180u8, 97u8, 234u8, 189u8,
                            36u8, 233u8, 158u8, 184u8, 45u8, 35u8, 129u8, 213u8, 133u8, 8u8, 104u8,
                            183u8, 46u8, 68u8, 154u8, 240u8, 132u8, 22u8, 247u8, 11u8, 54u8, 221u8,
                        ],
                    )
                }

                /// The next session keys for a validator.
                pub fn next_keys_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::next_keys::NextKeys,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "NextKeys",
                        vec![],
                        [
                            24u8, 204u8, 239u8, 120u8, 213u8, 252u8, 121u8, 93u8, 144u8, 97u8,
                            64u8, 52u8, 105u8, 146u8, 63u8, 76u8, 60u8, 161u8, 70u8, 198u8, 158u8,
                            139u8, 248u8, 134u8, 224u8, 82u8, 251u8, 68u8, 207u8, 88u8, 134u8,
                            238u8,
                        ],
                    )
                }

                /// The next session keys for a validator.
                pub fn next_keys(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::next_keys::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::next_keys::NextKeys,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "NextKeys",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            24u8, 204u8, 239u8, 120u8, 213u8, 252u8, 121u8, 93u8, 144u8, 97u8,
                            64u8, 52u8, 105u8, 146u8, 63u8, 76u8, 60u8, 161u8, 70u8, 198u8, 158u8,
                            139u8, 248u8, 134u8, 224u8, 82u8, 251u8, 68u8, 207u8, 88u8, 134u8,
                            238u8,
                        ],
                    )
                }

                /// The owner of a key. The key is the `KeyTypeId` + the encoded
                /// key.
                pub fn key_owner_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::key_owner::KeyOwner,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "KeyOwner",
                        vec![],
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }

                /// The owner of a key. The key is the `KeyTypeId` + the encoded
                /// key.
                pub fn key_owner_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::key_owner::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::key_owner::KeyOwner,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "KeyOwner",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }

                /// The owner of a key. The key is the `KeyTypeId` + the encoded
                /// key.
                pub fn key_owner(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::key_owner::Param0>,
                    _1: impl ::std::borrow::Borrow<types::key_owner::Param1>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::key_owner::KeyOwner,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Session",
                        "KeyOwner",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            217u8, 204u8, 21u8, 114u8, 247u8, 129u8, 32u8, 242u8, 93u8, 91u8,
                            253u8, 253u8, 248u8, 90u8, 12u8, 202u8, 195u8, 25u8, 18u8, 100u8,
                            253u8, 109u8, 88u8, 77u8, 217u8, 140u8, 51u8, 40u8, 118u8, 35u8, 107u8,
                            206u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod aura {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod authorities {
                    use super::runtime_types;
                    pub type Authorities =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                        >;
                }
                pub mod current_slot {
                    use super::runtime_types;
                    pub type CurrentSlot = runtime_types::sp_consensus_slots::Slot;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// The current authority set.
                pub fn authorities(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::authorities::Authorities,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Aura",
                        "Authorities",
                        vec![],
                        [
                            232u8, 129u8, 167u8, 104u8, 47u8, 188u8, 238u8, 164u8, 6u8, 29u8,
                            129u8, 45u8, 64u8, 182u8, 194u8, 47u8, 0u8, 73u8, 63u8, 102u8, 204u8,
                            94u8, 111u8, 96u8, 137u8, 7u8, 141u8, 110u8, 180u8, 80u8, 228u8, 16u8,
                        ],
                    )
                }

                /// The current slot of this block.
                ///
                /// This will be set in `on_initialize`.
                pub fn current_slot(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::current_slot::CurrentSlot,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Aura",
                        "CurrentSlot",
                        vec![],
                        [
                            112u8, 199u8, 115u8, 248u8, 217u8, 242u8, 45u8, 231u8, 178u8, 53u8,
                            236u8, 167u8, 219u8, 238u8, 81u8, 243u8, 39u8, 140u8, 68u8, 19u8,
                            201u8, 169u8, 211u8, 133u8, 135u8, 213u8, 150u8, 105u8, 60u8, 252u8,
                            43u8, 57u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod grandpa {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::pallet_grandpa::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::pallet_grandpa::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::report_equivocation`].
                pub struct ReportEquivocation {
                    pub equivocation_proof:
                        ::std::boxed::Box<report_equivocation::EquivocationProof>,
                    pub key_owner_proof: report_equivocation::KeyOwnerProof,
                }
                pub mod report_equivocation {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_core::Void;
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportEquivocation {
                    const CALL: &'static str = "report_equivocation";
                    const PALLET: &'static str = "Grandpa";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::report_equivocation_unsigned`].
                pub struct ReportEquivocationUnsigned {
                    pub equivocation_proof:
                        ::std::boxed::Box<report_equivocation_unsigned::EquivocationProof>,
                    pub key_owner_proof: report_equivocation_unsigned::KeyOwnerProof,
                }
                pub mod report_equivocation_unsigned {
                    use super::runtime_types;
                    pub type EquivocationProof =
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >;
                    pub type KeyOwnerProof = runtime_types::sp_core::Void;
                }
                impl ::subxt::blocks::StaticExtrinsic for ReportEquivocationUnsigned {
                    const CALL: &'static str = "report_equivocation_unsigned";
                    const PALLET: &'static str = "Grandpa";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::note_stalled`].
                pub struct NoteStalled {
                    pub delay: note_stalled::Delay,
                    pub best_finalized_block_number: note_stalled::BestFinalizedBlockNumber,
                }
                pub mod note_stalled {
                    use super::runtime_types;
                    pub type Delay = ::core::primitive::u32;
                    pub type BestFinalizedBlockNumber = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for NoteStalled {
                    const CALL: &'static str = "note_stalled";
                    const PALLET: &'static str = "Grandpa";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::report_equivocation`].
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: types::report_equivocation::EquivocationProof,
                    key_owner_proof: types::report_equivocation::KeyOwnerProof,
                ) -> ::subxt::tx::Payload<types::ReportEquivocation> {
                    ::subxt::tx::Payload::new_static(
                        "Grandpa",
                        "report_equivocation",
                        types::ReportEquivocation {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            158u8, 70u8, 189u8, 51u8, 231u8, 191u8, 199u8, 33u8, 64u8, 156u8, 71u8,
                            243u8, 122u8, 199u8, 216u8, 10u8, 45u8, 73u8, 198u8, 141u8, 31u8,
                            209u8, 58u8, 164u8, 219u8, 124u8, 242u8, 26u8, 114u8, 52u8, 65u8,
                            106u8,
                        ],
                    )
                }

                /// See [`Pallet::report_equivocation_unsigned`].
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: types::report_equivocation_unsigned::EquivocationProof,
                    key_owner_proof: types::report_equivocation_unsigned::KeyOwnerProof,
                ) -> ::subxt::tx::Payload<types::ReportEquivocationUnsigned> {
                    ::subxt::tx::Payload::new_static(
                        "Grandpa",
                        "report_equivocation_unsigned",
                        types::ReportEquivocationUnsigned {
                            equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                            key_owner_proof,
                        },
                        [
                            53u8, 23u8, 255u8, 215u8, 105u8, 11u8, 67u8, 177u8, 234u8, 248u8,
                            183u8, 57u8, 230u8, 239u8, 54u8, 238u8, 115u8, 170u8, 153u8, 18u8,
                            55u8, 195u8, 85u8, 98u8, 109u8, 194u8, 57u8, 225u8, 139u8, 237u8,
                            171u8, 152u8,
                        ],
                    )
                }

                /// See [`Pallet::note_stalled`].
                pub fn note_stalled(
                    &self,
                    delay: types::note_stalled::Delay,
                    best_finalized_block_number: types::note_stalled::BestFinalizedBlockNumber,
                ) -> ::subxt::tx::Payload<types::NoteStalled> {
                    ::subxt::tx::Payload::new_static(
                        "Grandpa",
                        "note_stalled",
                        types::NoteStalled {
                            delay,
                            best_finalized_block_number,
                        },
                        [
                            158u8, 25u8, 64u8, 114u8, 131u8, 139u8, 227u8, 132u8, 42u8, 107u8,
                            40u8, 249u8, 18u8, 93u8, 254u8, 86u8, 37u8, 67u8, 250u8, 35u8, 241u8,
                            194u8, 209u8, 20u8, 39u8, 75u8, 186u8, 21u8, 48u8, 124u8, 151u8, 31u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// New authority set has been applied.
            pub struct NewAuthorities {
                pub authority_set: new_authorities::AuthoritySet,
            }
            pub mod new_authorities {
                use super::runtime_types;
                pub type AuthoritySet = ::std::vec::Vec<(
                    runtime_types::sp_consensus_grandpa::app::Public,
                    ::core::primitive::u64,
                )>;
            }
            impl ::subxt::events::StaticEvent for NewAuthorities {
                const EVENT: &'static str = "NewAuthorities";
                const PALLET: &'static str = "Grandpa";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Current authority set has been paused.
            pub struct Paused;
            impl ::subxt::events::StaticEvent for Paused {
                const EVENT: &'static str = "Paused";
                const PALLET: &'static str = "Grandpa";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Current authority set has been resumed.
            pub struct Resumed;
            impl ::subxt::events::StaticEvent for Resumed {
                const EVENT: &'static str = "Resumed";
                const PALLET: &'static str = "Grandpa";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod state {
                    use super::runtime_types;
                    pub type State =
                        runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                }
                pub mod pending_change {
                    use super::runtime_types;
                    pub type PendingChange =
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                }
                pub mod next_forced {
                    use super::runtime_types;
                    pub type NextForced = ::core::primitive::u32;
                }
                pub mod stalled {
                    use super::runtime_types;
                    pub type Stalled = (::core::primitive::u32, ::core::primitive::u32);
                }
                pub mod current_set_id {
                    use super::runtime_types;
                    pub type CurrentSetId = ::core::primitive::u64;
                }
                pub mod set_id_session {
                    use super::runtime_types;
                    pub type SetIdSession = ::core::primitive::u32;
                    pub type Param0 = ::core::primitive::u64;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// State of the current authority set.
                pub fn state(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::state::State,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "State",
                        vec![],
                        [
                            73u8, 71u8, 112u8, 83u8, 238u8, 75u8, 44u8, 9u8, 180u8, 33u8, 30u8,
                            121u8, 98u8, 96u8, 61u8, 133u8, 16u8, 70u8, 30u8, 249u8, 34u8, 148u8,
                            15u8, 239u8, 164u8, 157u8, 52u8, 27u8, 144u8, 52u8, 223u8, 109u8,
                        ],
                    )
                }

                /// Pending change: (signaled at, scheduled change).
                pub fn pending_change(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::pending_change::PendingChange,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "PendingChange",
                        vec![],
                        [
                            150u8, 194u8, 185u8, 248u8, 239u8, 43u8, 141u8, 253u8, 61u8, 106u8,
                            74u8, 164u8, 209u8, 204u8, 206u8, 200u8, 32u8, 38u8, 11u8, 78u8, 84u8,
                            243u8, 181u8, 142u8, 179u8, 151u8, 81u8, 204u8, 244u8, 150u8, 137u8,
                            250u8,
                        ],
                    )
                }

                /// next block number where we can force a change.
                pub fn next_forced(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::next_forced::NextForced,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "NextForced",
                        vec![],
                        [
                            3u8, 231u8, 56u8, 18u8, 87u8, 112u8, 227u8, 126u8, 180u8, 131u8, 255u8,
                            141u8, 82u8, 34u8, 61u8, 47u8, 234u8, 37u8, 95u8, 62u8, 33u8, 235u8,
                            231u8, 122u8, 125u8, 8u8, 223u8, 95u8, 255u8, 204u8, 40u8, 97u8,
                        ],
                    )
                }

                /// `true` if we are currently stalled.
                pub fn stalled(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::stalled::Stalled,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "Stalled",
                        vec![],
                        [
                            6u8, 81u8, 205u8, 142u8, 195u8, 48u8, 0u8, 247u8, 108u8, 170u8, 10u8,
                            249u8, 72u8, 206u8, 32u8, 103u8, 109u8, 57u8, 51u8, 21u8, 144u8, 204u8,
                            79u8, 8u8, 191u8, 185u8, 38u8, 34u8, 118u8, 223u8, 75u8, 241u8,
                        ],
                    )
                }

                /// The number of changes (both in terms of keys and underlying
                /// economic responsibilities)
                /// in the "set" of Grandpa validators from genesis.
                pub fn current_set_id(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::current_set_id::CurrentSetId,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "CurrentSetId",
                        vec![],
                        [
                            234u8, 215u8, 218u8, 42u8, 30u8, 76u8, 129u8, 40u8, 125u8, 137u8,
                            207u8, 47u8, 46u8, 213u8, 159u8, 50u8, 175u8, 81u8, 155u8, 123u8,
                            246u8, 175u8, 156u8, 68u8, 22u8, 113u8, 135u8, 137u8, 163u8, 18u8,
                            115u8, 73u8,
                        ],
                    )
                }

                /// A mapping from grandpa set ID to the index of the *most
                /// recent* session for which its
                /// members were responsible.
                ///
                /// This is only used for validating equivocation proofs. An
                /// equivocation proof must
                /// contains a key-ownership proof for a given session,
                /// therefore we need a way to tie
                /// together sessions and GRANDPA set ids, i.e. we need to
                /// validate that a validator
                /// was the owner of a given key on a given session, and what
                /// the active set ID was
                /// during that session.
                ///
                /// TWOX-NOTE: `SetId` is not under user control.
                pub fn set_id_session_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::set_id_session::SetIdSession,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "SetIdSession",
                        vec![],
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }

                /// A mapping from grandpa set ID to the index of the *most
                /// recent* session for which its
                /// members were responsible.
                ///
                /// This is only used for validating equivocation proofs. An
                /// equivocation proof must
                /// contains a key-ownership proof for a given session,
                /// therefore we need a way to tie
                /// together sessions and GRANDPA set ids, i.e. we need to
                /// validate that a validator
                /// was the owner of a given key on a given session, and what
                /// the active set ID was
                /// during that session.
                ///
                /// TWOX-NOTE: `SetId` is not under user control.
                pub fn set_id_session(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::set_id_session::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::set_id_session::SetIdSession,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Grandpa",
                        "SetIdSession",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            47u8, 0u8, 239u8, 121u8, 187u8, 213u8, 254u8, 50u8, 238u8, 10u8, 162u8,
                            65u8, 189u8, 166u8, 37u8, 74u8, 82u8, 81u8, 160u8, 20u8, 180u8, 253u8,
                            238u8, 18u8, 209u8, 203u8, 38u8, 148u8, 16u8, 105u8, 72u8, 169u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// Max Authorities in use
                pub fn max_authorities(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Grandpa",
                        "MaxAuthorities",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// The maximum number of entries to keep in the set id to
                /// session index mapping.
                ///
                /// Since the `SetIdSession` map is only used for validating
                /// equivocations this
                /// value should relate to the bonding duration of whatever
                /// staking system is
                /// being used (if any). If equivocation handling is not enabled
                /// then this value
                /// can be zero.
                pub fn max_set_id_session_entries(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "Grandpa",
                        "MaxSetIdSessionEntries",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::root_mod;
        use super::runtime_types;
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_transaction_payment::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A transaction fee `actual_fee`, of which `tip` was added to the
            /// minimum inclusion fee,
            /// has been paid by `who`.
            pub struct TransactionFeePaid {
                pub who: transaction_fee_paid::Who,
                pub actual_fee: transaction_fee_paid::ActualFee,
                pub tip: transaction_fee_paid::Tip,
            }
            pub mod transaction_fee_paid {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type ActualFee = ::core::primitive::u128;
                pub type Tip = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for TransactionFeePaid {
                const EVENT: &'static str = "TransactionFeePaid";
                const PALLET: &'static str = "TransactionPayment";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod next_fee_multiplier {
                    use super::runtime_types;
                    pub type NextFeeMultiplier =
                        runtime_types::sp_arithmetic::fixed_point::FixedU128;
                }
                pub mod storage_version {
                    use super::runtime_types;
                    pub type StorageVersion = runtime_types::pallet_transaction_payment::Releases;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn next_fee_multiplier(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::next_fee_multiplier::NextFeeMultiplier,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TransactionPayment",
                        "NextFeeMultiplier",
                        vec![],
                        [
                            247u8, 39u8, 81u8, 170u8, 225u8, 226u8, 82u8, 147u8, 34u8, 113u8,
                            147u8, 213u8, 59u8, 80u8, 139u8, 35u8, 36u8, 196u8, 152u8, 19u8, 9u8,
                            159u8, 176u8, 79u8, 249u8, 201u8, 170u8, 1u8, 129u8, 79u8, 146u8,
                            197u8,
                        ],
                    )
                }

                pub fn storage_version(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::storage_version::StorageVersion,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "TransactionPayment",
                        "StorageVersion",
                        vec![],
                        [
                            105u8, 243u8, 158u8, 241u8, 159u8, 231u8, 253u8, 6u8, 4u8, 32u8, 85u8,
                            178u8, 126u8, 31u8, 203u8, 134u8, 154u8, 38u8, 122u8, 155u8, 150u8,
                            251u8, 174u8, 15u8, 74u8, 134u8, 216u8, 244u8, 168u8, 175u8, 158u8,
                            144u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// A fee mulitplier for `Operational` extrinsics to compute
                /// "virtual tip" to boost their
                /// `priority`
                ///
                /// This value is multipled by the `final_fee` to obtain a
                /// "virtual tip" that is later
                /// added to a tip component in regular `priority` calculations.
                /// It means that a `Normal` transaction can front-run a
                /// similarly-sized `Operational`
                /// extrinsic (with no tip), by including a tip value greater
                /// than the virtual tip.
                ///
                /// ```rust,ignore
                /// // For `Normal`
                /// let priority = priority_calc(tip);
                ///
                /// // For `Operational`
                /// let virtual_tip = (inclusion_fee + tip) *
                /// OperationalFeeMultiplier;
                /// let priority = priority_calc(tip + virtual_tip);
                /// ```
                ///
                /// Note that since we use `final_fee` the multiplier applies
                /// also to the regular `tip`
                /// sent with the transaction. So, not only does the transaction
                /// get a priority bump based
                /// on the `inclusion_fee`, but we also amplify the impact of
                /// tips applied to `Operational`
                /// transactions.
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u8> {
                    ::subxt::constants::Address::new_static(
                        "TransactionPayment",
                        "OperationalFeeMultiplier",
                        [
                            141u8, 130u8, 11u8, 35u8, 226u8, 114u8, 92u8, 179u8, 168u8, 110u8,
                            28u8, 91u8, 221u8, 64u8, 4u8, 148u8, 201u8, 193u8, 185u8, 66u8, 226u8,
                            114u8, 97u8, 79u8, 62u8, 212u8, 202u8, 114u8, 237u8, 228u8, 183u8,
                            165u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod sudo {
        use super::root_mod;
        use super::runtime_types;
        /// Error for the Sudo pallet
        pub type Error = runtime_types::pallet_sudo::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::pallet_sudo::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::sudo`].
                pub struct Sudo {
                    pub call: ::std::boxed::Box<sudo::Call>,
                }
                pub mod sudo {
                    use super::runtime_types;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                }
                impl ::subxt::blocks::StaticExtrinsic for Sudo {
                    const CALL: &'static str = "sudo";
                    const PALLET: &'static str = "Sudo";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::sudo_unchecked_weight`].
                pub struct SudoUncheckedWeight {
                    pub call: ::std::boxed::Box<sudo_unchecked_weight::Call>,
                    pub weight: sudo_unchecked_weight::Weight,
                }
                pub mod sudo_unchecked_weight {
                    use super::runtime_types;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::blocks::StaticExtrinsic for SudoUncheckedWeight {
                    const CALL: &'static str = "sudo_unchecked_weight";
                    const PALLET: &'static str = "Sudo";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_key`].
                pub struct SetKey {
                    pub new: set_key::New,
                }
                pub mod set_key {
                    use super::runtime_types;
                    pub type New = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetKey {
                    const CALL: &'static str = "set_key";
                    const PALLET: &'static str = "Sudo";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::sudo_as`].
                pub struct SudoAs {
                    pub who: sudo_as::Who,
                    pub call: ::std::boxed::Box<sudo_as::Call>,
                }
                pub mod sudo_as {
                    use super::runtime_types;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                }
                impl ::subxt::blocks::StaticExtrinsic for SudoAs {
                    const CALL: &'static str = "sudo_as";
                    const PALLET: &'static str = "Sudo";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::sudo`].
                pub fn sudo(&self, call: types::sudo::Call) -> ::subxt::tx::Payload<types::Sudo> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo",
                        types::Sudo {
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            45u8, 164u8, 222u8, 139u8, 30u8, 143u8, 190u8, 181u8, 150u8, 51u8,
                            253u8, 62u8, 135u8, 9u8, 143u8, 218u8, 148u8, 59u8, 24u8, 174u8, 133u8,
                            36u8, 37u8, 142u8, 29u8, 170u8, 220u8, 46u8, 241u8, 201u8, 230u8,
                            111u8,
                        ],
                    )
                }

                /// See [`Pallet::sudo_unchecked_weight`].
                pub fn sudo_unchecked_weight(
                    &self,
                    call: types::sudo_unchecked_weight::Call,
                    weight: types::sudo_unchecked_weight::Weight,
                ) -> ::subxt::tx::Payload<types::SudoUncheckedWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo_unchecked_weight",
                        types::SudoUncheckedWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        },
                        [
                            222u8, 188u8, 106u8, 146u8, 2u8, 8u8, 171u8, 4u8, 88u8, 200u8, 144u8,
                            30u8, 44u8, 170u8, 123u8, 119u8, 176u8, 162u8, 179u8, 167u8, 156u8,
                            229u8, 204u8, 164u8, 5u8, 89u8, 110u8, 118u8, 55u8, 96u8, 232u8, 147u8,
                        ],
                    )
                }

                /// See [`Pallet::set_key`].
                pub fn set_key(
                    &self,
                    new: types::set_key::New,
                ) -> ::subxt::tx::Payload<types::SetKey> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "set_key",
                        types::SetKey {
                            new,
                        },
                        [
                            9u8, 73u8, 39u8, 205u8, 188u8, 127u8, 143u8, 54u8, 128u8, 94u8, 8u8,
                            227u8, 197u8, 44u8, 70u8, 93u8, 228u8, 196u8, 64u8, 165u8, 226u8,
                            158u8, 101u8, 192u8, 22u8, 193u8, 102u8, 84u8, 21u8, 35u8, 92u8, 198u8,
                        ],
                    )
                }

                /// See [`Pallet::sudo_as`].
                pub fn sudo_as(
                    &self,
                    who: types::sudo_as::Who,
                    call: types::sudo_as::Call,
                ) -> ::subxt::tx::Payload<types::SudoAs> {
                    ::subxt::tx::Payload::new_static(
                        "Sudo",
                        "sudo_as",
                        types::SudoAs {
                            who,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            215u8, 177u8, 124u8, 60u8, 133u8, 182u8, 244u8, 28u8, 238u8, 158u8,
                            142u8, 6u8, 84u8, 196u8, 113u8, 183u8, 8u8, 220u8, 16u8, 20u8, 254u8,
                            25u8, 61u8, 21u8, 103u8, 108u8, 166u8, 220u8, 87u8, 60u8, 117u8, 15u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A sudo just took place. \[result\]
            pub struct Sudid {
                pub sudo_result: sudid::SudoResult,
            }
            pub mod sudid {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::events::StaticEvent for Sudid {
                const EVENT: &'static str = "Sudid";
                const PALLET: &'static str = "Sudo";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// The \[sudoer\] just switched identity; the old key is supplied
            /// if one existed.
            pub struct KeyChanged {
                pub old_sudoer: key_changed::OldSudoer,
            }
            pub mod key_changed {
                use super::runtime_types;
                pub type OldSudoer = ::core::option::Option<::subxt::utils::AccountId32>;
            }
            impl ::subxt::events::StaticEvent for KeyChanged {
                const EVENT: &'static str = "KeyChanged";
                const PALLET: &'static str = "Sudo";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A sudo just took place. \[result\]
            pub struct SudoAsDone {
                pub sudo_result: sudo_as_done::SudoResult,
            }
            pub mod sudo_as_done {
                use super::runtime_types;
                pub type SudoResult =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::events::StaticEvent for SudoAsDone {
                const EVENT: &'static str = "SudoAsDone";
                const PALLET: &'static str = "Sudo";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod key {
                    use super::runtime_types;
                    pub type Key = ::subxt::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// The `AccountId` of the sudo key.
                pub fn key(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::key::Key,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Sudo",
                        "Key",
                        vec![],
                        [
                            72u8, 14u8, 225u8, 162u8, 205u8, 247u8, 227u8, 105u8, 116u8, 57u8, 4u8,
                            31u8, 84u8, 137u8, 227u8, 228u8, 133u8, 245u8, 206u8, 227u8, 117u8,
                            36u8, 252u8, 151u8, 107u8, 15u8, 180u8, 4u8, 4u8, 152u8, 195u8, 144u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod utility {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::pallet_utility::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::pallet_utility::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::batch`].
                pub struct Batch {
                    pub calls: batch::Calls,
                }
                pub mod batch {
                    use super::runtime_types;
                    pub type Calls =
                        ::std::vec::Vec<runtime_types::nagara_core_runtime::RuntimeCall>;
                }
                impl ::subxt::blocks::StaticExtrinsic for Batch {
                    const CALL: &'static str = "batch";
                    const PALLET: &'static str = "Utility";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::as_derivative`].
                pub struct AsDerivative {
                    pub index: as_derivative::Index,
                    pub call: ::std::boxed::Box<as_derivative::Call>,
                }
                pub mod as_derivative {
                    use super::runtime_types;
                    pub type Index = ::core::primitive::u16;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                }
                impl ::subxt::blocks::StaticExtrinsic for AsDerivative {
                    const CALL: &'static str = "as_derivative";
                    const PALLET: &'static str = "Utility";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::batch_all`].
                pub struct BatchAll {
                    pub calls: batch_all::Calls,
                }
                pub mod batch_all {
                    use super::runtime_types;
                    pub type Calls =
                        ::std::vec::Vec<runtime_types::nagara_core_runtime::RuntimeCall>;
                }
                impl ::subxt::blocks::StaticExtrinsic for BatchAll {
                    const CALL: &'static str = "batch_all";
                    const PALLET: &'static str = "Utility";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::dispatch_as`].
                pub struct DispatchAs {
                    pub as_origin: ::std::boxed::Box<dispatch_as::AsOrigin>,
                    pub call: ::std::boxed::Box<dispatch_as::Call>,
                }
                pub mod dispatch_as {
                    use super::runtime_types;
                    pub type AsOrigin = runtime_types::nagara_core_runtime::OriginCaller;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                }
                impl ::subxt::blocks::StaticExtrinsic for DispatchAs {
                    const CALL: &'static str = "dispatch_as";
                    const PALLET: &'static str = "Utility";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_batch`].
                pub struct ForceBatch {
                    pub calls: force_batch::Calls,
                }
                pub mod force_batch {
                    use super::runtime_types;
                    pub type Calls =
                        ::std::vec::Vec<runtime_types::nagara_core_runtime::RuntimeCall>;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceBatch {
                    const CALL: &'static str = "force_batch";
                    const PALLET: &'static str = "Utility";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::with_weight`].
                pub struct WithWeight {
                    pub call: ::std::boxed::Box<with_weight::Call>,
                    pub weight: with_weight::Weight,
                }
                pub mod with_weight {
                    use super::runtime_types;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                    pub type Weight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::blocks::StaticExtrinsic for WithWeight {
                    const CALL: &'static str = "with_weight";
                    const PALLET: &'static str = "Utility";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::batch`].
                pub fn batch(
                    &self,
                    calls: types::batch::Calls,
                ) -> ::subxt::tx::Payload<types::Batch> {
                    ::subxt::tx::Payload::new_static(
                        "Utility",
                        "batch",
                        types::Batch {
                            calls,
                        },
                        [
                            126u8, 122u8, 160u8, 197u8, 3u8, 167u8, 227u8, 56u8, 142u8, 126u8,
                            247u8, 111u8, 189u8, 160u8, 184u8, 66u8, 236u8, 2u8, 160u8, 79u8, 70u8,
                            90u8, 182u8, 44u8, 95u8, 233u8, 253u8, 111u8, 88u8, 24u8, 202u8, 186u8,
                        ],
                    )
                }

                /// See [`Pallet::as_derivative`].
                pub fn as_derivative(
                    &self,
                    index: types::as_derivative::Index,
                    call: types::as_derivative::Call,
                ) -> ::subxt::tx::Payload<types::AsDerivative> {
                    ::subxt::tx::Payload::new_static(
                        "Utility",
                        "as_derivative",
                        types::AsDerivative {
                            index,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            23u8, 168u8, 130u8, 201u8, 214u8, 87u8, 205u8, 1u8, 231u8, 114u8, 48u8,
                            255u8, 0u8, 194u8, 207u8, 128u8, 108u8, 231u8, 199u8, 29u8, 97u8,
                            213u8, 240u8, 81u8, 204u8, 189u8, 105u8, 247u8, 196u8, 152u8, 7u8,
                            205u8,
                        ],
                    )
                }

                /// See [`Pallet::batch_all`].
                pub fn batch_all(
                    &self,
                    calls: types::batch_all::Calls,
                ) -> ::subxt::tx::Payload<types::BatchAll> {
                    ::subxt::tx::Payload::new_static(
                        "Utility",
                        "batch_all",
                        types::BatchAll {
                            calls,
                        },
                        [
                            166u8, 182u8, 231u8, 119u8, 220u8, 140u8, 48u8, 30u8, 101u8, 211u8,
                            190u8, 44u8, 97u8, 84u8, 239u8, 17u8, 196u8, 72u8, 143u8, 32u8, 41u8,
                            132u8, 133u8, 117u8, 171u8, 226u8, 251u8, 154u8, 15u8, 83u8, 210u8,
                            46u8,
                        ],
                    )
                }

                /// See [`Pallet::dispatch_as`].
                pub fn dispatch_as(
                    &self,
                    as_origin: types::dispatch_as::AsOrigin,
                    call: types::dispatch_as::Call,
                ) -> ::subxt::tx::Payload<types::DispatchAs> {
                    ::subxt::tx::Payload::new_static(
                        "Utility",
                        "dispatch_as",
                        types::DispatchAs {
                            as_origin: ::std::boxed::Box::new(as_origin),
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            53u8, 213u8, 198u8, 212u8, 229u8, 13u8, 250u8, 255u8, 55u8, 177u8,
                            250u8, 214u8, 83u8, 124u8, 97u8, 153u8, 70u8, 87u8, 72u8, 67u8, 17u8,
                            60u8, 25u8, 137u8, 221u8, 55u8, 90u8, 150u8, 0u8, 177u8, 241u8, 225u8,
                        ],
                    )
                }

                /// See [`Pallet::force_batch`].
                pub fn force_batch(
                    &self,
                    calls: types::force_batch::Calls,
                ) -> ::subxt::tx::Payload<types::ForceBatch> {
                    ::subxt::tx::Payload::new_static(
                        "Utility",
                        "force_batch",
                        types::ForceBatch {
                            calls,
                        },
                        [
                            187u8, 62u8, 41u8, 79u8, 63u8, 105u8, 81u8, 147u8, 59u8, 31u8, 200u8,
                            63u8, 103u8, 206u8, 152u8, 156u8, 127u8, 155u8, 142u8, 24u8, 234u8,
                            39u8, 171u8, 116u8, 59u8, 11u8, 10u8, 93u8, 135u8, 242u8, 217u8, 58u8,
                        ],
                    )
                }

                /// See [`Pallet::with_weight`].
                pub fn with_weight(
                    &self,
                    call: types::with_weight::Call,
                    weight: types::with_weight::Weight,
                ) -> ::subxt::tx::Payload<types::WithWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Utility",
                        "with_weight",
                        types::WithWeight {
                            call: ::std::boxed::Box::new(call),
                            weight,
                        },
                        [
                            190u8, 22u8, 87u8, 10u8, 190u8, 219u8, 98u8, 185u8, 15u8, 164u8, 1u8,
                            112u8, 228u8, 234u8, 216u8, 156u8, 237u8, 104u8, 86u8, 203u8, 170u8,
                            92u8, 158u8, 33u8, 203u8, 13u8, 19u8, 208u8, 65u8, 171u8, 144u8, 148u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Batch of dispatches did not complete fully. Index of first
            /// failing dispatch given, as
            /// well as the error.
            pub struct BatchInterrupted {
                pub index: batch_interrupted::Index,
                pub error: batch_interrupted::Error,
            }
            pub mod batch_interrupted {
                use super::runtime_types;
                pub type Index = ::core::primitive::u32;
                pub type Error = runtime_types::sp_runtime::DispatchError;
            }
            impl ::subxt::events::StaticEvent for BatchInterrupted {
                const EVENT: &'static str = "BatchInterrupted";
                const PALLET: &'static str = "Utility";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Batch of dispatches completed fully with no error.
            pub struct BatchCompleted;
            impl ::subxt::events::StaticEvent for BatchCompleted {
                const EVENT: &'static str = "BatchCompleted";
                const PALLET: &'static str = "Utility";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Batch of dispatches completed but has errors.
            pub struct BatchCompletedWithErrors;
            impl ::subxt::events::StaticEvent for BatchCompletedWithErrors {
                const EVENT: &'static str = "BatchCompletedWithErrors";
                const PALLET: &'static str = "Utility";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A single item within a Batch of dispatches has completed with no
            /// error.
            pub struct ItemCompleted;
            impl ::subxt::events::StaticEvent for ItemCompleted {
                const EVENT: &'static str = "ItemCompleted";
                const PALLET: &'static str = "Utility";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A single item within a Batch of dispatches has completed with
            /// error.
            pub struct ItemFailed {
                pub error: item_failed::Error,
            }
            pub mod item_failed {
                use super::runtime_types;
                pub type Error = runtime_types::sp_runtime::DispatchError;
            }
            impl ::subxt::events::StaticEvent for ItemFailed {
                const EVENT: &'static str = "ItemFailed";
                const PALLET: &'static str = "Utility";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A call was dispatched.
            pub struct DispatchedAs {
                pub result: dispatched_as::Result,
            }
            pub mod dispatched_as {
                use super::runtime_types;
                pub type Result =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::events::StaticEvent for DispatchedAs {
                const EVENT: &'static str = "DispatchedAs";
                const PALLET: &'static str = "Utility";
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// The limit on the number of batched calls.
                pub fn batched_calls_limit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Utility",
                        "batched_calls_limit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod randomness_collective_flip {
        use super::root_mod;
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod random_material {
                    use super::runtime_types;
                    pub type RandomMaterial =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::utils::H256,
                        >;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// Series of block headers from the last 81 blocks that acts as
                /// random seed material. This
                /// is arranged as a ring buffer with `block_number % 81` being
                /// the index into the `Vec` of
                /// the oldest hash.
                pub fn random_material(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::random_material::RandomMaterial,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "RandomnessCollectiveFlip",
                        "RandomMaterial",
                        vec![],
                        [
                            195u8, 232u8, 244u8, 162u8, 110u8, 137u8, 66u8, 57u8, 51u8, 221u8,
                            143u8, 38u8, 51u8, 183u8, 105u8, 245u8, 175u8, 13u8, 33u8, 192u8, 53u8,
                            16u8, 161u8, 76u8, 219u8, 177u8, 144u8, 192u8, 96u8, 166u8, 117u8,
                            247u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod contracts {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::pallet_contracts::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::pallet_contracts::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::call_old_weight`].
                pub struct CallOldWeight {
                    pub dest: call_old_weight::Dest,
                    #[codec(compact)]
                    pub value: call_old_weight::Value,
                    #[codec(compact)]
                    pub gas_limit: call_old_weight::GasLimit,
                    pub storage_deposit_limit: call_old_weight::StorageDepositLimit,
                    pub data: call_old_weight::Data,
                }
                pub mod call_old_weight {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = ::core::primitive::u64;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Data = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for CallOldWeight {
                    const CALL: &'static str = "call_old_weight";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::instantiate_with_code_old_weight`].
                pub struct InstantiateWithCodeOldWeight {
                    #[codec(compact)]
                    pub value: instantiate_with_code_old_weight::Value,
                    #[codec(compact)]
                    pub gas_limit: instantiate_with_code_old_weight::GasLimit,
                    pub storage_deposit_limit:
                        instantiate_with_code_old_weight::StorageDepositLimit,
                    pub code: instantiate_with_code_old_weight::Code,
                    pub data: instantiate_with_code_old_weight::Data,
                    pub salt: instantiate_with_code_old_weight::Salt,
                }
                pub mod instantiate_with_code_old_weight {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = ::core::primitive::u64;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Code = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Data = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Salt = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for InstantiateWithCodeOldWeight {
                    const CALL: &'static str = "instantiate_with_code_old_weight";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::instantiate_old_weight`].
                pub struct InstantiateOldWeight {
                    #[codec(compact)]
                    pub value: instantiate_old_weight::Value,
                    #[codec(compact)]
                    pub gas_limit: instantiate_old_weight::GasLimit,
                    pub storage_deposit_limit: instantiate_old_weight::StorageDepositLimit,
                    pub code_hash: instantiate_old_weight::CodeHash,
                    pub data: instantiate_old_weight::Data,
                    pub salt: instantiate_old_weight::Salt,
                }
                pub mod instantiate_old_weight {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = ::core::primitive::u64;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type CodeHash = ::subxt::utils::H256;
                    pub type Data = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Salt = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for InstantiateOldWeight {
                    const CALL: &'static str = "instantiate_old_weight";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::upload_code`].
                pub struct UploadCode {
                    pub code: upload_code::Code,
                    pub storage_deposit_limit: upload_code::StorageDepositLimit,
                    pub determinism: upload_code::Determinism,
                }
                pub mod upload_code {
                    use super::runtime_types;
                    pub type Code = ::std::vec::Vec<::core::primitive::u8>;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Determinism = runtime_types::pallet_contracts::wasm::Determinism;
                }
                impl ::subxt::blocks::StaticExtrinsic for UploadCode {
                    const CALL: &'static str = "upload_code";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::remove_code`].
                pub struct RemoveCode {
                    pub code_hash: remove_code::CodeHash,
                }
                pub mod remove_code {
                    use super::runtime_types;
                    pub type CodeHash = ::subxt::utils::H256;
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveCode {
                    const CALL: &'static str = "remove_code";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_code`].
                pub struct SetCode {
                    pub dest: set_code::Dest,
                    pub code_hash: set_code::CodeHash,
                }
                pub mod set_code {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type CodeHash = ::subxt::utils::H256;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetCode {
                    const CALL: &'static str = "set_code";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::call`].
                pub struct Call {
                    pub dest: call::Dest,
                    #[codec(compact)]
                    pub value: call::Value,
                    pub gas_limit: call::GasLimit,
                    pub storage_deposit_limit: call::StorageDepositLimit,
                    pub data: call::Data,
                }
                pub mod call {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = runtime_types::sp_weights::weight_v2::Weight;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Data = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for Call {
                    const CALL: &'static str = "call";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::instantiate_with_code`].
                pub struct InstantiateWithCode {
                    #[codec(compact)]
                    pub value: instantiate_with_code::Value,
                    pub gas_limit: instantiate_with_code::GasLimit,
                    pub storage_deposit_limit: instantiate_with_code::StorageDepositLimit,
                    pub code: instantiate_with_code::Code,
                    pub data: instantiate_with_code::Data,
                    pub salt: instantiate_with_code::Salt,
                }
                pub mod instantiate_with_code {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = runtime_types::sp_weights::weight_v2::Weight;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type Code = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Data = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Salt = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for InstantiateWithCode {
                    const CALL: &'static str = "instantiate_with_code";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::instantiate`].
                pub struct Instantiate {
                    #[codec(compact)]
                    pub value: instantiate::Value,
                    pub gas_limit: instantiate::GasLimit,
                    pub storage_deposit_limit: instantiate::StorageDepositLimit,
                    pub code_hash: instantiate::CodeHash,
                    pub data: instantiate::Data,
                    pub salt: instantiate::Salt,
                }
                pub mod instantiate {
                    use super::runtime_types;
                    pub type Value = ::core::primitive::u128;
                    pub type GasLimit = runtime_types::sp_weights::weight_v2::Weight;
                    pub type StorageDepositLimit = ::core::option::Option<
                        ::subxt::ext::codec::Compact<::core::primitive::u128>,
                    >;
                    pub type CodeHash = ::subxt::utils::H256;
                    pub type Data = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Salt = ::std::vec::Vec<::core::primitive::u8>;
                }
                impl ::subxt::blocks::StaticExtrinsic for Instantiate {
                    const CALL: &'static str = "instantiate";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::migrate`].
                pub struct Migrate {
                    pub weight_limit: migrate::WeightLimit,
                }
                pub mod migrate {
                    use super::runtime_types;
                    pub type WeightLimit = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::blocks::StaticExtrinsic for Migrate {
                    const CALL: &'static str = "migrate";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::replace_contract_master`].
                pub struct ReplaceContractMaster {
                    pub new: replace_contract_master::New,
                }
                pub mod replace_contract_master {
                    use super::runtime_types;
                    pub type New = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for ReplaceContractMaster {
                    const CALL: &'static str = "replace_contract_master";
                    const PALLET: &'static str = "Contracts";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::remove_contract_master`].
                pub struct RemoveContractMaster;
                impl ::subxt::blocks::StaticExtrinsic for RemoveContractMaster {
                    const CALL: &'static str = "remove_contract_master";
                    const PALLET: &'static str = "Contracts";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::call_old_weight`].
                pub fn call_old_weight(
                    &self,
                    dest: types::call_old_weight::Dest,
                    value: types::call_old_weight::Value,
                    gas_limit: types::call_old_weight::GasLimit,
                    storage_deposit_limit: types::call_old_weight::StorageDepositLimit,
                    data: types::call_old_weight::Data,
                ) -> ::subxt::tx::Payload<types::CallOldWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "call_old_weight",
                        types::CallOldWeight {
                            dest,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            data,
                        },
                        [
                            159u8, 201u8, 54u8, 189u8, 207u8, 238u8, 0u8, 63u8, 0u8, 188u8, 150u8,
                            113u8, 13u8, 9u8, 199u8, 250u8, 77u8, 35u8, 174u8, 97u8, 13u8, 249u8,
                            21u8, 172u8, 49u8, 32u8, 228u8, 13u8, 229u8, 107u8, 135u8, 182u8,
                        ],
                    )
                }

                /// See [`Pallet::instantiate_with_code_old_weight`].
                pub fn instantiate_with_code_old_weight(
                    &self,
                    value: types::instantiate_with_code_old_weight::Value,
                    gas_limit: types::instantiate_with_code_old_weight::GasLimit,
                    storage_deposit_limit : types :: instantiate_with_code_old_weight :: StorageDepositLimit,
                    code: types::instantiate_with_code_old_weight::Code,
                    data: types::instantiate_with_code_old_weight::Data,
                    salt: types::instantiate_with_code_old_weight::Salt,
                ) -> ::subxt::tx::Payload<types::InstantiateWithCodeOldWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "instantiate_with_code_old_weight",
                        types::InstantiateWithCodeOldWeight {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code,
                            data,
                            salt,
                        },
                        [
                            48u8, 125u8, 188u8, 220u8, 158u8, 122u8, 158u8, 63u8, 0u8, 249u8,
                            164u8, 200u8, 199u8, 2u8, 21u8, 168u8, 84u8, 158u8, 120u8, 17u8, 82u8,
                            54u8, 115u8, 185u8, 69u8, 236u8, 64u8, 176u8, 187u8, 201u8, 230u8,
                            98u8,
                        ],
                    )
                }

                /// See [`Pallet::instantiate_old_weight`].
                pub fn instantiate_old_weight(
                    &self,
                    value: types::instantiate_old_weight::Value,
                    gas_limit: types::instantiate_old_weight::GasLimit,
                    storage_deposit_limit: types::instantiate_old_weight::StorageDepositLimit,
                    code_hash: types::instantiate_old_weight::CodeHash,
                    data: types::instantiate_old_weight::Data,
                    salt: types::instantiate_old_weight::Salt,
                ) -> ::subxt::tx::Payload<types::InstantiateOldWeight> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "instantiate_old_weight",
                        types::InstantiateOldWeight {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code_hash,
                            data,
                            salt,
                        },
                        [
                            145u8, 119u8, 37u8, 211u8, 172u8, 215u8, 72u8, 110u8, 71u8, 230u8,
                            212u8, 56u8, 78u8, 221u8, 239u8, 159u8, 110u8, 219u8, 71u8, 10u8,
                            248u8, 112u8, 237u8, 188u8, 198u8, 0u8, 28u8, 255u8, 147u8, 152u8,
                            162u8, 83u8,
                        ],
                    )
                }

                /// See [`Pallet::upload_code`].
                pub fn upload_code(
                    &self,
                    code: types::upload_code::Code,
                    storage_deposit_limit: types::upload_code::StorageDepositLimit,
                    determinism: types::upload_code::Determinism,
                ) -> ::subxt::tx::Payload<types::UploadCode> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "upload_code",
                        types::UploadCode {
                            code,
                            storage_deposit_limit,
                            determinism,
                        },
                        [
                            159u8, 17u8, 234u8, 83u8, 162u8, 68u8, 117u8, 80u8, 64u8, 251u8, 31u8,
                            38u8, 214u8, 227u8, 235u8, 74u8, 97u8, 72u8, 83u8, 197u8, 7u8, 57u8,
                            212u8, 217u8, 219u8, 139u8, 182u8, 248u8, 92u8, 91u8, 56u8, 2u8,
                        ],
                    )
                }

                /// See [`Pallet::remove_code`].
                pub fn remove_code(
                    &self,
                    code_hash: types::remove_code::CodeHash,
                ) -> ::subxt::tx::Payload<types::RemoveCode> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "remove_code",
                        types::RemoveCode {
                            code_hash,
                        },
                        [
                            99u8, 184u8, 12u8, 208u8, 123u8, 158u8, 140u8, 21u8, 190u8, 152u8,
                            95u8, 79u8, 217u8, 131u8, 161u8, 160u8, 21u8, 56u8, 167u8, 27u8, 90u8,
                            255u8, 75u8, 0u8, 133u8, 111u8, 119u8, 217u8, 157u8, 67u8, 238u8, 69u8,
                        ],
                    )
                }

                /// See [`Pallet::set_code`].
                pub fn set_code(
                    &self,
                    dest: types::set_code::Dest,
                    code_hash: types::set_code::CodeHash,
                ) -> ::subxt::tx::Payload<types::SetCode> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "set_code",
                        types::SetCode {
                            dest,
                            code_hash,
                        },
                        [
                            66u8, 53u8, 180u8, 182u8, 167u8, 134u8, 212u8, 45u8, 162u8, 121u8,
                            89u8, 105u8, 7u8, 166u8, 112u8, 13u8, 250u8, 115u8, 128u8, 235u8,
                            124u8, 55u8, 166u8, 5u8, 158u8, 163u8, 159u8, 113u8, 243u8, 103u8,
                            214u8, 108u8,
                        ],
                    )
                }

                /// See [`Pallet::call`].
                pub fn call(
                    &self,
                    dest: types::call::Dest,
                    value: types::call::Value,
                    gas_limit: types::call::GasLimit,
                    storage_deposit_limit: types::call::StorageDepositLimit,
                    data: types::call::Data,
                ) -> ::subxt::tx::Payload<types::Call> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "call",
                        types::Call {
                            dest,
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            data,
                        },
                        [
                            38u8, 98u8, 23u8, 7u8, 215u8, 169u8, 8u8, 156u8, 72u8, 172u8, 166u8,
                            189u8, 34u8, 9u8, 193u8, 204u8, 20u8, 152u8, 48u8, 40u8, 106u8, 109u8,
                            23u8, 64u8, 48u8, 131u8, 99u8, 37u8, 49u8, 26u8, 210u8, 196u8,
                        ],
                    )
                }

                /// See [`Pallet::instantiate_with_code`].
                pub fn instantiate_with_code(
                    &self,
                    value: types::instantiate_with_code::Value,
                    gas_limit: types::instantiate_with_code::GasLimit,
                    storage_deposit_limit: types::instantiate_with_code::StorageDepositLimit,
                    code: types::instantiate_with_code::Code,
                    data: types::instantiate_with_code::Data,
                    salt: types::instantiate_with_code::Salt,
                ) -> ::subxt::tx::Payload<types::InstantiateWithCode> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "instantiate_with_code",
                        types::InstantiateWithCode {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code,
                            data,
                            salt,
                        },
                        [
                            34u8, 182u8, 171u8, 163u8, 86u8, 205u8, 184u8, 72u8, 117u8, 214u8,
                            11u8, 24u8, 73u8, 6u8, 158u8, 16u8, 5u8, 212u8, 209u8, 64u8, 66u8,
                            98u8, 47u8, 14u8, 96u8, 132u8, 22u8, 37u8, 202u8, 148u8, 83u8, 125u8,
                        ],
                    )
                }

                /// See [`Pallet::instantiate`].
                pub fn instantiate(
                    &self,
                    value: types::instantiate::Value,
                    gas_limit: types::instantiate::GasLimit,
                    storage_deposit_limit: types::instantiate::StorageDepositLimit,
                    code_hash: types::instantiate::CodeHash,
                    data: types::instantiate::Data,
                    salt: types::instantiate::Salt,
                ) -> ::subxt::tx::Payload<types::Instantiate> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "instantiate",
                        types::Instantiate {
                            value,
                            gas_limit,
                            storage_deposit_limit,
                            code_hash,
                            data,
                            salt,
                        },
                        [
                            221u8, 142u8, 55u8, 187u8, 6u8, 98u8, 228u8, 231u8, 38u8, 81u8, 222u8,
                            86u8, 205u8, 122u8, 32u8, 236u8, 237u8, 50u8, 201u8, 140u8, 111u8,
                            23u8, 242u8, 212u8, 118u8, 212u8, 98u8, 247u8, 166u8, 196u8, 206u8,
                            232u8,
                        ],
                    )
                }

                /// See [`Pallet::migrate`].
                pub fn migrate(
                    &self,
                    weight_limit: types::migrate::WeightLimit,
                ) -> ::subxt::tx::Payload<types::Migrate> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "migrate",
                        types::Migrate {
                            weight_limit,
                        },
                        [
                            11u8, 183u8, 183u8, 30u8, 18u8, 17u8, 58u8, 145u8, 254u8, 126u8, 21u8,
                            155u8, 27u8, 218u8, 95u8, 35u8, 38u8, 102u8, 234u8, 241u8, 67u8, 99u8,
                            183u8, 164u8, 5u8, 66u8, 186u8, 77u8, 234u8, 76u8, 206u8, 248u8,
                        ],
                    )
                }

                /// See [`Pallet::replace_contract_master`].
                pub fn replace_contract_master(
                    &self,
                    new: types::replace_contract_master::New,
                ) -> ::subxt::tx::Payload<types::ReplaceContractMaster> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "replace_contract_master",
                        types::ReplaceContractMaster {
                            new,
                        },
                        [
                            120u8, 81u8, 167u8, 148u8, 189u8, 85u8, 196u8, 166u8, 244u8, 99u8,
                            24u8, 187u8, 72u8, 204u8, 14u8, 12u8, 6u8, 167u8, 160u8, 16u8, 146u8,
                            191u8, 177u8, 208u8, 146u8, 86u8, 230u8, 3u8, 173u8, 77u8, 237u8, 8u8,
                        ],
                    )
                }

                /// See [`Pallet::remove_contract_master`].
                pub fn remove_contract_master(
                    &self,
                ) -> ::subxt::tx::Payload<types::RemoveContractMaster> {
                    ::subxt::tx::Payload::new_static(
                        "Contracts",
                        "remove_contract_master",
                        types::RemoveContractMaster {},
                        [
                            75u8, 10u8, 243u8, 26u8, 174u8, 217u8, 81u8, 129u8, 240u8, 221u8, 0u8,
                            83u8, 210u8, 154u8, 60u8, 168u8, 219u8, 161u8, 88u8, 80u8, 156u8, 59u8,
                            103u8, 45u8, 30u8, 114u8, 103u8, 255u8, 169u8, 122u8, 233u8, 239u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_contracts::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Contract deployed by address at the specified address.
            pub struct Instantiated {
                pub deployer: instantiated::Deployer,
                pub contract: instantiated::Contract,
            }
            pub mod instantiated {
                use super::runtime_types;
                pub type Deployer = ::subxt::utils::AccountId32;
                pub type Contract = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Instantiated {
                const EVENT: &'static str = "Instantiated";
                const PALLET: &'static str = "Contracts";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Contract has been removed.
            ///
            /// # Note
            ///
            /// The only way for a contract to be removed and emitting this
            /// event is by calling
            /// `seal_terminate`.
            pub struct Terminated {
                pub contract: terminated::Contract,
                pub beneficiary: terminated::Beneficiary,
            }
            pub mod terminated {
                use super::runtime_types;
                pub type Contract = ::subxt::utils::AccountId32;
                pub type Beneficiary = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Terminated {
                const EVENT: &'static str = "Terminated";
                const PALLET: &'static str = "Contracts";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Code with the specified hash has been stored.
            pub struct CodeStored {
                pub code_hash: code_stored::CodeHash,
            }
            pub mod code_stored {
                use super::runtime_types;
                pub type CodeHash = ::subxt::utils::H256;
            }
            impl ::subxt::events::StaticEvent for CodeStored {
                const EVENT: &'static str = "CodeStored";
                const PALLET: &'static str = "Contracts";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A custom event emitted by the contract.
            pub struct ContractEmitted {
                pub contract: contract_emitted::Contract,
                pub data: contract_emitted::Data,
            }
            pub mod contract_emitted {
                use super::runtime_types;
                pub type Contract = ::subxt::utils::AccountId32;
                pub type Data = ::std::vec::Vec<::core::primitive::u8>;
            }
            impl ::subxt::events::StaticEvent for ContractEmitted {
                const EVENT: &'static str = "ContractEmitted";
                const PALLET: &'static str = "Contracts";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A code with the specified hash was removed.
            pub struct CodeRemoved {
                pub code_hash: code_removed::CodeHash,
            }
            pub mod code_removed {
                use super::runtime_types;
                pub type CodeHash = ::subxt::utils::H256;
            }
            impl ::subxt::events::StaticEvent for CodeRemoved {
                const EVENT: &'static str = "CodeRemoved";
                const PALLET: &'static str = "Contracts";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A contract's code was updated.
            pub struct ContractCodeUpdated {
                pub contract: contract_code_updated::Contract,
                pub new_code_hash: contract_code_updated::NewCodeHash,
                pub old_code_hash: contract_code_updated::OldCodeHash,
            }
            pub mod contract_code_updated {
                use super::runtime_types;
                pub type Contract = ::subxt::utils::AccountId32;
                pub type NewCodeHash = ::subxt::utils::H256;
                pub type OldCodeHash = ::subxt::utils::H256;
            }
            impl ::subxt::events::StaticEvent for ContractCodeUpdated {
                const EVENT: &'static str = "ContractCodeUpdated";
                const PALLET: &'static str = "Contracts";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A contract was called either by a plain account or another
            /// contract.
            ///
            /// # Note
            ///
            /// Please keep in mind that like all events this is only emitted
            /// for successful
            /// calls. This is because on failure all storage changes including
            /// events are
            /// rolled back.
            pub struct Called {
                pub caller: called::Caller,
                pub contract: called::Contract,
            }
            pub mod called {
                use super::runtime_types;
                pub type Caller = runtime_types::pallet_contracts::Origin<
                    runtime_types::nagara_core_runtime::Runtime,
                >;
                pub type Contract = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Called {
                const EVENT: &'static str = "Called";
                const PALLET: &'static str = "Contracts";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A contract delegate called a code hash.
            ///
            /// # Note
            ///
            /// Please keep in mind that like all events this is only emitted
            /// for successful
            /// calls. This is because on failure all storage changes including
            /// events are
            /// rolled back.
            pub struct DelegateCalled {
                pub contract: delegate_called::Contract,
                pub code_hash: delegate_called::CodeHash,
            }
            pub mod delegate_called {
                use super::runtime_types;
                pub type Contract = ::subxt::utils::AccountId32;
                pub type CodeHash = ::subxt::utils::H256;
            }
            impl ::subxt::events::StaticEvent for DelegateCalled {
                const EVENT: &'static str = "DelegateCalled";
                const PALLET: &'static str = "Contracts";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// New Contract Master has been replaced.
            pub struct ContractMasterReplaced {
                pub from: contract_master_replaced::From,
                pub new: contract_master_replaced::New,
            }
            pub mod contract_master_replaced {
                use super::runtime_types;
                pub type From = ::core::option::Option<::subxt::utils::AccountId32>;
                pub type New = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for ContractMasterReplaced {
                const EVENT: &'static str = "ContractMasterReplaced";
                const PALLET: &'static str = "Contracts";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Contract Master killed.
            pub struct ContractMasterKilled {
                pub who: contract_master_killed::Who,
            }
            pub mod contract_master_killed {
                use super::runtime_types;
                pub type Who = ::core::option::Option<::subxt::utils::AccountId32>;
            }
            impl ::subxt::events::StaticEvent for ContractMasterKilled {
                const EVENT: &'static str = "ContractMasterKilled";
                const PALLET: &'static str = "Contracts";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod contract_master {
                    use super::runtime_types;
                    pub type ContractMaster = ::subxt::utils::AccountId32;
                }
                pub mod pristine_code {
                    use super::runtime_types;
                    pub type PristineCode =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >;
                    pub type Param0 = ::subxt::utils::H256;
                }
                pub mod code_info_of {
                    use super::runtime_types;
                    pub type CodeInfoOf = runtime_types::pallet_contracts::wasm::CodeInfo;
                    pub type Param0 = ::subxt::utils::H256;
                }
                pub mod nonce {
                    use super::runtime_types;
                    pub type Nonce = ::core::primitive::u64;
                }
                pub mod contract_info_of {
                    use super::runtime_types;
                    pub type ContractInfoOf =
                        runtime_types::pallet_contracts::storage::ContractInfo;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod deletion_queue {
                    use super::runtime_types;
                    pub type DeletionQueue =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod deletion_queue_counter {
                    use super::runtime_types;
                    pub type DeletionQueueCounter =
                        runtime_types::pallet_contracts::storage::DeletionQueueManager;
                }
                pub mod migration_in_progress {
                    use super::runtime_types;
                    pub type MigrationInProgress =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn contract_master(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::contract_master::ContractMaster,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "ContractMaster",
                        vec![],
                        [
                            112u8, 145u8, 68u8, 8u8, 252u8, 15u8, 188u8, 9u8, 169u8, 112u8, 9u8,
                            121u8, 47u8, 89u8, 240u8, 235u8, 94u8, 201u8, 101u8, 146u8, 56u8,
                            113u8, 31u8, 80u8, 9u8, 188u8, 237u8, 13u8, 143u8, 205u8, 230u8, 196u8,
                        ],
                    )
                }

                /// A mapping from a contract's code hash to its code.
                pub fn pristine_code_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::pristine_code::PristineCode,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "PristineCode",
                        vec![],
                        [
                            6u8, 31u8, 218u8, 40u8, 203u8, 188u8, 155u8, 242u8, 11u8, 64u8, 196u8,
                            23u8, 70u8, 117u8, 21u8, 42u8, 68u8, 254u8, 90u8, 190u8, 155u8, 117u8,
                            153u8, 198u8, 119u8, 35u8, 52u8, 217u8, 209u8, 144u8, 1u8, 66u8,
                        ],
                    )
                }

                /// A mapping from a contract's code hash to its code.
                pub fn pristine_code(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::pristine_code::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::pristine_code::PristineCode,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "PristineCode",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            6u8, 31u8, 218u8, 40u8, 203u8, 188u8, 155u8, 242u8, 11u8, 64u8, 196u8,
                            23u8, 70u8, 117u8, 21u8, 42u8, 68u8, 254u8, 90u8, 190u8, 155u8, 117u8,
                            153u8, 198u8, 119u8, 35u8, 52u8, 217u8, 209u8, 144u8, 1u8, 66u8,
                        ],
                    )
                }

                /// A mapping from a contract's code hash to its code info.
                pub fn code_info_of_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::code_info_of::CodeInfoOf,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "CodeInfoOf",
                        vec![],
                        [
                            16u8, 119u8, 167u8, 116u8, 213u8, 33u8, 175u8, 218u8, 170u8, 250u8,
                            110u8, 248u8, 215u8, 25u8, 10u8, 143u8, 21u8, 37u8, 88u8, 239u8, 35u8,
                            53u8, 133u8, 126u8, 97u8, 32u8, 60u8, 8u8, 180u8, 123u8, 229u8, 163u8,
                        ],
                    )
                }

                /// A mapping from a contract's code hash to its code info.
                pub fn code_info_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::code_info_of::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::code_info_of::CodeInfoOf,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "CodeInfoOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            16u8, 119u8, 167u8, 116u8, 213u8, 33u8, 175u8, 218u8, 170u8, 250u8,
                            110u8, 248u8, 215u8, 25u8, 10u8, 143u8, 21u8, 37u8, 88u8, 239u8, 35u8,
                            53u8, 133u8, 126u8, 97u8, 32u8, 60u8, 8u8, 180u8, 123u8, 229u8, 163u8,
                        ],
                    )
                }

                /// This is a **monotonic** counter incremented on contract
                /// instantiation.
                ///
                /// This is used in order to generate unique trie ids for
                /// contracts.
                /// The trie id of a new contract is calculated from
                /// hash(account_id, nonce).
                /// The nonce is required because otherwise the following
                /// sequence would lead to
                /// a possible collision of storage:
                ///
                /// 1. Create a new contract.
                /// 2. Terminate the contract.
                /// 3. Immediately recreate the contract with the same
                ///    account_id.
                ///
                /// This is bad because the contents of a trie are deleted
                /// lazily and there might be
                /// storage of the old instantiation still in it when the new
                /// contract is created. Please
                /// note that we can't replace the counter by the block number
                /// because the sequence above
                /// can happen in the same block. We also can't keep the account
                /// counter in memory only
                /// because storage is the only way to communicate across
                /// different extrinsics in the
                /// same block.
                ///
                /// # Note
                ///
                /// Do not use it to determine the number of contracts. It won't
                /// be decremented if
                /// a contract is destroyed.
                pub fn nonce(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::nonce::Nonce,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "Nonce",
                        vec![],
                        [
                            47u8, 101u8, 89u8, 252u8, 98u8, 25u8, 178u8, 154u8, 17u8, 57u8, 185u8,
                            10u8, 133u8, 94u8, 73u8, 160u8, 137u8, 150u8, 97u8, 119u8, 8u8, 146u8,
                            149u8, 146u8, 212u8, 60u8, 141u8, 24u8, 124u8, 28u8, 57u8, 19u8,
                        ],
                    )
                }

                /// The code associated with a given account.
                ///
                /// TWOX-NOTE: SAFE since `AccountId` is a secure hash.
                pub fn contract_info_of_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::contract_info_of::ContractInfoOf,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "ContractInfoOf",
                        vec![],
                        [
                            248u8, 123u8, 214u8, 11u8, 141u8, 157u8, 174u8, 206u8, 251u8, 239u8,
                            184u8, 167u8, 218u8, 140u8, 245u8, 159u8, 190u8, 198u8, 167u8, 196u8,
                            205u8, 229u8, 6u8, 194u8, 88u8, 26u8, 57u8, 94u8, 140u8, 76u8, 8u8,
                            144u8,
                        ],
                    )
                }

                /// The code associated with a given account.
                ///
                /// TWOX-NOTE: SAFE since `AccountId` is a secure hash.
                pub fn contract_info_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::contract_info_of::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::contract_info_of::ContractInfoOf,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "ContractInfoOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            248u8, 123u8, 214u8, 11u8, 141u8, 157u8, 174u8, 206u8, 251u8, 239u8,
                            184u8, 167u8, 218u8, 140u8, 245u8, 159u8, 190u8, 198u8, 167u8, 196u8,
                            205u8, 229u8, 6u8, 194u8, 88u8, 26u8, 57u8, 94u8, 140u8, 76u8, 8u8,
                            144u8,
                        ],
                    )
                }

                /// Evicted contracts that await child trie deletion.
                ///
                /// Child trie deletion is a heavy operation depending on the
                /// amount of storage items
                /// stored in said trie. Therefore this operation is performed
                /// lazily in `on_idle`.
                pub fn deletion_queue_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::deletion_queue::DeletionQueue,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "DeletionQueue",
                        vec![],
                        [
                            233u8, 193u8, 191u8, 44u8, 151u8, 46u8, 124u8, 188u8, 132u8, 227u8,
                            107u8, 210u8, 37u8, 110u8, 172u8, 95u8, 12u8, 114u8, 63u8, 83u8, 60u8,
                            163u8, 58u8, 174u8, 160u8, 47u8, 198u8, 156u8, 216u8, 182u8, 65u8,
                            229u8,
                        ],
                    )
                }

                /// Evicted contracts that await child trie deletion.
                ///
                /// Child trie deletion is a heavy operation depending on the
                /// amount of storage items
                /// stored in said trie. Therefore this operation is performed
                /// lazily in `on_idle`.
                pub fn deletion_queue(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::deletion_queue::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::deletion_queue::DeletionQueue,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "DeletionQueue",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            233u8, 193u8, 191u8, 44u8, 151u8, 46u8, 124u8, 188u8, 132u8, 227u8,
                            107u8, 210u8, 37u8, 110u8, 172u8, 95u8, 12u8, 114u8, 63u8, 83u8, 60u8,
                            163u8, 58u8, 174u8, 160u8, 47u8, 198u8, 156u8, 216u8, 182u8, 65u8,
                            229u8,
                        ],
                    )
                }

                /// A pair of monotonic counters used to track the latest
                /// contract marked for deletion
                /// and the latest deleted contract in queue.
                pub fn deletion_queue_counter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::deletion_queue_counter::DeletionQueueCounter,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "DeletionQueueCounter",
                        vec![],
                        [
                            124u8, 63u8, 32u8, 109u8, 8u8, 113u8, 105u8, 172u8, 87u8, 88u8, 244u8,
                            191u8, 252u8, 196u8, 10u8, 137u8, 101u8, 87u8, 124u8, 220u8, 178u8,
                            155u8, 163u8, 214u8, 116u8, 121u8, 129u8, 129u8, 173u8, 76u8, 188u8,
                            41u8,
                        ],
                    )
                }

                /// A migration can span across multiple blocks. This storage
                /// defines a cursor to track the
                /// progress of the migration, enabling us to resume from the
                /// last completed position.
                pub fn migration_in_progress(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::migration_in_progress::MigrationInProgress,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Contracts",
                        "MigrationInProgress",
                        vec![],
                        [
                            238u8, 96u8, 248u8, 141u8, 247u8, 233u8, 27u8, 21u8, 187u8, 56u8,
                            195u8, 67u8, 21u8, 215u8, 30u8, 236u8, 151u8, 163u8, 115u8, 117u8,
                            154u8, 54u8, 37u8, 240u8, 136u8, 240u8, 35u8, 192u8, 168u8, 250u8,
                            132u8, 63u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// Cost schedule and limits.
                pub fn schedule(
                    &self,
                ) -> ::subxt::constants::Address<runtime_types::pallet_contracts::schedule::Schedule>
                {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "Schedule",
                        [
                            206u8, 138u8, 245u8, 112u8, 185u8, 93u8, 92u8, 41u8, 132u8, 217u8,
                            98u8, 105u8, 54u8, 227u8, 212u8, 56u8, 48u8, 0u8, 251u8, 95u8, 56u8,
                            140u8, 22u8, 211u8, 105u8, 238u8, 183u8, 142u8, 65u8, 20u8, 13u8, 80u8,
                        ],
                    )
                }

                /// The amount of balance a caller has to pay for each byte of
                /// storage.
                ///
                /// # Note
                ///
                /// Changing this value for an existing chain might need a
                /// storage migration.
                pub fn deposit_per_byte(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "DepositPerByte",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// Fallback value to limit the storage deposit if it's not
                /// being set by the caller.
                pub fn default_deposit_limit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "DefaultDepositLimit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The amount of balance a caller has to pay for each storage
                /// item.
                ///
                /// # Note
                ///
                /// Changing this value for an existing chain might need a
                /// storage migration.
                pub fn deposit_per_item(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "DepositPerItem",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The maximum length of a contract code in bytes.
                ///
                /// The value should be chosen carefully taking into the account
                /// the overall memory limit
                /// your runtime has, as well as the [maximum allowed callstack
                /// depth](#associatedtype.CallStack). Look into the
                /// `integrity_test()` for some insights.
                pub fn max_code_len(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "MaxCodeLen",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// The maximum allowable length in bytes for storage keys.
                pub fn max_storage_key_len(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "MaxStorageKeyLen",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// Make contract callable functions marked as `#[unstable]`
                /// available.
                ///
                /// Contracts that use `#[unstable]` functions won't be able to
                /// be uploaded unless
                /// this is set to `true`. This is only meant for testnets and
                /// dev nodes in order to
                /// experiment with new features.
                ///
                /// # Warning
                ///
                /// Do **not** set to `true` on productions chains.
                pub fn unsafe_unstable_interface(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::bool> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "UnsafeUnstableInterface",
                        [
                            165u8, 28u8, 112u8, 190u8, 18u8, 129u8, 182u8, 206u8, 237u8, 1u8, 68u8,
                            252u8, 125u8, 234u8, 185u8, 50u8, 149u8, 164u8, 47u8, 126u8, 134u8,
                            100u8, 14u8, 86u8, 209u8, 39u8, 20u8, 4u8, 233u8, 115u8, 102u8, 131u8,
                        ],
                    )
                }

                /// The maximum length of the debug buffer in bytes.
                pub fn max_debug_buffer_len(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Contracts",
                        "MaxDebugBufferLen",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod assets {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::pallet_assets::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::pallet_assets::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::create`].
                pub struct Create {
                    #[codec(compact)]
                    pub id: create::Id,
                    pub admin: create::Admin,
                    pub min_balance: create::MinBalance,
                }
                pub mod create {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Admin = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type MinBalance = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for Create {
                    const CALL: &'static str = "create";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_create`].
                pub struct ForceCreate {
                    #[codec(compact)]
                    pub id: force_create::Id,
                    pub owner: force_create::Owner,
                    pub is_sufficient: force_create::IsSufficient,
                    #[codec(compact)]
                    pub min_balance: force_create::MinBalance,
                }
                pub mod force_create {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type IsSufficient = ::core::primitive::bool;
                    pub type MinBalance = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceCreate {
                    const CALL: &'static str = "force_create";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::start_destroy`].
                pub struct StartDestroy {
                    #[codec(compact)]
                    pub id: start_destroy::Id,
                }
                pub mod start_destroy {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for StartDestroy {
                    const CALL: &'static str = "start_destroy";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::destroy_accounts`].
                pub struct DestroyAccounts {
                    #[codec(compact)]
                    pub id: destroy_accounts::Id,
                }
                pub mod destroy_accounts {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for DestroyAccounts {
                    const CALL: &'static str = "destroy_accounts";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::destroy_approvals`].
                pub struct DestroyApprovals {
                    #[codec(compact)]
                    pub id: destroy_approvals::Id,
                }
                pub mod destroy_approvals {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for DestroyApprovals {
                    const CALL: &'static str = "destroy_approvals";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::finish_destroy`].
                pub struct FinishDestroy {
                    #[codec(compact)]
                    pub id: finish_destroy::Id,
                }
                pub mod finish_destroy {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for FinishDestroy {
                    const CALL: &'static str = "finish_destroy";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::mint`].
                pub struct Mint {
                    #[codec(compact)]
                    pub id: mint::Id,
                    pub beneficiary: mint::Beneficiary,
                    #[codec(compact)]
                    pub amount: mint::Amount,
                }
                pub mod mint {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Beneficiary =
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for Mint {
                    const CALL: &'static str = "mint";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::burn`].
                pub struct Burn {
                    #[codec(compact)]
                    pub id: burn::Id,
                    pub who: burn::Who,
                    #[codec(compact)]
                    pub amount: burn::Amount,
                }
                pub mod burn {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for Burn {
                    const CALL: &'static str = "burn";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::transfer`].
                pub struct Transfer {
                    #[codec(compact)]
                    pub id: transfer::Id,
                    pub target: transfer::Target,
                    #[codec(compact)]
                    pub amount: transfer::Amount,
                }
                pub mod transfer {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Target = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for Transfer {
                    const CALL: &'static str = "transfer";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::transfer_keep_alive`].
                pub struct TransferKeepAlive {
                    #[codec(compact)]
                    pub id: transfer_keep_alive::Id,
                    pub target: transfer_keep_alive::Target,
                    #[codec(compact)]
                    pub amount: transfer_keep_alive::Amount,
                }
                pub mod transfer_keep_alive {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Target = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferKeepAlive {
                    const CALL: &'static str = "transfer_keep_alive";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_transfer`].
                pub struct ForceTransfer {
                    #[codec(compact)]
                    pub id: force_transfer::Id,
                    pub source: force_transfer::Source,
                    pub dest: force_transfer::Dest,
                    #[codec(compact)]
                    pub amount: force_transfer::Amount,
                }
                pub mod force_transfer {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Source = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Dest = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceTransfer {
                    const CALL: &'static str = "force_transfer";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::freeze`].
                pub struct Freeze {
                    #[codec(compact)]
                    pub id: freeze::Id,
                    pub who: freeze::Who,
                }
                pub mod freeze {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for Freeze {
                    const CALL: &'static str = "freeze";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::thaw`].
                pub struct Thaw {
                    #[codec(compact)]
                    pub id: thaw::Id,
                    pub who: thaw::Who,
                }
                pub mod thaw {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for Thaw {
                    const CALL: &'static str = "thaw";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::freeze_asset`].
                pub struct FreezeAsset {
                    #[codec(compact)]
                    pub id: freeze_asset::Id,
                }
                pub mod freeze_asset {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for FreezeAsset {
                    const CALL: &'static str = "freeze_asset";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::thaw_asset`].
                pub struct ThawAsset {
                    #[codec(compact)]
                    pub id: thaw_asset::Id,
                }
                pub mod thaw_asset {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for ThawAsset {
                    const CALL: &'static str = "thaw_asset";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::transfer_ownership`].
                pub struct TransferOwnership {
                    #[codec(compact)]
                    pub id: transfer_ownership::Id,
                    pub owner: transfer_ownership::Owner,
                }
                pub mod transfer_ownership {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferOwnership {
                    const CALL: &'static str = "transfer_ownership";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_team`].
                pub struct SetTeam {
                    #[codec(compact)]
                    pub id: set_team::Id,
                    pub issuer: set_team::Issuer,
                    pub admin: set_team::Admin,
                    pub freezer: set_team::Freezer,
                }
                pub mod set_team {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Issuer = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Admin = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Freezer =
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetTeam {
                    const CALL: &'static str = "set_team";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_metadata`].
                pub struct SetMetadata {
                    #[codec(compact)]
                    pub id: set_metadata::Id,
                    pub name: set_metadata::Name,
                    pub symbol: set_metadata::Symbol,
                    pub decimals: set_metadata::Decimals,
                }
                pub mod set_metadata {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Symbol = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Decimals = ::core::primitive::u8;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetMetadata {
                    const CALL: &'static str = "set_metadata";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::clear_metadata`].
                pub struct ClearMetadata {
                    #[codec(compact)]
                    pub id: clear_metadata::Id,
                }
                pub mod clear_metadata {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for ClearMetadata {
                    const CALL: &'static str = "clear_metadata";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_set_metadata`].
                pub struct ForceSetMetadata {
                    #[codec(compact)]
                    pub id: force_set_metadata::Id,
                    pub name: force_set_metadata::Name,
                    pub symbol: force_set_metadata::Symbol,
                    pub decimals: force_set_metadata::Decimals,
                    pub is_frozen: force_set_metadata::IsFrozen,
                }
                pub mod force_set_metadata {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Symbol = ::std::vec::Vec<::core::primitive::u8>;
                    pub type Decimals = ::core::primitive::u8;
                    pub type IsFrozen = ::core::primitive::bool;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceSetMetadata {
                    const CALL: &'static str = "force_set_metadata";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_clear_metadata`].
                pub struct ForceClearMetadata {
                    #[codec(compact)]
                    pub id: force_clear_metadata::Id,
                }
                pub mod force_clear_metadata {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceClearMetadata {
                    const CALL: &'static str = "force_clear_metadata";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_asset_status`].
                pub struct ForceAssetStatus {
                    #[codec(compact)]
                    pub id: force_asset_status::Id,
                    pub owner: force_asset_status::Owner,
                    pub issuer: force_asset_status::Issuer,
                    pub admin: force_asset_status::Admin,
                    pub freezer: force_asset_status::Freezer,
                    #[codec(compact)]
                    pub min_balance: force_asset_status::MinBalance,
                    pub is_sufficient: force_asset_status::IsSufficient,
                    pub is_frozen: force_asset_status::IsFrozen,
                }
                pub mod force_asset_status {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Issuer = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Admin = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Freezer =
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type MinBalance = ::core::primitive::u128;
                    pub type IsSufficient = ::core::primitive::bool;
                    pub type IsFrozen = ::core::primitive::bool;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceAssetStatus {
                    const CALL: &'static str = "force_asset_status";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::approve_transfer`].
                pub struct ApproveTransfer {
                    #[codec(compact)]
                    pub id: approve_transfer::Id,
                    pub delegate: approve_transfer::Delegate,
                    #[codec(compact)]
                    pub amount: approve_transfer::Amount,
                }
                pub mod approve_transfer {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Delegate =
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for ApproveTransfer {
                    const CALL: &'static str = "approve_transfer";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::cancel_approval`].
                pub struct CancelApproval {
                    #[codec(compact)]
                    pub id: cancel_approval::Id,
                    pub delegate: cancel_approval::Delegate,
                }
                pub mod cancel_approval {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Delegate =
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for CancelApproval {
                    const CALL: &'static str = "cancel_approval";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::force_cancel_approval`].
                pub struct ForceCancelApproval {
                    #[codec(compact)]
                    pub id: force_cancel_approval::Id,
                    pub owner: force_cancel_approval::Owner,
                    pub delegate: force_cancel_approval::Delegate,
                }
                pub mod force_cancel_approval {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Delegate =
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for ForceCancelApproval {
                    const CALL: &'static str = "force_cancel_approval";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::transfer_approved`].
                pub struct TransferApproved {
                    #[codec(compact)]
                    pub id: transfer_approved::Id,
                    pub owner: transfer_approved::Owner,
                    pub destination: transfer_approved::Destination,
                    #[codec(compact)]
                    pub amount: transfer_approved::Amount,
                }
                pub mod transfer_approved {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Owner = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Destination =
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for TransferApproved {
                    const CALL: &'static str = "transfer_approved";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::touch`].
                pub struct Touch {
                    #[codec(compact)]
                    pub id: touch::Id,
                }
                pub mod touch {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for Touch {
                    const CALL: &'static str = "touch";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::refund`].
                pub struct Refund {
                    #[codec(compact)]
                    pub id: refund::Id,
                    pub allow_burn: refund::AllowBurn,
                }
                pub mod refund {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type AllowBurn = ::core::primitive::bool;
                }
                impl ::subxt::blocks::StaticExtrinsic for Refund {
                    const CALL: &'static str = "refund";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_min_balance`].
                pub struct SetMinBalance {
                    #[codec(compact)]
                    pub id: set_min_balance::Id,
                    pub min_balance: set_min_balance::MinBalance,
                }
                pub mod set_min_balance {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type MinBalance = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetMinBalance {
                    const CALL: &'static str = "set_min_balance";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::touch_other`].
                pub struct TouchOther {
                    #[codec(compact)]
                    pub id: touch_other::Id,
                    pub who: touch_other::Who,
                }
                pub mod touch_other {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for TouchOther {
                    const CALL: &'static str = "touch_other";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::refund_other`].
                pub struct RefundOther {
                    #[codec(compact)]
                    pub id: refund_other::Id,
                    pub who: refund_other::Who,
                }
                pub mod refund_other {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for RefundOther {
                    const CALL: &'static str = "refund_other";
                    const PALLET: &'static str = "Assets";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::block`].
                pub struct Block {
                    #[codec(compact)]
                    pub id: block::Id,
                    pub who: block::Who,
                }
                pub mod block {
                    use super::runtime_types;
                    pub type Id = ::core::primitive::u32;
                    pub type Who = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for Block {
                    const CALL: &'static str = "block";
                    const PALLET: &'static str = "Assets";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::create`].
                pub fn create(
                    &self,
                    id: types::create::Id,
                    admin: types::create::Admin,
                    min_balance: types::create::MinBalance,
                ) -> ::subxt::tx::Payload<types::Create> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "create",
                        types::Create {
                            id,
                            admin,
                            min_balance,
                        },
                        [
                            120u8, 25u8, 99u8, 39u8, 102u8, 201u8, 14u8, 2u8, 32u8, 139u8, 206u8,
                            218u8, 223u8, 161u8, 25u8, 98u8, 159u8, 133u8, 65u8, 105u8, 45u8, 4u8,
                            28u8, 49u8, 248u8, 147u8, 2u8, 179u8, 11u8, 195u8, 177u8, 250u8,
                        ],
                    )
                }

                /// See [`Pallet::force_create`].
                pub fn force_create(
                    &self,
                    id: types::force_create::Id,
                    owner: types::force_create::Owner,
                    is_sufficient: types::force_create::IsSufficient,
                    min_balance: types::force_create::MinBalance,
                ) -> ::subxt::tx::Payload<types::ForceCreate> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "force_create",
                        types::ForceCreate {
                            id,
                            owner,
                            is_sufficient,
                            min_balance,
                        },
                        [
                            149u8, 41u8, 54u8, 146u8, 18u8, 248u8, 84u8, 52u8, 202u8, 88u8, 192u8,
                            208u8, 247u8, 227u8, 254u8, 98u8, 92u8, 46u8, 164u8, 152u8, 143u8,
                            20u8, 179u8, 227u8, 197u8, 247u8, 242u8, 153u8, 142u8, 148u8, 40u8,
                            184u8,
                        ],
                    )
                }

                /// See [`Pallet::start_destroy`].
                pub fn start_destroy(
                    &self,
                    id: types::start_destroy::Id,
                ) -> ::subxt::tx::Payload<types::StartDestroy> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "start_destroy",
                        types::StartDestroy {
                            id,
                        },
                        [
                            125u8, 82u8, 151u8, 106u8, 25u8, 49u8, 68u8, 203u8, 247u8, 175u8,
                            117u8, 230u8, 84u8, 98u8, 172u8, 73u8, 233u8, 218u8, 212u8, 198u8,
                            69u8, 35u8, 15u8, 179u8, 161u8, 205u8, 190u8, 109u8, 198u8, 214u8,
                            65u8, 164u8,
                        ],
                    )
                }

                /// See [`Pallet::destroy_accounts`].
                pub fn destroy_accounts(
                    &self,
                    id: types::destroy_accounts::Id,
                ) -> ::subxt::tx::Payload<types::DestroyAccounts> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "destroy_accounts",
                        types::DestroyAccounts {
                            id,
                        },
                        [
                            236u8, 102u8, 233u8, 170u8, 179u8, 46u8, 42u8, 29u8, 200u8, 116u8,
                            62u8, 114u8, 233u8, 59u8, 217u8, 215u8, 109u8, 232u8, 147u8, 95u8,
                            255u8, 248u8, 119u8, 222u8, 216u8, 165u8, 138u8, 47u8, 28u8, 56u8,
                            204u8, 93u8,
                        ],
                    )
                }

                /// See [`Pallet::destroy_approvals`].
                pub fn destroy_approvals(
                    &self,
                    id: types::destroy_approvals::Id,
                ) -> ::subxt::tx::Payload<types::DestroyApprovals> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "destroy_approvals",
                        types::DestroyApprovals {
                            id,
                        },
                        [
                            34u8, 35u8, 15u8, 44u8, 239u8, 232u8, 88u8, 130u8, 130u8, 87u8, 171u8,
                            255u8, 247u8, 179u8, 14u8, 35u8, 47u8, 223u8, 32u8, 232u8, 41u8, 105u8,
                            207u8, 199u8, 90u8, 136u8, 144u8, 139u8, 252u8, 76u8, 177u8, 106u8,
                        ],
                    )
                }

                /// See [`Pallet::finish_destroy`].
                pub fn finish_destroy(
                    &self,
                    id: types::finish_destroy::Id,
                ) -> ::subxt::tx::Payload<types::FinishDestroy> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "finish_destroy",
                        types::FinishDestroy {
                            id,
                        },
                        [
                            132u8, 67u8, 78u8, 84u8, 240u8, 51u8, 176u8, 119u8, 48u8, 34u8, 153u8,
                            37u8, 25u8, 171u8, 21u8, 164u8, 53u8, 214u8, 36u8, 149u8, 20u8, 240u8,
                            123u8, 195u8, 170u8, 162u8, 118u8, 81u8, 176u8, 218u8, 114u8, 113u8,
                        ],
                    )
                }

                /// See [`Pallet::mint`].
                pub fn mint(
                    &self,
                    id: types::mint::Id,
                    beneficiary: types::mint::Beneficiary,
                    amount: types::mint::Amount,
                ) -> ::subxt::tx::Payload<types::Mint> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "mint",
                        types::Mint {
                            id,
                            beneficiary,
                            amount,
                        },
                        [
                            172u8, 131u8, 103u8, 81u8, 206u8, 2u8, 143u8, 114u8, 137u8, 60u8,
                            147u8, 67u8, 226u8, 64u8, 71u8, 11u8, 36u8, 145u8, 51u8, 8u8, 0u8,
                            110u8, 8u8, 195u8, 103u8, 205u8, 156u8, 43u8, 215u8, 12u8, 150u8,
                            135u8,
                        ],
                    )
                }

                /// See [`Pallet::burn`].
                pub fn burn(
                    &self,
                    id: types::burn::Id,
                    who: types::burn::Who,
                    amount: types::burn::Amount,
                ) -> ::subxt::tx::Payload<types::Burn> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "burn",
                        types::Burn {
                            id,
                            who,
                            amount,
                        },
                        [
                            105u8, 133u8, 82u8, 100u8, 124u8, 65u8, 174u8, 31u8, 152u8, 45u8, 23u8,
                            200u8, 23u8, 199u8, 239u8, 8u8, 187u8, 142u8, 21u8, 192u8, 35u8, 211u8,
                            172u8, 130u8, 169u8, 74u8, 167u8, 36u8, 149u8, 7u8, 19u8, 37u8,
                        ],
                    )
                }

                /// See [`Pallet::transfer`].
                pub fn transfer(
                    &self,
                    id: types::transfer::Id,
                    target: types::transfer::Target,
                    amount: types::transfer::Amount,
                ) -> ::subxt::tx::Payload<types::Transfer> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "transfer",
                        types::Transfer {
                            id,
                            target,
                            amount,
                        },
                        [
                            126u8, 31u8, 70u8, 179u8, 222u8, 190u8, 12u8, 19u8, 94u8, 225u8, 217u8,
                            109u8, 54u8, 69u8, 124u8, 61u8, 97u8, 199u8, 193u8, 166u8, 39u8, 143u8,
                            125u8, 251u8, 87u8, 173u8, 149u8, 91u8, 182u8, 18u8, 184u8, 65u8,
                        ],
                    )
                }

                /// See [`Pallet::transfer_keep_alive`].
                pub fn transfer_keep_alive(
                    &self,
                    id: types::transfer_keep_alive::Id,
                    target: types::transfer_keep_alive::Target,
                    amount: types::transfer_keep_alive::Amount,
                ) -> ::subxt::tx::Payload<types::TransferKeepAlive> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "transfer_keep_alive",
                        types::TransferKeepAlive {
                            id,
                            target,
                            amount,
                        },
                        [
                            99u8, 101u8, 219u8, 188u8, 238u8, 230u8, 141u8, 43u8, 38u8, 175u8,
                            46u8, 89u8, 33u8, 23u8, 223u8, 115u8, 108u8, 18u8, 190u8, 213u8, 157u8,
                            12u8, 139u8, 97u8, 7u8, 75u8, 196u8, 159u8, 122u8, 32u8, 164u8, 154u8,
                        ],
                    )
                }

                /// See [`Pallet::force_transfer`].
                pub fn force_transfer(
                    &self,
                    id: types::force_transfer::Id,
                    source: types::force_transfer::Source,
                    dest: types::force_transfer::Dest,
                    amount: types::force_transfer::Amount,
                ) -> ::subxt::tx::Payload<types::ForceTransfer> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "force_transfer",
                        types::ForceTransfer {
                            id,
                            source,
                            dest,
                            amount,
                        },
                        [
                            10u8, 210u8, 8u8, 209u8, 8u8, 78u8, 40u8, 213u8, 235u8, 176u8, 144u8,
                            145u8, 70u8, 13u8, 75u8, 72u8, 166u8, 137u8, 22u8, 191u8, 226u8, 244u8,
                            92u8, 183u8, 129u8, 212u8, 158u8, 179u8, 169u8, 232u8, 177u8, 225u8,
                        ],
                    )
                }

                /// See [`Pallet::freeze`].
                pub fn freeze(
                    &self,
                    id: types::freeze::Id,
                    who: types::freeze::Who,
                ) -> ::subxt::tx::Payload<types::Freeze> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "freeze",
                        types::Freeze {
                            id,
                            who,
                        },
                        [
                            180u8, 124u8, 252u8, 66u8, 205u8, 23u8, 32u8, 217u8, 173u8, 10u8, 91u8,
                            57u8, 44u8, 215u8, 234u8, 152u8, 115u8, 38u8, 141u8, 212u8, 57u8,
                            217u8, 169u8, 61u8, 215u8, 130u8, 172u8, 58u8, 90u8, 193u8, 25u8,
                            249u8,
                        ],
                    )
                }

                /// See [`Pallet::thaw`].
                pub fn thaw(
                    &self,
                    id: types::thaw::Id,
                    who: types::thaw::Who,
                ) -> ::subxt::tx::Payload<types::Thaw> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "thaw",
                        types::Thaw {
                            id,
                            who,
                        },
                        [
                            187u8, 130u8, 9u8, 152u8, 231u8, 9u8, 245u8, 162u8, 115u8, 19u8, 73u8,
                            176u8, 16u8, 230u8, 30u8, 60u8, 180u8, 183u8, 154u8, 160u8, 72u8,
                            219u8, 116u8, 57u8, 140u8, 6u8, 105u8, 38u8, 98u8, 90u8, 250u8, 135u8,
                        ],
                    )
                }

                /// See [`Pallet::freeze_asset`].
                pub fn freeze_asset(
                    &self,
                    id: types::freeze_asset::Id,
                ) -> ::subxt::tx::Payload<types::FreezeAsset> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "freeze_asset",
                        types::FreezeAsset {
                            id,
                        },
                        [
                            75u8, 237u8, 183u8, 112u8, 112u8, 123u8, 250u8, 203u8, 169u8, 51u8,
                            218u8, 35u8, 159u8, 23u8, 21u8, 10u8, 167u8, 84u8, 161u8, 212u8, 124u8,
                            236u8, 88u8, 175u8, 48u8, 195u8, 33u8, 145u8, 141u8, 156u8, 31u8,
                            250u8,
                        ],
                    )
                }

                /// See [`Pallet::thaw_asset`].
                pub fn thaw_asset(
                    &self,
                    id: types::thaw_asset::Id,
                ) -> ::subxt::tx::Payload<types::ThawAsset> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "thaw_asset",
                        types::ThawAsset {
                            id,
                        },
                        [
                            151u8, 6u8, 170u8, 114u8, 55u8, 8u8, 5u8, 194u8, 251u8, 78u8, 232u8,
                            181u8, 157u8, 62u8, 16u8, 39u8, 79u8, 119u8, 205u8, 198u8, 199u8, 26u8,
                            92u8, 162u8, 169u8, 173u8, 93u8, 51u8, 7u8, 79u8, 198u8, 77u8,
                        ],
                    )
                }

                /// See [`Pallet::transfer_ownership`].
                pub fn transfer_ownership(
                    &self,
                    id: types::transfer_ownership::Id,
                    owner: types::transfer_ownership::Owner,
                ) -> ::subxt::tx::Payload<types::TransferOwnership> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "transfer_ownership",
                        types::TransferOwnership {
                            id,
                            owner,
                        },
                        [
                            65u8, 85u8, 40u8, 202u8, 212u8, 170u8, 130u8, 132u8, 140u8, 90u8, 68u8,
                            28u8, 101u8, 154u8, 222u8, 150u8, 244u8, 165u8, 44u8, 22u8, 225u8,
                            152u8, 7u8, 162u8, 110u8, 54u8, 173u8, 181u8, 54u8, 215u8, 105u8,
                            239u8,
                        ],
                    )
                }

                /// See [`Pallet::set_team`].
                pub fn set_team(
                    &self,
                    id: types::set_team::Id,
                    issuer: types::set_team::Issuer,
                    admin: types::set_team::Admin,
                    freezer: types::set_team::Freezer,
                ) -> ::subxt::tx::Payload<types::SetTeam> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "set_team",
                        types::SetTeam {
                            id,
                            issuer,
                            admin,
                            freezer,
                        },
                        [
                            52u8, 75u8, 50u8, 30u8, 164u8, 161u8, 121u8, 25u8, 135u8, 83u8, 115u8,
                            25u8, 103u8, 1u8, 124u8, 206u8, 83u8, 182u8, 41u8, 116u8, 44u8, 37u8,
                            75u8, 70u8, 252u8, 225u8, 240u8, 144u8, 96u8, 160u8, 151u8, 4u8,
                        ],
                    )
                }

                /// See [`Pallet::set_metadata`].
                pub fn set_metadata(
                    &self,
                    id: types::set_metadata::Id,
                    name: types::set_metadata::Name,
                    symbol: types::set_metadata::Symbol,
                    decimals: types::set_metadata::Decimals,
                ) -> ::subxt::tx::Payload<types::SetMetadata> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "set_metadata",
                        types::SetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                        },
                        [
                            215u8, 66u8, 15u8, 17u8, 88u8, 174u8, 77u8, 75u8, 229u8, 155u8, 160u8,
                            34u8, 108u8, 194u8, 88u8, 238u8, 131u8, 97u8, 234u8, 102u8, 71u8, 56u8,
                            70u8, 248u8, 211u8, 85u8, 72u8, 92u8, 71u8, 222u8, 190u8, 91u8,
                        ],
                    )
                }

                /// See [`Pallet::clear_metadata`].
                pub fn clear_metadata(
                    &self,
                    id: types::clear_metadata::Id,
                ) -> ::subxt::tx::Payload<types::ClearMetadata> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "clear_metadata",
                        types::ClearMetadata {
                            id,
                        },
                        [
                            68u8, 172u8, 6u8, 158u8, 237u8, 254u8, 22u8, 4u8, 254u8, 157u8, 179u8,
                            168u8, 105u8, 114u8, 56u8, 166u8, 213u8, 38u8, 188u8, 195u8, 99u8,
                            43u8, 142u8, 220u8, 94u8, 248u8, 51u8, 226u8, 233u8, 114u8, 86u8, 93u8,
                        ],
                    )
                }

                /// See [`Pallet::force_set_metadata`].
                pub fn force_set_metadata(
                    &self,
                    id: types::force_set_metadata::Id,
                    name: types::force_set_metadata::Name,
                    symbol: types::force_set_metadata::Symbol,
                    decimals: types::force_set_metadata::Decimals,
                    is_frozen: types::force_set_metadata::IsFrozen,
                ) -> ::subxt::tx::Payload<types::ForceSetMetadata> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "force_set_metadata",
                        types::ForceSetMetadata {
                            id,
                            name,
                            symbol,
                            decimals,
                            is_frozen,
                        },
                        [
                            76u8, 90u8, 182u8, 13u8, 133u8, 248u8, 94u8, 136u8, 169u8, 114u8,
                            151u8, 20u8, 106u8, 89u8, 78u8, 228u8, 22u8, 29u8, 68u8, 8u8, 54u8,
                            47u8, 1u8, 186u8, 45u8, 167u8, 14u8, 112u8, 34u8, 43u8, 91u8, 140u8,
                        ],
                    )
                }

                /// See [`Pallet::force_clear_metadata`].
                pub fn force_clear_metadata(
                    &self,
                    id: types::force_clear_metadata::Id,
                ) -> ::subxt::tx::Payload<types::ForceClearMetadata> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "force_clear_metadata",
                        types::ForceClearMetadata {
                            id,
                        },
                        [
                            2u8, 224u8, 84u8, 48u8, 130u8, 132u8, 79u8, 38u8, 217u8, 17u8, 165u8,
                            139u8, 89u8, 53u8, 116u8, 184u8, 32u8, 91u8, 122u8, 39u8, 85u8, 40u8,
                            213u8, 216u8, 135u8, 171u8, 50u8, 69u8, 202u8, 28u8, 166u8, 147u8,
                        ],
                    )
                }

                /// See [`Pallet::force_asset_status`].
                pub fn force_asset_status(
                    &self,
                    id: types::force_asset_status::Id,
                    owner: types::force_asset_status::Owner,
                    issuer: types::force_asset_status::Issuer,
                    admin: types::force_asset_status::Admin,
                    freezer: types::force_asset_status::Freezer,
                    min_balance: types::force_asset_status::MinBalance,
                    is_sufficient: types::force_asset_status::IsSufficient,
                    is_frozen: types::force_asset_status::IsFrozen,
                ) -> ::subxt::tx::Payload<types::ForceAssetStatus> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "force_asset_status",
                        types::ForceAssetStatus {
                            id,
                            owner,
                            issuer,
                            admin,
                            freezer,
                            min_balance,
                            is_sufficient,
                            is_frozen,
                        },
                        [
                            149u8, 136u8, 250u8, 33u8, 53u8, 220u8, 207u8, 187u8, 42u8, 118u8,
                            93u8, 173u8, 100u8, 243u8, 234u8, 207u8, 88u8, 45u8, 79u8, 221u8,
                            113u8, 166u8, 229u8, 171u8, 223u8, 126u8, 20u8, 67u8, 19u8, 77u8, 44u8,
                            19u8,
                        ],
                    )
                }

                /// See [`Pallet::approve_transfer`].
                pub fn approve_transfer(
                    &self,
                    id: types::approve_transfer::Id,
                    delegate: types::approve_transfer::Delegate,
                    amount: types::approve_transfer::Amount,
                ) -> ::subxt::tx::Payload<types::ApproveTransfer> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "approve_transfer",
                        types::ApproveTransfer {
                            id,
                            delegate,
                            amount,
                        },
                        [
                            39u8, 227u8, 23u8, 143u8, 10u8, 120u8, 227u8, 1u8, 223u8, 78u8, 40u8,
                            213u8, 249u8, 175u8, 170u8, 183u8, 10u8, 244u8, 117u8, 111u8, 140u8,
                            157u8, 153u8, 212u8, 94u8, 119u8, 213u8, 44u8, 41u8, 8u8, 114u8, 200u8,
                        ],
                    )
                }

                /// See [`Pallet::cancel_approval`].
                pub fn cancel_approval(
                    &self,
                    id: types::cancel_approval::Id,
                    delegate: types::cancel_approval::Delegate,
                ) -> ::subxt::tx::Payload<types::CancelApproval> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "cancel_approval",
                        types::CancelApproval {
                            id,
                            delegate,
                        },
                        [
                            74u8, 117u8, 101u8, 78u8, 152u8, 208u8, 16u8, 102u8, 34u8, 195u8, 61u8,
                            36u8, 85u8, 91u8, 253u8, 182u8, 61u8, 199u8, 12u8, 102u8, 149u8, 20u8,
                            238u8, 207u8, 236u8, 50u8, 63u8, 249u8, 34u8, 85u8, 88u8, 229u8,
                        ],
                    )
                }

                /// See [`Pallet::force_cancel_approval`].
                pub fn force_cancel_approval(
                    &self,
                    id: types::force_cancel_approval::Id,
                    owner: types::force_cancel_approval::Owner,
                    delegate: types::force_cancel_approval::Delegate,
                ) -> ::subxt::tx::Payload<types::ForceCancelApproval> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "force_cancel_approval",
                        types::ForceCancelApproval {
                            id,
                            owner,
                            delegate,
                        },
                        [
                            27u8, 231u8, 85u8, 241u8, 18u8, 151u8, 64u8, 234u8, 11u8, 84u8, 252u8,
                            128u8, 44u8, 247u8, 132u8, 82u8, 34u8, 210u8, 202u8, 50u8, 158u8, 45u8,
                            239u8, 192u8, 7u8, 24u8, 39u8, 95u8, 57u8, 21u8, 178u8, 113u8,
                        ],
                    )
                }

                /// See [`Pallet::transfer_approved`].
                pub fn transfer_approved(
                    &self,
                    id: types::transfer_approved::Id,
                    owner: types::transfer_approved::Owner,
                    destination: types::transfer_approved::Destination,
                    amount: types::transfer_approved::Amount,
                ) -> ::subxt::tx::Payload<types::TransferApproved> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "transfer_approved",
                        types::TransferApproved {
                            id,
                            owner,
                            destination,
                            amount,
                        },
                        [
                            214u8, 51u8, 243u8, 129u8, 116u8, 233u8, 199u8, 183u8, 25u8, 5u8,
                            109u8, 85u8, 255u8, 68u8, 36u8, 99u8, 99u8, 179u8, 34u8, 66u8, 65u8,
                            82u8, 189u8, 174u8, 22u8, 100u8, 211u8, 13u8, 178u8, 19u8, 128u8,
                            177u8,
                        ],
                    )
                }

                /// See [`Pallet::touch`].
                pub fn touch(&self, id: types::touch::Id) -> ::subxt::tx::Payload<types::Touch> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "touch",
                        types::Touch {
                            id,
                        },
                        [
                            50u8, 185u8, 46u8, 134u8, 136u8, 31u8, 191u8, 34u8, 215u8, 150u8, 73u8,
                            103u8, 140u8, 36u8, 95u8, 156u8, 201u8, 152u8, 32u8, 165u8, 47u8, 86u8,
                            163u8, 255u8, 8u8, 251u8, 176u8, 138u8, 165u8, 48u8, 12u8, 27u8,
                        ],
                    )
                }

                /// See [`Pallet::refund`].
                pub fn refund(
                    &self,
                    id: types::refund::Id,
                    allow_burn: types::refund::AllowBurn,
                ) -> ::subxt::tx::Payload<types::Refund> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "refund",
                        types::Refund {
                            id,
                            allow_burn,
                        },
                        [
                            218u8, 207u8, 8u8, 41u8, 154u8, 250u8, 117u8, 174u8, 143u8, 133u8,
                            34u8, 113u8, 171u8, 18u8, 177u8, 227u8, 146u8, 92u8, 12u8, 226u8,
                            101u8, 230u8, 246u8, 162u8, 32u8, 73u8, 138u8, 158u8, 95u8, 226u8,
                            75u8, 95u8,
                        ],
                    )
                }

                /// See [`Pallet::set_min_balance`].
                pub fn set_min_balance(
                    &self,
                    id: types::set_min_balance::Id,
                    min_balance: types::set_min_balance::MinBalance,
                ) -> ::subxt::tx::Payload<types::SetMinBalance> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "set_min_balance",
                        types::SetMinBalance {
                            id,
                            min_balance,
                        },
                        [
                            141u8, 241u8, 137u8, 50u8, 232u8, 122u8, 252u8, 104u8, 185u8, 170u8,
                            246u8, 0u8, 20u8, 128u8, 136u8, 155u8, 62u8, 243u8, 4u8, 221u8, 42u8,
                            225u8, 16u8, 245u8, 58u8, 127u8, 84u8, 193u8, 175u8, 165u8, 35u8, 49u8,
                        ],
                    )
                }

                /// See [`Pallet::touch_other`].
                pub fn touch_other(
                    &self,
                    id: types::touch_other::Id,
                    who: types::touch_other::Who,
                ) -> ::subxt::tx::Payload<types::TouchOther> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "touch_other",
                        types::TouchOther {
                            id,
                            who,
                        },
                        [
                            104u8, 85u8, 80u8, 68u8, 135u8, 149u8, 102u8, 104u8, 188u8, 79u8, 42u8,
                            34u8, 241u8, 84u8, 183u8, 176u8, 215u8, 172u8, 78u8, 196u8, 206u8,
                            214u8, 138u8, 240u8, 92u8, 65u8, 117u8, 170u8, 140u8, 120u8, 50u8,
                            166u8,
                        ],
                    )
                }

                /// See [`Pallet::refund_other`].
                pub fn refund_other(
                    &self,
                    id: types::refund_other::Id,
                    who: types::refund_other::Who,
                ) -> ::subxt::tx::Payload<types::RefundOther> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "refund_other",
                        types::RefundOther {
                            id,
                            who,
                        },
                        [
                            113u8, 58u8, 33u8, 109u8, 233u8, 229u8, 210u8, 40u8, 176u8, 252u8,
                            131u8, 80u8, 33u8, 132u8, 19u8, 170u8, 145u8, 146u8, 246u8, 31u8,
                            222u8, 120u8, 167u8, 187u8, 8u8, 144u8, 164u8, 251u8, 52u8, 249u8,
                            91u8, 136u8,
                        ],
                    )
                }

                /// See [`Pallet::block`].
                pub fn block(
                    &self,
                    id: types::block::Id,
                    who: types::block::Who,
                ) -> ::subxt::tx::Payload<types::Block> {
                    ::subxt::tx::Payload::new_static(
                        "Assets",
                        "block",
                        types::Block {
                            id,
                            who,
                        },
                        [
                            224u8, 63u8, 26u8, 229u8, 23u8, 164u8, 212u8, 170u8, 156u8, 104u8,
                            63u8, 158u8, 53u8, 162u8, 157u8, 127u8, 183u8, 94u8, 211u8, 123u8,
                            228u8, 198u8, 47u8, 80u8, 53u8, 122u8, 46u8, 69u8, 67u8, 170u8, 193u8,
                            33u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_assets::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some asset class was created.
            pub struct Created {
                pub asset_id: created::AssetId,
                pub creator: created::Creator,
                pub owner: created::Owner,
            }
            pub mod created {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Creator = ::subxt::utils::AccountId32;
                pub type Owner = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Created {
                const EVENT: &'static str = "Created";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some assets were issued.
            pub struct Issued {
                pub asset_id: issued::AssetId,
                pub owner: issued::Owner,
                pub amount: issued::Amount,
            }
            pub mod issued {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Issued {
                const EVENT: &'static str = "Issued";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some assets were transferred.
            pub struct Transferred {
                pub asset_id: transferred::AssetId,
                pub from: transferred::From,
                pub to: transferred::To,
                pub amount: transferred::Amount,
            }
            pub mod transferred {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type From = ::subxt::utils::AccountId32;
                pub type To = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Transferred {
                const EVENT: &'static str = "Transferred";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some assets were destroyed.
            pub struct Burned {
                pub asset_id: burned::AssetId,
                pub owner: burned::Owner,
                pub balance: burned::Balance,
            }
            pub mod burned {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::utils::AccountId32;
                pub type Balance = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for Burned {
                const EVENT: &'static str = "Burned";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// The management team changed.
            pub struct TeamChanged {
                pub asset_id: team_changed::AssetId,
                pub issuer: team_changed::Issuer,
                pub admin: team_changed::Admin,
                pub freezer: team_changed::Freezer,
            }
            pub mod team_changed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Issuer = ::subxt::utils::AccountId32;
                pub type Admin = ::subxt::utils::AccountId32;
                pub type Freezer = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for TeamChanged {
                const EVENT: &'static str = "TeamChanged";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// The owner changed.
            pub struct OwnerChanged {
                pub asset_id: owner_changed::AssetId,
                pub owner: owner_changed::Owner,
            }
            pub mod owner_changed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for OwnerChanged {
                const EVENT: &'static str = "OwnerChanged";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some account `who` was frozen.
            pub struct Frozen {
                pub asset_id: frozen::AssetId,
                pub who: frozen::Who,
            }
            pub mod frozen {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Frozen {
                const EVENT: &'static str = "Frozen";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some account `who` was thawed.
            pub struct Thawed {
                pub asset_id: thawed::AssetId,
                pub who: thawed::Who,
            }
            pub mod thawed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Thawed {
                const EVENT: &'static str = "Thawed";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some asset `asset_id` was frozen.
            pub struct AssetFrozen {
                pub asset_id: asset_frozen::AssetId,
            }
            pub mod asset_frozen {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for AssetFrozen {
                const EVENT: &'static str = "AssetFrozen";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some asset `asset_id` was thawed.
            pub struct AssetThawed {
                pub asset_id: asset_thawed::AssetId,
            }
            pub mod asset_thawed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for AssetThawed {
                const EVENT: &'static str = "AssetThawed";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Accounts were destroyed for given asset.
            pub struct AccountsDestroyed {
                pub asset_id: accounts_destroyed::AssetId,
                pub accounts_destroyed: accounts_destroyed::AccountsDestroyed,
                pub accounts_remaining: accounts_destroyed::AccountsRemaining,
            }
            pub mod accounts_destroyed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type AccountsDestroyed = ::core::primitive::u32;
                pub type AccountsRemaining = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for AccountsDestroyed {
                const EVENT: &'static str = "AccountsDestroyed";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Approvals were destroyed for given asset.
            pub struct ApprovalsDestroyed {
                pub asset_id: approvals_destroyed::AssetId,
                pub approvals_destroyed: approvals_destroyed::ApprovalsDestroyed,
                pub approvals_remaining: approvals_destroyed::ApprovalsRemaining,
            }
            pub mod approvals_destroyed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type ApprovalsDestroyed = ::core::primitive::u32;
                pub type ApprovalsRemaining = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for ApprovalsDestroyed {
                const EVENT: &'static str = "ApprovalsDestroyed";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An asset class is in the process of being destroyed.
            pub struct DestructionStarted {
                pub asset_id: destruction_started::AssetId,
            }
            pub mod destruction_started {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for DestructionStarted {
                const EVENT: &'static str = "DestructionStarted";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An asset class was destroyed.
            pub struct Destroyed {
                pub asset_id: destroyed::AssetId,
            }
            pub mod destroyed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for Destroyed {
                const EVENT: &'static str = "Destroyed";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some asset class was force-created.
            pub struct ForceCreated {
                pub asset_id: force_created::AssetId,
                pub owner: force_created::Owner,
            }
            pub mod force_created {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for ForceCreated {
                const EVENT: &'static str = "ForceCreated";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// New metadata has been set for an asset.
            pub struct MetadataSet {
                pub asset_id: metadata_set::AssetId,
                pub name: metadata_set::Name,
                pub symbol: metadata_set::Symbol,
                pub decimals: metadata_set::Decimals,
                pub is_frozen: metadata_set::IsFrozen,
            }
            pub mod metadata_set {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Name = ::std::vec::Vec<::core::primitive::u8>;
                pub type Symbol = ::std::vec::Vec<::core::primitive::u8>;
                pub type Decimals = ::core::primitive::u8;
                pub type IsFrozen = ::core::primitive::bool;
            }
            impl ::subxt::events::StaticEvent for MetadataSet {
                const EVENT: &'static str = "MetadataSet";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Metadata has been cleared for an asset.
            pub struct MetadataCleared {
                pub asset_id: metadata_cleared::AssetId,
            }
            pub mod metadata_cleared {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for MetadataCleared {
                const EVENT: &'static str = "MetadataCleared";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// (Additional) funds have been approved for transfer to a
            /// destination account.
            pub struct ApprovedTransfer {
                pub asset_id: approved_transfer::AssetId,
                pub source: approved_transfer::Source,
                pub delegate: approved_transfer::Delegate,
                pub amount: approved_transfer::Amount,
            }
            pub mod approved_transfer {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Source = ::subxt::utils::AccountId32;
                pub type Delegate = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for ApprovedTransfer {
                const EVENT: &'static str = "ApprovedTransfer";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An approval for account `delegate` was cancelled by `owner`.
            pub struct ApprovalCancelled {
                pub asset_id: approval_cancelled::AssetId,
                pub owner: approval_cancelled::Owner,
                pub delegate: approval_cancelled::Delegate,
            }
            pub mod approval_cancelled {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::utils::AccountId32;
                pub type Delegate = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for ApprovalCancelled {
                const EVENT: &'static str = "ApprovalCancelled";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An `amount` was transferred in its entirety from `owner` to
            /// `destination` by
            /// the approved `delegate`.
            pub struct TransferredApproved {
                pub asset_id: transferred_approved::AssetId,
                pub owner: transferred_approved::Owner,
                pub delegate: transferred_approved::Delegate,
                pub destination: transferred_approved::Destination,
                pub amount: transferred_approved::Amount,
            }
            pub mod transferred_approved {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Owner = ::subxt::utils::AccountId32;
                pub type Delegate = ::subxt::utils::AccountId32;
                pub type Destination = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for TransferredApproved {
                const EVENT: &'static str = "TransferredApproved";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// An asset has had its attributes changed by the `Force` origin.
            pub struct AssetStatusChanged {
                pub asset_id: asset_status_changed::AssetId,
            }
            pub mod asset_status_changed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for AssetStatusChanged {
                const EVENT: &'static str = "AssetStatusChanged";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// The min_balance of an asset has been updated by the asset owner.
            pub struct AssetMinBalanceChanged {
                pub asset_id: asset_min_balance_changed::AssetId,
                pub new_min_balance: asset_min_balance_changed::NewMinBalance,
            }
            pub mod asset_min_balance_changed {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type NewMinBalance = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for AssetMinBalanceChanged {
                const EVENT: &'static str = "AssetMinBalanceChanged";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some account `who` was created with a deposit from `depositor`.
            pub struct Touched {
                pub asset_id: touched::AssetId,
                pub who: touched::Who,
                pub depositor: touched::Depositor,
            }
            pub mod touched {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Depositor = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Touched {
                const EVENT: &'static str = "Touched";
                const PALLET: &'static str = "Assets";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Some account `who` was blocked.
            pub struct Blocked {
                pub asset_id: blocked::AssetId,
                pub who: blocked::Who,
            }
            pub mod blocked {
                use super::runtime_types;
                pub type AssetId = ::core::primitive::u32;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for Blocked {
                const EVENT: &'static str = "Blocked";
                const PALLET: &'static str = "Assets";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod asset {
                    use super::runtime_types;
                    pub type Asset = runtime_types::pallet_assets::types::AssetDetails<
                        ::core::primitive::u128,
                        ::subxt::utils::AccountId32,
                        ::core::primitive::u128,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                }
                pub mod account {
                    use super::runtime_types;
                    pub type Account = runtime_types::pallet_assets::types::AssetAccount<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                        (),
                        ::subxt::utils::AccountId32,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                    pub type Param1 = ::subxt::utils::AccountId32;
                }
                pub mod approvals {
                    use super::runtime_types;
                    pub type Approvals = runtime_types::pallet_assets::types::Approval<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                    pub type Param1 = ::subxt::utils::AccountId32;
                    pub type Param2 = ::subxt::utils::AccountId32;
                }
                pub mod metadata {
                    use super::runtime_types;
                    pub type Metadata = runtime_types::pallet_assets::types::AssetMetadata<
                        ::core::primitive::u128,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    >;
                    pub type Param0 = ::core::primitive::u32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// Details of an asset.
                pub fn asset_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::asset::Asset,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Asset",
                        vec![],
                        [
                            159u8, 234u8, 177u8, 31u8, 58u8, 51u8, 173u8, 184u8, 250u8, 169u8,
                            246u8, 122u8, 54u8, 19u8, 232u8, 60u8, 0u8, 165u8, 12u8, 101u8, 93u8,
                            169u8, 23u8, 34u8, 154u8, 44u8, 134u8, 128u8, 97u8, 71u8, 167u8, 224u8,
                        ],
                    )
                }

                /// Details of an asset.
                pub fn asset(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::asset::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::asset::Asset,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Asset",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            159u8, 234u8, 177u8, 31u8, 58u8, 51u8, 173u8, 184u8, 250u8, 169u8,
                            246u8, 122u8, 54u8, 19u8, 232u8, 60u8, 0u8, 165u8, 12u8, 101u8, 93u8,
                            169u8, 23u8, 34u8, 154u8, 44u8, 134u8, 128u8, 97u8, 71u8, 167u8, 224u8,
                        ],
                    )
                }

                /// The holdings of a specific account for a specific asset.
                pub fn account_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Account",
                        vec![],
                        [
                            188u8, 242u8, 133u8, 64u8, 0u8, 11u8, 57u8, 146u8, 60u8, 137u8, 35u8,
                            23u8, 183u8, 200u8, 242u8, 8u8, 94u8, 158u8, 218u8, 13u8, 104u8, 215u8,
                            87u8, 86u8, 69u8, 200u8, 11u8, 51u8, 6u8, 65u8, 216u8, 102u8,
                        ],
                    )
                }

                /// The holdings of a specific account for a specific asset.
                pub fn account_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::account::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Account",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            188u8, 242u8, 133u8, 64u8, 0u8, 11u8, 57u8, 146u8, 60u8, 137u8, 35u8,
                            23u8, 183u8, 200u8, 242u8, 8u8, 94u8, 158u8, 218u8, 13u8, 104u8, 215u8,
                            87u8, 86u8, 69u8, 200u8, 11u8, 51u8, 6u8, 65u8, 216u8, 102u8,
                        ],
                    )
                }

                /// The holdings of a specific account for a specific asset.
                pub fn account(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::account::Param0>,
                    _1: impl ::std::borrow::Borrow<types::account::Param1>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::account::Account,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Account",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            188u8, 242u8, 133u8, 64u8, 0u8, 11u8, 57u8, 146u8, 60u8, 137u8, 35u8,
                            23u8, 183u8, 200u8, 242u8, 8u8, 94u8, 158u8, 218u8, 13u8, 104u8, 215u8,
                            87u8, 86u8, 69u8, 200u8, 11u8, 51u8, 6u8, 65u8, 216u8, 102u8,
                        ],
                    )
                }

                /// Approved balance transfers. First balance is the amount
                /// approved for transfer. Second
                /// is the amount of `T::Currency` reserved for storing this.
                /// First key is the asset ID, second key is the owner and third
                /// key is the delegate.
                pub fn approvals_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::approvals::Approvals,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Approvals",
                        vec![],
                        [
                            122u8, 92u8, 51u8, 45u8, 200u8, 200u8, 182u8, 208u8, 18u8, 47u8, 139u8,
                            68u8, 254u8, 15u8, 152u8, 110u8, 3u8, 138u8, 13u8, 183u8, 5u8, 185u8,
                            218u8, 44u8, 93u8, 28u8, 56u8, 189u8, 125u8, 127u8, 123u8, 8u8,
                        ],
                    )
                }

                /// Approved balance transfers. First balance is the amount
                /// approved for transfer. Second
                /// is the amount of `T::Currency` reserved for storing this.
                /// First key is the asset ID, second key is the owner and third
                /// key is the delegate.
                pub fn approvals_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::approvals::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::approvals::Approvals,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Approvals",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            122u8, 92u8, 51u8, 45u8, 200u8, 200u8, 182u8, 208u8, 18u8, 47u8, 139u8,
                            68u8, 254u8, 15u8, 152u8, 110u8, 3u8, 138u8, 13u8, 183u8, 5u8, 185u8,
                            218u8, 44u8, 93u8, 28u8, 56u8, 189u8, 125u8, 127u8, 123u8, 8u8,
                        ],
                    )
                }

                /// Approved balance transfers. First balance is the amount
                /// approved for transfer. Second
                /// is the amount of `T::Currency` reserved for storing this.
                /// First key is the asset ID, second key is the owner and third
                /// key is the delegate.
                pub fn approvals_iter2(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::approvals::Param0>,
                    _1: impl ::std::borrow::Borrow<types::approvals::Param1>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::approvals::Approvals,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Approvals",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            122u8, 92u8, 51u8, 45u8, 200u8, 200u8, 182u8, 208u8, 18u8, 47u8, 139u8,
                            68u8, 254u8, 15u8, 152u8, 110u8, 3u8, 138u8, 13u8, 183u8, 5u8, 185u8,
                            218u8, 44u8, 93u8, 28u8, 56u8, 189u8, 125u8, 127u8, 123u8, 8u8,
                        ],
                    )
                }

                /// Approved balance transfers. First balance is the amount
                /// approved for transfer. Second
                /// is the amount of `T::Currency` reserved for storing this.
                /// First key is the asset ID, second key is the owner and third
                /// key is the delegate.
                pub fn approvals(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::approvals::Param0>,
                    _1: impl ::std::borrow::Borrow<types::approvals::Param1>,
                    _2: impl ::std::borrow::Borrow<types::approvals::Param2>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::approvals::Approvals,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Approvals",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_2.borrow()),
                        ],
                        [
                            122u8, 92u8, 51u8, 45u8, 200u8, 200u8, 182u8, 208u8, 18u8, 47u8, 139u8,
                            68u8, 254u8, 15u8, 152u8, 110u8, 3u8, 138u8, 13u8, 183u8, 5u8, 185u8,
                            218u8, 44u8, 93u8, 28u8, 56u8, 189u8, 125u8, 127u8, 123u8, 8u8,
                        ],
                    )
                }

                /// Metadata of an asset.
                pub fn metadata_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::metadata::Metadata,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Metadata",
                        vec![],
                        [
                            129u8, 202u8, 244u8, 77u8, 55u8, 81u8, 86u8, 106u8, 20u8, 153u8, 209u8,
                            69u8, 199u8, 107u8, 111u8, 49u8, 88u8, 157u8, 84u8, 41u8, 198u8, 190u8,
                            234u8, 218u8, 68u8, 207u8, 87u8, 217u8, 73u8, 66u8, 211u8, 163u8,
                        ],
                    )
                }

                /// Metadata of an asset.
                pub fn metadata(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::metadata::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::metadata::Metadata,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Assets",
                        "Metadata",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            129u8, 202u8, 244u8, 77u8, 55u8, 81u8, 86u8, 106u8, 20u8, 153u8, 209u8,
                            69u8, 199u8, 107u8, 111u8, 49u8, 88u8, 157u8, 84u8, 41u8, 198u8, 190u8,
                            234u8, 218u8, 68u8, 207u8, 87u8, 217u8, 73u8, 66u8, 211u8, 163u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// Max number of items to destroy per `destroy_accounts` and
                /// `destroy_approvals` call.
                ///
                /// Must be configured to result in a weight that makes each
                /// call fit in a block.
                pub fn remove_items_limit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Assets",
                        "RemoveItemsLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// The basic amount of funds that must be reserved for an
                /// asset.
                pub fn asset_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Assets",
                        "AssetDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The amount of funds that must be reserved for a non-provider
                /// asset account to be
                /// maintained.
                pub fn asset_account_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Assets",
                        "AssetAccountDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The basic amount of funds that must be reserved when adding
                /// metadata to your asset.
                pub fn metadata_deposit_base(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Assets",
                        "MetadataDepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The additional funds that must be reserved for the number of
                /// bytes you store in your
                /// metadata.
                pub fn metadata_deposit_per_byte(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Assets",
                        "MetadataDepositPerByte",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The amount of funds that must be reserved when creating a
                /// new approval.
                pub fn approval_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Assets",
                        "ApprovalDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The maximum length of a name or symbol stored on-chain.
                pub fn string_limit(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Assets",
                        "StringLimit",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod multisig {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::pallet_multisig::pallet::Error;
        /// Contains a variant per dispatchable extrinsic that this pallet has.
        pub type Call = runtime_types::pallet_multisig::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::as_multi_threshold_1`].
                pub struct AsMultiThreshold1 {
                    pub other_signatories: as_multi_threshold1::OtherSignatories,
                    pub call: ::std::boxed::Box<as_multi_threshold1::Call>,
                }
                pub mod as_multi_threshold1 {
                    use super::runtime_types;
                    pub type OtherSignatories = ::std::vec::Vec<::subxt::utils::AccountId32>;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                }
                impl ::subxt::blocks::StaticExtrinsic for AsMultiThreshold1 {
                    const CALL: &'static str = "as_multi_threshold_1";
                    const PALLET: &'static str = "Multisig";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::as_multi`].
                pub struct AsMulti {
                    pub threshold: as_multi::Threshold,
                    pub other_signatories: as_multi::OtherSignatories,
                    pub maybe_timepoint: as_multi::MaybeTimepoint,
                    pub call: ::std::boxed::Box<as_multi::Call>,
                    pub max_weight: as_multi::MaxWeight,
                }
                pub mod as_multi {
                    use super::runtime_types;
                    pub type Threshold = ::core::primitive::u16;
                    pub type OtherSignatories = ::std::vec::Vec<::subxt::utils::AccountId32>;
                    pub type MaybeTimepoint = ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >;
                    pub type Call = runtime_types::nagara_core_runtime::RuntimeCall;
                    pub type MaxWeight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::blocks::StaticExtrinsic for AsMulti {
                    const CALL: &'static str = "as_multi";
                    const PALLET: &'static str = "Multisig";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::approve_as_multi`].
                pub struct ApproveAsMulti {
                    pub threshold: approve_as_multi::Threshold,
                    pub other_signatories: approve_as_multi::OtherSignatories,
                    pub maybe_timepoint: approve_as_multi::MaybeTimepoint,
                    pub call_hash: approve_as_multi::CallHash,
                    pub max_weight: approve_as_multi::MaxWeight,
                }
                pub mod approve_as_multi {
                    use super::runtime_types;
                    pub type Threshold = ::core::primitive::u16;
                    pub type OtherSignatories = ::std::vec::Vec<::subxt::utils::AccountId32>;
                    pub type MaybeTimepoint = ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >;
                    pub type CallHash = [::core::primitive::u8; 32usize];
                    pub type MaxWeight = runtime_types::sp_weights::weight_v2::Weight;
                }
                impl ::subxt::blocks::StaticExtrinsic for ApproveAsMulti {
                    const CALL: &'static str = "approve_as_multi";
                    const PALLET: &'static str = "Multisig";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::cancel_as_multi`].
                pub struct CancelAsMulti {
                    pub threshold: cancel_as_multi::Threshold,
                    pub other_signatories: cancel_as_multi::OtherSignatories,
                    pub timepoint: cancel_as_multi::Timepoint,
                    pub call_hash: cancel_as_multi::CallHash,
                }
                pub mod cancel_as_multi {
                    use super::runtime_types;
                    pub type Threshold = ::core::primitive::u16;
                    pub type OtherSignatories = ::std::vec::Vec<::subxt::utils::AccountId32>;
                    pub type Timepoint =
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>;
                    pub type CallHash = [::core::primitive::u8; 32usize];
                }
                impl ::subxt::blocks::StaticExtrinsic for CancelAsMulti {
                    const CALL: &'static str = "cancel_as_multi";
                    const PALLET: &'static str = "Multisig";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::as_multi_threshold_1`].
                pub fn as_multi_threshold_1(
                    &self,
                    other_signatories: types::as_multi_threshold1::OtherSignatories,
                    call: types::as_multi_threshold1::Call,
                ) -> ::subxt::tx::Payload<types::AsMultiThreshold1> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "as_multi_threshold_1",
                        types::AsMultiThreshold1 {
                            other_signatories,
                            call: ::std::boxed::Box::new(call),
                        },
                        [
                            132u8, 18u8, 132u8, 118u8, 67u8, 198u8, 51u8, 201u8, 50u8, 128u8,
                            113u8, 190u8, 143u8, 18u8, 9u8, 65u8, 113u8, 140u8, 32u8, 58u8, 64u8,
                            80u8, 156u8, 59u8, 24u8, 32u8, 74u8, 86u8, 173u8, 51u8, 62u8, 179u8,
                        ],
                    )
                }

                /// See [`Pallet::as_multi`].
                pub fn as_multi(
                    &self,
                    threshold: types::as_multi::Threshold,
                    other_signatories: types::as_multi::OtherSignatories,
                    maybe_timepoint: types::as_multi::MaybeTimepoint,
                    call: types::as_multi::Call,
                    max_weight: types::as_multi::MaxWeight,
                ) -> ::subxt::tx::Payload<types::AsMulti> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "as_multi",
                        types::AsMulti {
                            threshold,
                            other_signatories,
                            maybe_timepoint,
                            call: ::std::boxed::Box::new(call),
                            max_weight,
                        },
                        [
                            254u8, 230u8, 148u8, 232u8, 198u8, 73u8, 49u8, 81u8, 21u8, 144u8,
                            216u8, 63u8, 71u8, 44u8, 22u8, 208u8, 204u8, 189u8, 104u8, 198u8,
                            237u8, 224u8, 188u8, 91u8, 61u8, 47u8, 234u8, 87u8, 0u8, 169u8, 166u8,
                            220u8,
                        ],
                    )
                }

                /// See [`Pallet::approve_as_multi`].
                pub fn approve_as_multi(
                    &self,
                    threshold: types::approve_as_multi::Threshold,
                    other_signatories: types::approve_as_multi::OtherSignatories,
                    maybe_timepoint: types::approve_as_multi::MaybeTimepoint,
                    call_hash: types::approve_as_multi::CallHash,
                    max_weight: types::approve_as_multi::MaxWeight,
                ) -> ::subxt::tx::Payload<types::ApproveAsMulti> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "approve_as_multi",
                        types::ApproveAsMulti {
                            threshold,
                            other_signatories,
                            maybe_timepoint,
                            call_hash,
                            max_weight,
                        },
                        [
                            248u8, 46u8, 131u8, 35u8, 204u8, 12u8, 218u8, 150u8, 88u8, 131u8, 89u8,
                            13u8, 95u8, 122u8, 87u8, 107u8, 136u8, 154u8, 92u8, 199u8, 108u8, 92u8,
                            207u8, 171u8, 113u8, 8u8, 47u8, 248u8, 65u8, 26u8, 203u8, 135u8,
                        ],
                    )
                }

                /// See [`Pallet::cancel_as_multi`].
                pub fn cancel_as_multi(
                    &self,
                    threshold: types::cancel_as_multi::Threshold,
                    other_signatories: types::cancel_as_multi::OtherSignatories,
                    timepoint: types::cancel_as_multi::Timepoint,
                    call_hash: types::cancel_as_multi::CallHash,
                ) -> ::subxt::tx::Payload<types::CancelAsMulti> {
                    ::subxt::tx::Payload::new_static(
                        "Multisig",
                        "cancel_as_multi",
                        types::CancelAsMulti {
                            threshold,
                            other_signatories,
                            timepoint,
                            call_hash,
                        },
                        [
                            212u8, 179u8, 123u8, 40u8, 209u8, 228u8, 181u8, 0u8, 109u8, 28u8, 27u8,
                            48u8, 15u8, 47u8, 203u8, 54u8, 106u8, 114u8, 28u8, 118u8, 101u8, 201u8,
                            95u8, 187u8, 46u8, 182u8, 4u8, 30u8, 227u8, 105u8, 14u8, 81u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A new multisig operation has begun.
            pub struct NewMultisig {
                pub approving: new_multisig::Approving,
                pub multisig: new_multisig::Multisig,
                pub call_hash: new_multisig::CallHash,
            }
            pub mod new_multisig {
                use super::runtime_types;
                pub type Approving = ::subxt::utils::AccountId32;
                pub type Multisig = ::subxt::utils::AccountId32;
                pub type CallHash = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for NewMultisig {
                const EVENT: &'static str = "NewMultisig";
                const PALLET: &'static str = "Multisig";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A multisig operation has been approved by someone.
            pub struct MultisigApproval {
                pub approving: multisig_approval::Approving,
                pub timepoint: multisig_approval::Timepoint,
                pub multisig: multisig_approval::Multisig,
                pub call_hash: multisig_approval::CallHash,
            }
            pub mod multisig_approval {
                use super::runtime_types;
                pub type Approving = ::subxt::utils::AccountId32;
                pub type Timepoint =
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>;
                pub type Multisig = ::subxt::utils::AccountId32;
                pub type CallHash = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for MultisigApproval {
                const EVENT: &'static str = "MultisigApproval";
                const PALLET: &'static str = "Multisig";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A multisig operation has been executed.
            pub struct MultisigExecuted {
                pub approving: multisig_executed::Approving,
                pub timepoint: multisig_executed::Timepoint,
                pub multisig: multisig_executed::Multisig,
                pub call_hash: multisig_executed::CallHash,
                pub result: multisig_executed::Result,
            }
            pub mod multisig_executed {
                use super::runtime_types;
                pub type Approving = ::subxt::utils::AccountId32;
                pub type Timepoint =
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>;
                pub type Multisig = ::subxt::utils::AccountId32;
                pub type CallHash = [::core::primitive::u8; 32usize];
                pub type Result =
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>;
            }
            impl ::subxt::events::StaticEvent for MultisigExecuted {
                const EVENT: &'static str = "MultisigExecuted";
                const PALLET: &'static str = "Multisig";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A multisig operation has been cancelled.
            pub struct MultisigCancelled {
                pub cancelling: multisig_cancelled::Cancelling,
                pub timepoint: multisig_cancelled::Timepoint,
                pub multisig: multisig_cancelled::Multisig,
                pub call_hash: multisig_cancelled::CallHash,
            }
            pub mod multisig_cancelled {
                use super::runtime_types;
                pub type Cancelling = ::subxt::utils::AccountId32;
                pub type Timepoint =
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>;
                pub type Multisig = ::subxt::utils::AccountId32;
                pub type CallHash = [::core::primitive::u8; 32usize];
            }
            impl ::subxt::events::StaticEvent for MultisigCancelled {
                const EVENT: &'static str = "MultisigCancelled";
                const PALLET: &'static str = "Multisig";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod multisigs {
                    use super::runtime_types;
                    pub type Multisigs = runtime_types::pallet_multisig::Multisig<
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::utils::AccountId32,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                    pub type Param1 = [::core::primitive::u8; 32usize];
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// The set of open multisig operations.
                pub fn multisigs_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::multisigs::Multisigs,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Multisig",
                        "Multisigs",
                        vec![],
                        [
                            154u8, 109u8, 45u8, 18u8, 155u8, 151u8, 81u8, 28u8, 86u8, 127u8, 189u8,
                            151u8, 49u8, 61u8, 12u8, 149u8, 84u8, 61u8, 110u8, 197u8, 200u8, 140u8,
                            37u8, 100u8, 14u8, 162u8, 158u8, 161u8, 48u8, 117u8, 102u8, 61u8,
                        ],
                    )
                }

                /// The set of open multisig operations.
                pub fn multisigs_iter1(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::multisigs::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::multisigs::Multisigs,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Multisig",
                        "Multisigs",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            154u8, 109u8, 45u8, 18u8, 155u8, 151u8, 81u8, 28u8, 86u8, 127u8, 189u8,
                            151u8, 49u8, 61u8, 12u8, 149u8, 84u8, 61u8, 110u8, 197u8, 200u8, 140u8,
                            37u8, 100u8, 14u8, 162u8, 158u8, 161u8, 48u8, 117u8, 102u8, 61u8,
                        ],
                    )
                }

                /// The set of open multisig operations.
                pub fn multisigs(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::multisigs::Param0>,
                    _1: impl ::std::borrow::Borrow<types::multisigs::Param1>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::multisigs::Multisigs,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Multisig",
                        "Multisigs",
                        vec![
                            ::subxt::storage::address::make_static_storage_map_key(_0.borrow()),
                            ::subxt::storage::address::make_static_storage_map_key(_1.borrow()),
                        ],
                        [
                            154u8, 109u8, 45u8, 18u8, 155u8, 151u8, 81u8, 28u8, 86u8, 127u8, 189u8,
                            151u8, 49u8, 61u8, 12u8, 149u8, 84u8, 61u8, 110u8, 197u8, 200u8, 140u8,
                            37u8, 100u8, 14u8, 162u8, 158u8, 161u8, 48u8, 117u8, 102u8, 61u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// The base amount of currency needed to reserve for creating a
                /// multisig execution or to
                /// store a dispatch call for later.
                ///
                /// This is held for an additional storage item whose value size
                /// is
                /// `4 + sizeof((BlockNumber, Balance, AccountId))` bytes and
                /// whose key size is
                /// `32 + sizeof(AccountId)` bytes.
                pub fn deposit_base(&self) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Multisig",
                        "DepositBase",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The amount of currency needed per unit threshold when
                /// creating a multisig execution.
                ///
                /// This is held for adding 32 bytes more into a pre-existing
                /// storage value.
                pub fn deposit_factor(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Multisig",
                        "DepositFactor",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The maximum amount of signatories allowed in the multisig.
                pub fn max_signatories(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Multisig",
                        "MaxSignatories",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod identity {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::pallet_identity::pallet::Error;
        /// Identity pallet declaration.
        pub type Call = runtime_types::pallet_identity::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::add_registrar`].
                pub struct AddRegistrar {
                    pub account: add_registrar::Account,
                }
                pub mod add_registrar {
                    use super::runtime_types;
                    pub type Account =
                        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for AddRegistrar {
                    const CALL: &'static str = "add_registrar";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_identity`].
                pub struct SetIdentity {
                    pub info: ::std::boxed::Box<set_identity::Info>,
                }
                pub mod set_identity {
                    use super::runtime_types;
                    pub type Info = runtime_types::pallet_identity::types::IdentityInfo;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetIdentity {
                    const CALL: &'static str = "set_identity";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_subs`].
                pub struct SetSubs {
                    pub subs: set_subs::Subs,
                }
                pub mod set_subs {
                    use super::runtime_types;
                    pub type Subs = ::std::vec::Vec<(
                        ::subxt::utils::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetSubs {
                    const CALL: &'static str = "set_subs";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::clear_identity`].
                pub struct ClearIdentity;
                impl ::subxt::blocks::StaticExtrinsic for ClearIdentity {
                    const CALL: &'static str = "clear_identity";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::request_judgement`].
                pub struct RequestJudgement {
                    #[codec(compact)]
                    pub reg_index: request_judgement::RegIndex,
                    #[codec(compact)]
                    pub max_fee: request_judgement::MaxFee,
                }
                pub mod request_judgement {
                    use super::runtime_types;
                    pub type RegIndex = ::core::primitive::u32;
                    pub type MaxFee = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for RequestJudgement {
                    const CALL: &'static str = "request_judgement";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::cancel_request`].
                pub struct CancelRequest {
                    pub reg_index: cancel_request::RegIndex,
                }
                pub mod cancel_request {
                    use super::runtime_types;
                    pub type RegIndex = ::core::primitive::u32;
                }
                impl ::subxt::blocks::StaticExtrinsic for CancelRequest {
                    const CALL: &'static str = "cancel_request";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_fee`].
                pub struct SetFee {
                    #[codec(compact)]
                    pub index: set_fee::Index,
                    #[codec(compact)]
                    pub fee: set_fee::Fee,
                }
                pub mod set_fee {
                    use super::runtime_types;
                    pub type Index = ::core::primitive::u32;
                    pub type Fee = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetFee {
                    const CALL: &'static str = "set_fee";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_account_id`].
                pub struct SetAccountId {
                    #[codec(compact)]
                    pub index: set_account_id::Index,
                    pub new: set_account_id::New,
                }
                pub mod set_account_id {
                    use super::runtime_types;
                    pub type Index = ::core::primitive::u32;
                    pub type New = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetAccountId {
                    const CALL: &'static str = "set_account_id";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::set_fields`].
                pub struct SetFields {
                    #[codec(compact)]
                    pub index: set_fields::Index,
                    pub fields: set_fields::Fields,
                }
                pub mod set_fields {
                    use super::runtime_types;
                    pub type Index = ::core::primitive::u32;
                    pub type Fields = runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >;
                }
                impl ::subxt::blocks::StaticExtrinsic for SetFields {
                    const CALL: &'static str = "set_fields";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::provide_judgement`].
                pub struct ProvideJudgement {
                    #[codec(compact)]
                    pub reg_index: provide_judgement::RegIndex,
                    pub target: provide_judgement::Target,
                    pub judgement: provide_judgement::Judgement,
                    pub identity: provide_judgement::Identity,
                }
                pub mod provide_judgement {
                    use super::runtime_types;
                    pub type RegIndex = ::core::primitive::u32;
                    pub type Target = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Judgement =
                        runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>;
                    pub type Identity = ::subxt::utils::H256;
                }
                impl ::subxt::blocks::StaticExtrinsic for ProvideJudgement {
                    const CALL: &'static str = "provide_judgement";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::kill_identity`].
                pub struct KillIdentity {
                    pub target: kill_identity::Target,
                }
                pub mod kill_identity {
                    use super::runtime_types;
                    pub type Target = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for KillIdentity {
                    const CALL: &'static str = "kill_identity";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::add_sub`].
                pub struct AddSub {
                    pub sub: add_sub::Sub,
                    pub data: add_sub::Data,
                }
                pub mod add_sub {
                    use super::runtime_types;
                    pub type Sub = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Data = runtime_types::pallet_identity::types::Data;
                }
                impl ::subxt::blocks::StaticExtrinsic for AddSub {
                    const CALL: &'static str = "add_sub";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::rename_sub`].
                pub struct RenameSub {
                    pub sub: rename_sub::Sub,
                    pub data: rename_sub::Data,
                }
                pub mod rename_sub {
                    use super::runtime_types;
                    pub type Sub = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                    pub type Data = runtime_types::pallet_identity::types::Data;
                }
                impl ::subxt::blocks::StaticExtrinsic for RenameSub {
                    const CALL: &'static str = "rename_sub";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::remove_sub`].
                pub struct RemoveSub {
                    pub sub: remove_sub::Sub,
                }
                pub mod remove_sub {
                    use super::runtime_types;
                    pub type Sub = ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>;
                }
                impl ::subxt::blocks::StaticExtrinsic for RemoveSub {
                    const CALL: &'static str = "remove_sub";
                    const PALLET: &'static str = "Identity";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::quit_sub`].
                pub struct QuitSub;
                impl ::subxt::blocks::StaticExtrinsic for QuitSub {
                    const CALL: &'static str = "quit_sub";
                    const PALLET: &'static str = "Identity";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::add_registrar`].
                pub fn add_registrar(
                    &self,
                    account: types::add_registrar::Account,
                ) -> ::subxt::tx::Payload<types::AddRegistrar> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "add_registrar",
                        types::AddRegistrar {
                            account,
                        },
                        [
                            6u8, 131u8, 82u8, 191u8, 37u8, 240u8, 158u8, 187u8, 247u8, 98u8, 175u8,
                            200u8, 147u8, 78u8, 88u8, 176u8, 227u8, 179u8, 184u8, 194u8, 91u8, 1u8,
                            1u8, 20u8, 121u8, 4u8, 96u8, 94u8, 103u8, 140u8, 247u8, 253u8,
                        ],
                    )
                }

                /// See [`Pallet::set_identity`].
                pub fn set_identity(
                    &self,
                    info: types::set_identity::Info,
                ) -> ::subxt::tx::Payload<types::SetIdentity> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_identity",
                        types::SetIdentity {
                            info: ::std::boxed::Box::new(info),
                        },
                        [
                            18u8, 86u8, 67u8, 10u8, 116u8, 254u8, 94u8, 95u8, 166u8, 30u8, 204u8,
                            189u8, 174u8, 70u8, 191u8, 255u8, 149u8, 93u8, 156u8, 120u8, 105u8,
                            138u8, 199u8, 181u8, 43u8, 150u8, 143u8, 254u8, 182u8, 81u8, 86u8,
                            45u8,
                        ],
                    )
                }

                /// See [`Pallet::set_subs`].
                pub fn set_subs(
                    &self,
                    subs: types::set_subs::Subs,
                ) -> ::subxt::tx::Payload<types::SetSubs> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_subs",
                        types::SetSubs {
                            subs,
                        },
                        [
                            34u8, 184u8, 18u8, 155u8, 112u8, 247u8, 235u8, 75u8, 209u8, 236u8,
                            21u8, 238u8, 43u8, 237u8, 223u8, 147u8, 48u8, 6u8, 39u8, 231u8, 174u8,
                            164u8, 243u8, 184u8, 220u8, 151u8, 165u8, 69u8, 219u8, 122u8, 234u8,
                            100u8,
                        ],
                    )
                }

                /// See [`Pallet::clear_identity`].
                pub fn clear_identity(&self) -> ::subxt::tx::Payload<types::ClearIdentity> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "clear_identity",
                        types::ClearIdentity {},
                        [
                            43u8, 115u8, 205u8, 44u8, 24u8, 130u8, 220u8, 69u8, 247u8, 176u8,
                            200u8, 175u8, 67u8, 183u8, 36u8, 200u8, 162u8, 132u8, 242u8, 25u8,
                            21u8, 106u8, 197u8, 219u8, 141u8, 51u8, 204u8, 13u8, 191u8, 201u8,
                            31u8, 31u8,
                        ],
                    )
                }

                /// See [`Pallet::request_judgement`].
                pub fn request_judgement(
                    &self,
                    reg_index: types::request_judgement::RegIndex,
                    max_fee: types::request_judgement::MaxFee,
                ) -> ::subxt::tx::Payload<types::RequestJudgement> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "request_judgement",
                        types::RequestJudgement {
                            reg_index,
                            max_fee,
                        },
                        [
                            83u8, 85u8, 55u8, 184u8, 14u8, 54u8, 49u8, 212u8, 26u8, 148u8, 33u8,
                            147u8, 182u8, 54u8, 180u8, 12u8, 61u8, 179u8, 216u8, 157u8, 103u8,
                            52u8, 120u8, 252u8, 83u8, 203u8, 144u8, 65u8, 15u8, 3u8, 21u8, 33u8,
                        ],
                    )
                }

                /// See [`Pallet::cancel_request`].
                pub fn cancel_request(
                    &self,
                    reg_index: types::cancel_request::RegIndex,
                ) -> ::subxt::tx::Payload<types::CancelRequest> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "cancel_request",
                        types::CancelRequest {
                            reg_index,
                        },
                        [
                            81u8, 14u8, 133u8, 219u8, 43u8, 84u8, 163u8, 208u8, 21u8, 185u8, 75u8,
                            117u8, 126u8, 33u8, 210u8, 106u8, 122u8, 210u8, 35u8, 207u8, 104u8,
                            206u8, 41u8, 117u8, 247u8, 108u8, 56u8, 23u8, 123u8, 169u8, 169u8,
                            61u8,
                        ],
                    )
                }

                /// See [`Pallet::set_fee`].
                pub fn set_fee(
                    &self,
                    index: types::set_fee::Index,
                    fee: types::set_fee::Fee,
                ) -> ::subxt::tx::Payload<types::SetFee> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_fee",
                        types::SetFee {
                            index,
                            fee,
                        },
                        [
                            131u8, 20u8, 17u8, 127u8, 180u8, 65u8, 225u8, 144u8, 193u8, 60u8,
                            131u8, 241u8, 30u8, 149u8, 8u8, 76u8, 29u8, 52u8, 102u8, 108u8, 127u8,
                            130u8, 70u8, 18u8, 94u8, 145u8, 179u8, 109u8, 252u8, 219u8, 58u8,
                            163u8,
                        ],
                    )
                }

                /// See [`Pallet::set_account_id`].
                pub fn set_account_id(
                    &self,
                    index: types::set_account_id::Index,
                    new: types::set_account_id::New,
                ) -> ::subxt::tx::Payload<types::SetAccountId> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_account_id",
                        types::SetAccountId {
                            index,
                            new,
                        },
                        [
                            68u8, 57u8, 39u8, 134u8, 39u8, 82u8, 156u8, 107u8, 113u8, 99u8, 9u8,
                            163u8, 58u8, 249u8, 247u8, 208u8, 38u8, 203u8, 54u8, 153u8, 116u8,
                            143u8, 81u8, 46u8, 228u8, 149u8, 127u8, 115u8, 252u8, 83u8, 33u8,
                            101u8,
                        ],
                    )
                }

                /// See [`Pallet::set_fields`].
                pub fn set_fields(
                    &self,
                    index: types::set_fields::Index,
                    fields: types::set_fields::Fields,
                ) -> ::subxt::tx::Payload<types::SetFields> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "set_fields",
                        types::SetFields {
                            index,
                            fields,
                        },
                        [
                            25u8, 129u8, 119u8, 232u8, 18u8, 32u8, 77u8, 23u8, 185u8, 56u8, 32u8,
                            199u8, 74u8, 174u8, 104u8, 203u8, 171u8, 253u8, 19u8, 225u8, 101u8,
                            239u8, 14u8, 242u8, 157u8, 51u8, 203u8, 74u8, 1u8, 65u8, 165u8, 205u8,
                        ],
                    )
                }

                /// See [`Pallet::provide_judgement`].
                pub fn provide_judgement(
                    &self,
                    reg_index: types::provide_judgement::RegIndex,
                    target: types::provide_judgement::Target,
                    judgement: types::provide_judgement::Judgement,
                    identity: types::provide_judgement::Identity,
                ) -> ::subxt::tx::Payload<types::ProvideJudgement> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "provide_judgement",
                        types::ProvideJudgement {
                            reg_index,
                            target,
                            judgement,
                            identity,
                        },
                        [
                            145u8, 188u8, 61u8, 236u8, 183u8, 49u8, 49u8, 149u8, 240u8, 184u8,
                            202u8, 75u8, 69u8, 0u8, 95u8, 103u8, 132u8, 24u8, 107u8, 221u8, 236u8,
                            75u8, 231u8, 125u8, 39u8, 189u8, 45u8, 202u8, 116u8, 123u8, 236u8,
                            96u8,
                        ],
                    )
                }

                /// See [`Pallet::kill_identity`].
                pub fn kill_identity(
                    &self,
                    target: types::kill_identity::Target,
                ) -> ::subxt::tx::Payload<types::KillIdentity> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "kill_identity",
                        types::KillIdentity {
                            target,
                        },
                        [
                            114u8, 249u8, 102u8, 62u8, 118u8, 105u8, 185u8, 61u8, 173u8, 52u8,
                            57u8, 190u8, 102u8, 74u8, 108u8, 239u8, 142u8, 176u8, 116u8, 51u8,
                            49u8, 197u8, 6u8, 183u8, 248u8, 202u8, 202u8, 140u8, 134u8, 59u8,
                            103u8, 182u8,
                        ],
                    )
                }

                /// See [`Pallet::add_sub`].
                pub fn add_sub(
                    &self,
                    sub: types::add_sub::Sub,
                    data: types::add_sub::Data,
                ) -> ::subxt::tx::Payload<types::AddSub> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "add_sub",
                        types::AddSub {
                            sub,
                            data,
                        },
                        [
                            3u8, 65u8, 137u8, 35u8, 238u8, 133u8, 56u8, 233u8, 37u8, 125u8, 221u8,
                            186u8, 153u8, 74u8, 69u8, 196u8, 244u8, 82u8, 51u8, 7u8, 216u8, 29u8,
                            18u8, 16u8, 198u8, 184u8, 0u8, 181u8, 71u8, 227u8, 144u8, 33u8,
                        ],
                    )
                }

                /// See [`Pallet::rename_sub`].
                pub fn rename_sub(
                    &self,
                    sub: types::rename_sub::Sub,
                    data: types::rename_sub::Data,
                ) -> ::subxt::tx::Payload<types::RenameSub> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "rename_sub",
                        types::RenameSub {
                            sub,
                            data,
                        },
                        [
                            252u8, 50u8, 201u8, 112u8, 49u8, 248u8, 223u8, 239u8, 219u8, 226u8,
                            64u8, 68u8, 227u8, 20u8, 30u8, 24u8, 36u8, 77u8, 26u8, 235u8, 144u8,
                            240u8, 11u8, 111u8, 145u8, 167u8, 184u8, 207u8, 173u8, 58u8, 152u8,
                            202u8,
                        ],
                    )
                }

                /// See [`Pallet::remove_sub`].
                pub fn remove_sub(
                    &self,
                    sub: types::remove_sub::Sub,
                ) -> ::subxt::tx::Payload<types::RemoveSub> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "remove_sub",
                        types::RemoveSub {
                            sub,
                        },
                        [
                            95u8, 249u8, 171u8, 27u8, 100u8, 186u8, 67u8, 214u8, 226u8, 6u8, 118u8,
                            39u8, 91u8, 122u8, 1u8, 87u8, 1u8, 226u8, 101u8, 9u8, 199u8, 167u8,
                            84u8, 202u8, 141u8, 196u8, 80u8, 195u8, 15u8, 114u8, 140u8, 144u8,
                        ],
                    )
                }

                /// See [`Pallet::quit_sub`].
                pub fn quit_sub(&self) -> ::subxt::tx::Payload<types::QuitSub> {
                    ::subxt::tx::Payload::new_static(
                        "Identity",
                        "quit_sub",
                        types::QuitSub {},
                        [
                            147u8, 131u8, 175u8, 171u8, 187u8, 201u8, 240u8, 26u8, 146u8, 224u8,
                            74u8, 166u8, 242u8, 193u8, 204u8, 247u8, 168u8, 93u8, 18u8, 32u8, 27u8,
                            208u8, 149u8, 146u8, 179u8, 172u8, 75u8, 112u8, 84u8, 141u8, 233u8,
                            223u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::pallet_identity::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A name was set or reset (which will remove all judgements).
            pub struct IdentitySet {
                pub who: identity_set::Who,
            }
            pub mod identity_set {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for IdentitySet {
                const EVENT: &'static str = "IdentitySet";
                const PALLET: &'static str = "Identity";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A name was cleared, and the given balance returned.
            pub struct IdentityCleared {
                pub who: identity_cleared::Who,
                pub deposit: identity_cleared::Deposit,
            }
            pub mod identity_cleared {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Deposit = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for IdentityCleared {
                const EVENT: &'static str = "IdentityCleared";
                const PALLET: &'static str = "Identity";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A name was removed and the given balance slashed.
            pub struct IdentityKilled {
                pub who: identity_killed::Who,
                pub deposit: identity_killed::Deposit,
            }
            pub mod identity_killed {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Deposit = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for IdentityKilled {
                const EVENT: &'static str = "IdentityKilled";
                const PALLET: &'static str = "Identity";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A judgement was asked from a registrar.
            pub struct JudgementRequested {
                pub who: judgement_requested::Who,
                pub registrar_index: judgement_requested::RegistrarIndex,
            }
            pub mod judgement_requested {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type RegistrarIndex = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for JudgementRequested {
                const EVENT: &'static str = "JudgementRequested";
                const PALLET: &'static str = "Identity";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A judgement request was retracted.
            pub struct JudgementUnrequested {
                pub who: judgement_unrequested::Who,
                pub registrar_index: judgement_unrequested::RegistrarIndex,
            }
            pub mod judgement_unrequested {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type RegistrarIndex = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for JudgementUnrequested {
                const EVENT: &'static str = "JudgementUnrequested";
                const PALLET: &'static str = "Identity";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A judgement was given by a registrar.
            pub struct JudgementGiven {
                pub target: judgement_given::Target,
                pub registrar_index: judgement_given::RegistrarIndex,
            }
            pub mod judgement_given {
                use super::runtime_types;
                pub type Target = ::subxt::utils::AccountId32;
                pub type RegistrarIndex = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for JudgementGiven {
                const EVENT: &'static str = "JudgementGiven";
                const PALLET: &'static str = "Identity";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A registrar was added.
            pub struct RegistrarAdded {
                pub registrar_index: registrar_added::RegistrarIndex,
            }
            pub mod registrar_added {
                use super::runtime_types;
                pub type RegistrarIndex = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for RegistrarAdded {
                const EVENT: &'static str = "RegistrarAdded";
                const PALLET: &'static str = "Identity";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A sub-identity was added to an identity and the deposit paid.
            pub struct SubIdentityAdded {
                pub sub: sub_identity_added::Sub,
                pub main: sub_identity_added::Main,
                pub deposit: sub_identity_added::Deposit,
            }
            pub mod sub_identity_added {
                use super::runtime_types;
                pub type Sub = ::subxt::utils::AccountId32;
                pub type Main = ::subxt::utils::AccountId32;
                pub type Deposit = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for SubIdentityAdded {
                const EVENT: &'static str = "SubIdentityAdded";
                const PALLET: &'static str = "Identity";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A sub-identity was removed from an identity and the deposit
            /// freed.
            pub struct SubIdentityRemoved {
                pub sub: sub_identity_removed::Sub,
                pub main: sub_identity_removed::Main,
                pub deposit: sub_identity_removed::Deposit,
            }
            pub mod sub_identity_removed {
                use super::runtime_types;
                pub type Sub = ::subxt::utils::AccountId32;
                pub type Main = ::subxt::utils::AccountId32;
                pub type Deposit = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for SubIdentityRemoved {
                const EVENT: &'static str = "SubIdentityRemoved";
                const PALLET: &'static str = "Identity";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// A sub-identity was cleared, and the given deposit repatriated
            /// from the
            /// main identity account to the sub-identity account.
            pub struct SubIdentityRevoked {
                pub sub: sub_identity_revoked::Sub,
                pub main: sub_identity_revoked::Main,
                pub deposit: sub_identity_revoked::Deposit,
            }
            pub mod sub_identity_revoked {
                use super::runtime_types;
                pub type Sub = ::subxt::utils::AccountId32;
                pub type Main = ::subxt::utils::AccountId32;
                pub type Deposit = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for SubIdentityRevoked {
                const EVENT: &'static str = "SubIdentityRevoked";
                const PALLET: &'static str = "Identity";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod identity_of {
                    use super::runtime_types;
                    pub type IdentityOf = runtime_types::pallet_identity::types::Registration<
                        ::core::primitive::u128,
                    >;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod super_of {
                    use super::runtime_types;
                    pub type SuperOf = (
                        ::subxt::utils::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    );
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod subs_of {
                    use super::runtime_types;
                    pub type SubsOf = (
                        ::core::primitive::u128,
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::subxt::utils::AccountId32,
                        >,
                    );
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
                pub mod registrars {
                    use super::runtime_types;
                    pub type Registrars =
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::option::Option<
                                runtime_types::pallet_identity::types::RegistrarInfo<
                                    ::core::primitive::u128,
                                    ::subxt::utils::AccountId32,
                                >,
                            >,
                        >;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                /// Information that is pertinent to identify the entity behind
                /// an account.
                ///
                /// TWOX-NOTE: OK ― `AccountId` is a secure hash.
                pub fn identity_of_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::identity_of::IdentityOf,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "IdentityOf",
                        vec![],
                        [
                            112u8, 2u8, 209u8, 123u8, 138u8, 171u8, 80u8, 243u8, 226u8, 88u8, 81u8,
                            49u8, 59u8, 172u8, 88u8, 180u8, 255u8, 119u8, 57u8, 16u8, 169u8, 149u8,
                            77u8, 239u8, 73u8, 182u8, 28u8, 112u8, 150u8, 110u8, 65u8, 139u8,
                        ],
                    )
                }

                /// Information that is pertinent to identify the entity behind
                /// an account.
                ///
                /// TWOX-NOTE: OK ― `AccountId` is a secure hash.
                pub fn identity_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::identity_of::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::identity_of::IdentityOf,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "IdentityOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            112u8, 2u8, 209u8, 123u8, 138u8, 171u8, 80u8, 243u8, 226u8, 88u8, 81u8,
                            49u8, 59u8, 172u8, 88u8, 180u8, 255u8, 119u8, 57u8, 16u8, 169u8, 149u8,
                            77u8, 239u8, 73u8, 182u8, 28u8, 112u8, 150u8, 110u8, 65u8, 139u8,
                        ],
                    )
                }

                /// The super-identity of an alternative "sub" identity together
                /// with its name, within that
                /// context. If the account is not some other account's
                /// sub-identity, then just `None`.
                pub fn super_of_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::super_of::SuperOf,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "SuperOf",
                        vec![],
                        [
                            84u8, 72u8, 64u8, 14u8, 56u8, 9u8, 143u8, 100u8, 141u8, 163u8, 36u8,
                            55u8, 38u8, 254u8, 164u8, 17u8, 3u8, 110u8, 88u8, 175u8, 161u8, 65u8,
                            159u8, 40u8, 46u8, 8u8, 177u8, 81u8, 130u8, 38u8, 193u8, 28u8,
                        ],
                    )
                }

                /// The super-identity of an alternative "sub" identity together
                /// with its name, within that
                /// context. If the account is not some other account's
                /// sub-identity, then just `None`.
                pub fn super_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::super_of::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::super_of::SuperOf,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "SuperOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            84u8, 72u8, 64u8, 14u8, 56u8, 9u8, 143u8, 100u8, 141u8, 163u8, 36u8,
                            55u8, 38u8, 254u8, 164u8, 17u8, 3u8, 110u8, 88u8, 175u8, 161u8, 65u8,
                            159u8, 40u8, 46u8, 8u8, 177u8, 81u8, 130u8, 38u8, 193u8, 28u8,
                        ],
                    )
                }

                /// Alternative "sub" identities of this account.
                ///
                /// The first item is the deposit, the second is a vector of the
                /// accounts.
                ///
                /// TWOX-NOTE: OK ― `AccountId` is a secure hash.
                pub fn subs_of_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::subs_of::SubsOf,
                    (),
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "SubsOf",
                        vec![],
                        [
                            164u8, 140u8, 52u8, 123u8, 220u8, 118u8, 147u8, 3u8, 67u8, 22u8, 191u8,
                            18u8, 186u8, 21u8, 154u8, 8u8, 205u8, 224u8, 163u8, 173u8, 174u8,
                            107u8, 144u8, 215u8, 116u8, 64u8, 159u8, 115u8, 159u8, 205u8, 91u8,
                            28u8,
                        ],
                    )
                }

                /// Alternative "sub" identities of this account.
                ///
                /// The first item is the deposit, the second is a vector of the
                /// accounts.
                ///
                /// TWOX-NOTE: OK ― `AccountId` is a secure hash.
                pub fn subs_of(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::subs_of::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::subs_of::SubsOf,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "SubsOf",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            164u8, 140u8, 52u8, 123u8, 220u8, 118u8, 147u8, 3u8, 67u8, 22u8, 191u8,
                            18u8, 186u8, 21u8, 154u8, 8u8, 205u8, 224u8, 163u8, 173u8, 174u8,
                            107u8, 144u8, 215u8, 116u8, 64u8, 159u8, 115u8, 159u8, 205u8, 91u8,
                            28u8,
                        ],
                    )
                }

                /// The set of registrars. Not expected to get very big as can
                /// only be added through a
                /// special origin (likely a council motion).
                ///
                /// The index into this can be cast to `RegistrarIndex` to get a
                /// valid value.
                pub fn registrars(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::registrars::Registrars,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "Identity",
                        "Registrars",
                        vec![],
                        [
                            207u8, 253u8, 229u8, 237u8, 228u8, 85u8, 173u8, 74u8, 164u8, 67u8,
                            144u8, 144u8, 5u8, 242u8, 84u8, 187u8, 110u8, 181u8, 2u8, 162u8, 239u8,
                            212u8, 72u8, 233u8, 160u8, 196u8, 121u8, 218u8, 100u8, 0u8, 219u8,
                            181u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// The amount held on deposit for a registered identity
                pub fn basic_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "BasicDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The amount held on deposit per additional field for a
                /// registered identity.
                pub fn field_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "FieldDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The amount held on deposit for a registered subaccount. This
                /// should account for the fact
                /// that one storage item's value will increase by the size of
                /// an account ID, and there will
                /// be another trie item whose value is the size of an account
                /// ID plus 32 bytes.
                pub fn sub_account_deposit(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "SubAccountDeposit",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// The maximum number of sub-accounts allowed per identified
                /// account.
                pub fn max_sub_accounts(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "MaxSubAccounts",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// Maximum number of additional fields that may be stored in an
                /// ID. Needed to bound the I/O
                /// required to access an identity, but can be pretty high.
                pub fn max_additional_fields(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "MaxAdditionalFields",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// Maxmimum number of registrars allowed in the system. Needed
                /// to bound the complexity
                /// of, e.g., updating judgements.
                pub fn max_registrars(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "Identity",
                        "MaxRegistrars",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod big_brother_council {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::nagara_council_bigbrothers::pallet::Error;
        /// Dispatchable functions.
        pub type Call = runtime_types::nagara_council_bigbrothers::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::su_elder_replace`].
                pub struct SuElderReplace {
                    pub new_elder: su_elder_replace::NewElder,
                }
                pub mod su_elder_replace {
                    use super::runtime_types;
                    pub type NewElder = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for SuElderReplace {
                    const CALL: &'static str = "su_elder_replace";
                    const PALLET: &'static str = "BigBrotherCouncil";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::se_membership_add`].
                pub struct SeMembershipAdd {
                    pub new_council_member: se_membership_add::NewCouncilMember,
                }
                pub mod se_membership_add {
                    use super::runtime_types;
                    pub type NewCouncilMember = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for SeMembershipAdd {
                    const CALL: &'static str = "se_membership_add";
                    const PALLET: &'static str = "BigBrotherCouncil";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::se_membership_remove`].
                pub struct SeMembershipRemove {
                    pub council_member: se_membership_remove::CouncilMember,
                }
                pub mod se_membership_remove {
                    use super::runtime_types;
                    pub type CouncilMember = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for SeMembershipRemove {
                    const CALL: &'static str = "se_membership_remove";
                    const PALLET: &'static str = "BigBrotherCouncil";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::se_currency_mint_into`].
                pub struct SeCurrencyMintInto {
                    pub dest: se_currency_mint_into::Dest,
                    pub amount: se_currency_mint_into::Amount,
                }
                pub mod se_currency_mint_into {
                    use super::runtime_types;
                    pub type Dest = ::subxt::utils::AccountId32;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for SeCurrencyMintInto {
                    const CALL: &'static str = "se_currency_mint_into";
                    const PALLET: &'static str = "BigBrotherCouncil";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::se_currency_burn`].
                pub struct SeCurrencyBurn {
                    pub amount: se_currency_burn::Amount,
                }
                pub mod se_currency_burn {
                    use super::runtime_types;
                    pub type Amount = ::core::primitive::u128;
                }
                impl ::subxt::blocks::StaticExtrinsic for SeCurrencyBurn {
                    const CALL: &'static str = "se_currency_burn";
                    const PALLET: &'static str = "BigBrotherCouncil";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::se_currency_burn_all`].
                pub struct SeCurrencyBurnAll;
                impl ::subxt::blocks::StaticExtrinsic for SeCurrencyBurnAll {
                    const CALL: &'static str = "se_currency_burn_all";
                    const PALLET: &'static str = "BigBrotherCouncil";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::cm_proposal_new`].
                pub struct CmProposalNew {
                    pub new_multiplier: cm_proposal_new::NewMultiplier,
                    pub new_divider: cm_proposal_new::NewDivider,
                    pub new_minimum_fee: cm_proposal_new::NewMinimumFee,
                }
                pub mod cm_proposal_new {
                    use super::runtime_types;
                    pub type NewMultiplier = ::core::option::Option<::core::primitive::u64>;
                    pub type NewDivider = ::core::option::Option<::core::primitive::u64>;
                    pub type NewMinimumFee = ::core::option::Option<::core::primitive::u128>;
                }
                impl ::subxt::blocks::StaticExtrinsic for CmProposalNew {
                    const CALL: &'static str = "cm_proposal_new";
                    const PALLET: &'static str = "BigBrotherCouncil";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::cm_proposal_vote`].
                pub struct CmProposalVote {
                    pub is_approving: cm_proposal_vote::IsApproving,
                }
                pub mod cm_proposal_vote {
                    use super::runtime_types;
                    pub type IsApproving = ::core::primitive::bool;
                }
                impl ::subxt::blocks::StaticExtrinsic for CmProposalVote {
                    const CALL: &'static str = "cm_proposal_vote";
                    const PALLET: &'static str = "BigBrotherCouncil";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::su_elder_replace`].
                pub fn su_elder_replace(
                    &self,
                    new_elder: types::su_elder_replace::NewElder,
                ) -> ::subxt::tx::Payload<types::SuElderReplace> {
                    ::subxt::tx::Payload::new_static(
                        "BigBrotherCouncil",
                        "su_elder_replace",
                        types::SuElderReplace {
                            new_elder,
                        },
                        [
                            232u8, 113u8, 200u8, 126u8, 196u8, 47u8, 123u8, 172u8, 48u8, 165u8,
                            255u8, 84u8, 179u8, 210u8, 56u8, 111u8, 51u8, 175u8, 150u8, 180u8,
                            167u8, 135u8, 116u8, 198u8, 27u8, 218u8, 4u8, 55u8, 95u8, 167u8, 67u8,
                            128u8,
                        ],
                    )
                }

                /// See [`Pallet::se_membership_add`].
                pub fn se_membership_add(
                    &self,
                    new_council_member: types::se_membership_add::NewCouncilMember,
                ) -> ::subxt::tx::Payload<types::SeMembershipAdd> {
                    ::subxt::tx::Payload::new_static(
                        "BigBrotherCouncil",
                        "se_membership_add",
                        types::SeMembershipAdd {
                            new_council_member,
                        },
                        [
                            120u8, 76u8, 179u8, 109u8, 38u8, 82u8, 58u8, 122u8, 56u8, 6u8, 109u8,
                            21u8, 143u8, 28u8, 39u8, 41u8, 29u8, 67u8, 90u8, 102u8, 111u8, 133u8,
                            253u8, 228u8, 2u8, 138u8, 217u8, 69u8, 179u8, 31u8, 254u8, 17u8,
                        ],
                    )
                }

                /// See [`Pallet::se_membership_remove`].
                pub fn se_membership_remove(
                    &self,
                    council_member: types::se_membership_remove::CouncilMember,
                ) -> ::subxt::tx::Payload<types::SeMembershipRemove> {
                    ::subxt::tx::Payload::new_static(
                        "BigBrotherCouncil",
                        "se_membership_remove",
                        types::SeMembershipRemove {
                            council_member,
                        },
                        [
                            199u8, 96u8, 188u8, 97u8, 242u8, 78u8, 188u8, 118u8, 164u8, 99u8,
                            132u8, 248u8, 28u8, 79u8, 204u8, 229u8, 42u8, 149u8, 45u8, 137u8, 46u8,
                            170u8, 83u8, 103u8, 41u8, 79u8, 46u8, 37u8, 37u8, 236u8, 179u8, 55u8,
                        ],
                    )
                }

                /// See [`Pallet::se_currency_mint_into`].
                pub fn se_currency_mint_into(
                    &self,
                    dest: types::se_currency_mint_into::Dest,
                    amount: types::se_currency_mint_into::Amount,
                ) -> ::subxt::tx::Payload<types::SeCurrencyMintInto> {
                    ::subxt::tx::Payload::new_static(
                        "BigBrotherCouncil",
                        "se_currency_mint_into",
                        types::SeCurrencyMintInto {
                            dest,
                            amount,
                        },
                        [
                            191u8, 250u8, 193u8, 43u8, 189u8, 177u8, 236u8, 148u8, 11u8, 241u8,
                            125u8, 251u8, 254u8, 53u8, 102u8, 226u8, 138u8, 74u8, 25u8, 250u8,
                            73u8, 53u8, 30u8, 189u8, 5u8, 220u8, 117u8, 41u8, 189u8, 210u8, 137u8,
                            117u8,
                        ],
                    )
                }

                /// See [`Pallet::se_currency_burn`].
                pub fn se_currency_burn(
                    &self,
                    amount: types::se_currency_burn::Amount,
                ) -> ::subxt::tx::Payload<types::SeCurrencyBurn> {
                    ::subxt::tx::Payload::new_static(
                        "BigBrotherCouncil",
                        "se_currency_burn",
                        types::SeCurrencyBurn {
                            amount,
                        },
                        [
                            134u8, 190u8, 221u8, 186u8, 132u8, 135u8, 228u8, 115u8, 100u8, 166u8,
                            187u8, 181u8, 169u8, 55u8, 99u8, 238u8, 151u8, 120u8, 222u8, 249u8,
                            126u8, 187u8, 191u8, 35u8, 140u8, 59u8, 95u8, 53u8, 42u8, 99u8, 67u8,
                            201u8,
                        ],
                    )
                }

                /// See [`Pallet::se_currency_burn_all`].
                pub fn se_currency_burn_all(
                    &self,
                ) -> ::subxt::tx::Payload<types::SeCurrencyBurnAll> {
                    ::subxt::tx::Payload::new_static(
                        "BigBrotherCouncil",
                        "se_currency_burn_all",
                        types::SeCurrencyBurnAll {},
                        [
                            58u8, 31u8, 132u8, 21u8, 79u8, 104u8, 20u8, 77u8, 84u8, 43u8, 218u8,
                            50u8, 30u8, 6u8, 227u8, 244u8, 106u8, 169u8, 31u8, 140u8, 155u8, 235u8,
                            247u8, 25u8, 112u8, 67u8, 55u8, 161u8, 104u8, 118u8, 105u8, 249u8,
                        ],
                    )
                }

                /// See [`Pallet::cm_proposal_new`].
                pub fn cm_proposal_new(
                    &self,
                    new_multiplier: types::cm_proposal_new::NewMultiplier,
                    new_divider: types::cm_proposal_new::NewDivider,
                    new_minimum_fee: types::cm_proposal_new::NewMinimumFee,
                ) -> ::subxt::tx::Payload<types::CmProposalNew> {
                    ::subxt::tx::Payload::new_static(
                        "BigBrotherCouncil",
                        "cm_proposal_new",
                        types::CmProposalNew {
                            new_multiplier,
                            new_divider,
                            new_minimum_fee,
                        },
                        [
                            71u8, 146u8, 255u8, 25u8, 112u8, 210u8, 3u8, 116u8, 58u8, 13u8, 80u8,
                            116u8, 192u8, 208u8, 126u8, 41u8, 69u8, 226u8, 15u8, 223u8, 161u8,
                            171u8, 88u8, 174u8, 226u8, 85u8, 76u8, 91u8, 72u8, 91u8, 74u8, 37u8,
                        ],
                    )
                }

                /// See [`Pallet::cm_proposal_vote`].
                pub fn cm_proposal_vote(
                    &self,
                    is_approving: types::cm_proposal_vote::IsApproving,
                ) -> ::subxt::tx::Payload<types::CmProposalVote> {
                    ::subxt::tx::Payload::new_static(
                        "BigBrotherCouncil",
                        "cm_proposal_vote",
                        types::CmProposalVote {
                            is_approving,
                        },
                        [
                            112u8, 110u8, 111u8, 246u8, 123u8, 155u8, 229u8, 11u8, 12u8, 122u8,
                            138u8, 211u8, 86u8, 219u8, 157u8, 137u8, 68u8, 76u8, 183u8, 161u8,
                            71u8, 195u8, 0u8, 127u8, 105u8, 4u8, 44u8, 153u8, 9u8, 98u8, 75u8,
                            135u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::nagara_council_bigbrothers::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Big Brother Registered/Added
            pub struct BigBrotherAdded {
                pub who: big_brother_added::Who,
                pub by: big_brother_added::By,
                pub hold: big_brother_added::Hold,
            }
            pub mod big_brother_added {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type By = ::core::option::Option<::subxt::utils::AccountId32>;
                pub type Hold = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for BigBrotherAdded {
                const EVENT: &'static str = "BigBrotherAdded";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Big Brother Unregistered/Removed
            pub struct BigBrotherRemoved {
                pub who: big_brother_removed::Who,
                pub by: big_brother_removed::By,
                pub release: big_brother_removed::Release,
            }
            pub mod big_brother_removed {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type By = ::core::option::Option<::subxt::utils::AccountId32>;
                pub type Release = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for BigBrotherRemoved {
                const EVENT: &'static str = "BigBrotherRemoved";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Elder ascended
            pub struct ElderAscended {
                pub who: elder_ascended::Who,
            }
            pub mod elder_ascended {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for ElderAscended {
                const EVENT: &'static str = "ElderAscended";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Elder descended
            pub struct ElderDescended {
                pub who: elder_descended::Who,
            }
            pub mod elder_descended {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for ElderDescended {
                const EVENT: &'static str = "ElderDescended";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Token circulation increased
            pub struct CirculationIncreased {
                pub increase: circulation_increased::Increase,
                pub by: circulation_increased::By,
            }
            pub mod circulation_increased {
                use super::runtime_types;
                pub type Increase = ::core::primitive::u128;
                pub type By = ::core::option::Option<::subxt::utils::AccountId32>;
            }
            impl ::subxt::events::StaticEvent for CirculationIncreased {
                const EVENT: &'static str = "CirculationIncreased";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Token circulation decreased
            pub struct CirculationDecreased {
                pub decrease: circulation_decreased::Decrease,
                pub by: circulation_decreased::By,
            }
            pub mod circulation_decreased {
                use super::runtime_types;
                pub type Decrease = ::core::primitive::u128;
                pub type By = ::core::option::Option<::subxt::utils::AccountId32>;
            }
            impl ::subxt::events::StaticEvent for CirculationDecreased {
                const EVENT: &'static str = "CirculationDecreased";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Transaction Fee parameters changed
            pub struct TxFeeParametersChange {
                pub old: tx_fee_parameters_change::Old,
                pub new: tx_fee_parameters_change::New,
            }
            pub mod tx_fee_parameters_change {
                use super::runtime_types;
                pub type Old =
                    runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<
                        ::core::primitive::u128,
                    >;
                pub type New =
                    runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<
                        ::core::primitive::u128,
                    >;
            }
            impl ::subxt::events::StaticEvent for TxFeeParametersChange {
                const EVENT: &'static str = "TxFeeParametersChange";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Transaction fee parameters proposal rejected
            pub struct TxFeeParametersRejected {
                pub rejected: tx_fee_parameters_rejected::Rejected,
                pub by: tx_fee_parameters_rejected::By,
            }
            pub mod tx_fee_parameters_rejected {
                use super::runtime_types;
                pub type Rejected =
                    runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<
                        ::core::primitive::u128,
                    >;
                pub type By = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for TxFeeParametersRejected {
                const EVENT: &'static str = "TxFeeParametersRejected";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// New transaction fee parameters proposal
            pub struct TxFeeParametersChangeProposed {
                pub proposal: tx_fee_parameters_change_proposed::Proposal,
                pub by: tx_fee_parameters_change_proposed::By,
            }
            pub mod tx_fee_parameters_change_proposed {
                use super::runtime_types;
                pub type Proposal =
                    runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<
                        ::core::primitive::u128,
                    >;
                pub type By = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for TxFeeParametersChangeProposed {
                const EVENT: &'static str = "TxFeeParametersChangeProposed";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Transaction fee parameters proposal vote count
            pub struct TxFeeParametersChangeVoted {
                pub by: tx_fee_parameters_change_voted::By,
                pub remaining_count: tx_fee_parameters_change_voted::RemainingCount,
            }
            pub mod tx_fee_parameters_change_voted {
                use super::runtime_types;
                pub type By = ::subxt::utils::AccountId32;
                pub type RemainingCount = ::core::primitive::u32;
            }
            impl ::subxt::events::StaticEvent for TxFeeParametersChangeVoted {
                const EVENT: &'static str = "TxFeeParametersChangeVoted";
                const PALLET: &'static str = "BigBrotherCouncil";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod elder {
                    use super::runtime_types;
                    pub type Elder = ::subxt::utils::AccountId32;
                }
                pub mod members {
                    use super::runtime_types;
                    pub type Members =
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::utils::AccountId32,
                        >;
                }
                pub mod tx_fee_info {
                    use super::runtime_types;
                    pub type TxFeeInfo =
                        runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<
                            ::core::primitive::u128,
                        >;
                }
                pub mod current_proposal {
                    use super::runtime_types;
                    pub type CurrentProposal = runtime_types :: nagara_council_bigbrothers :: pallet :: TransactionFeeChangeProposal < :: subxt :: utils :: AccountId32 , :: core :: primitive :: u128 , :: core :: primitive :: u32 > ;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn elder(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::elder::Elder,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "BigBrotherCouncil",
                        "Elder",
                        vec![],
                        [
                            97u8, 241u8, 24u8, 32u8, 157u8, 234u8, 22u8, 100u8, 57u8, 112u8, 48u8,
                            62u8, 99u8, 169u8, 253u8, 89u8, 130u8, 229u8, 252u8, 252u8, 100u8,
                            32u8, 160u8, 159u8, 71u8, 75u8, 80u8, 237u8, 111u8, 208u8, 98u8, 175u8,
                        ],
                    )
                }

                pub fn members(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::members::Members,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "BigBrotherCouncil",
                        "Members",
                        vec![],
                        [
                            242u8, 212u8, 123u8, 37u8, 234u8, 60u8, 193u8, 98u8, 79u8, 250u8,
                            208u8, 198u8, 199u8, 169u8, 25u8, 45u8, 184u8, 217u8, 72u8, 57u8, 5u8,
                            149u8, 227u8, 162u8, 31u8, 196u8, 20u8, 181u8, 139u8, 239u8, 241u8,
                            35u8,
                        ],
                    )
                }

                pub fn tx_fee_info(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::tx_fee_info::TxFeeInfo,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "BigBrotherCouncil",
                        "TxFeeInfo",
                        vec![],
                        [
                            88u8, 33u8, 26u8, 195u8, 237u8, 14u8, 228u8, 232u8, 164u8, 85u8, 187u8,
                            41u8, 131u8, 6u8, 111u8, 204u8, 127u8, 103u8, 155u8, 84u8, 25u8, 88u8,
                            211u8, 127u8, 105u8, 169u8, 11u8, 69u8, 214u8, 69u8, 239u8, 188u8,
                        ],
                    )
                }

                pub fn current_proposal(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::current_proposal::CurrentProposal,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "BigBrotherCouncil",
                        "CurrentProposal",
                        vec![],
                        [
                            246u8, 207u8, 78u8, 68u8, 193u8, 38u8, 41u8, 207u8, 96u8, 109u8, 34u8,
                            82u8, 63u8, 131u8, 255u8, 138u8, 104u8, 56u8, 201u8, 196u8, 200u8,
                            56u8, 32u8, 59u8, 180u8, 90u8, 44u8, 167u8, 40u8, 239u8, 156u8, 222u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// Council Membership deposit amount
                pub fn registration_deposit_amount(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "BigBrotherCouncil",
                        "RegistrationDepositAmount",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// Maximum members (Big Brothers)
                pub fn max_members(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "BigBrotherCouncil",
                        "MaxMembers",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }

                /// Chain's Burn Address
                pub fn burn_address(
                    &self,
                ) -> ::subxt::constants::Address<::subxt::utils::AccountId32> {
                    ::subxt::constants::Address::new_static(
                        "BigBrotherCouncil",
                        "BurnAddress",
                        [
                            115u8, 233u8, 13u8, 223u8, 88u8, 20u8, 202u8, 139u8, 153u8, 28u8,
                            155u8, 157u8, 224u8, 66u8, 3u8, 250u8, 23u8, 53u8, 88u8, 168u8, 211u8,
                            204u8, 122u8, 166u8, 248u8, 23u8, 174u8, 225u8, 99u8, 108u8, 89u8,
                            135u8,
                        ],
                    )
                }

                /// Initial Weight to Fee Divider
                pub fn initial_weight_to_fee_divider(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "BigBrotherCouncil",
                        "InitialWeightToFeeDivider",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }

                /// Initial Weight to Fee Multiplier
                pub fn initial_weight_to_fee_multiplier(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u64> {
                    ::subxt::constants::Address::new_static(
                        "BigBrotherCouncil",
                        "InitialWeightToFeeMultiplier",
                        [
                            128u8, 214u8, 205u8, 242u8, 181u8, 142u8, 124u8, 231u8, 190u8, 146u8,
                            59u8, 226u8, 157u8, 101u8, 103u8, 117u8, 249u8, 65u8, 18u8, 191u8,
                            103u8, 119u8, 53u8, 85u8, 81u8, 96u8, 220u8, 42u8, 184u8, 239u8, 42u8,
                            246u8,
                        ],
                    )
                }

                /// Initial Minimum Transaction Fee
                pub fn initial_minimum_transaction_fee(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "BigBrotherCouncil",
                        "InitialMinimumTransactionFee",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod servicer_registry {
        use super::root_mod;
        use super::runtime_types;
        /// The `Error` enum of this pallet.
        pub type Error = runtime_types::nagara_registry_servicers::pallet::Error;
        /// Dispatchable functions.
        pub type Call = runtime_types::nagara_registry_servicers::pallet::Call;
        pub mod calls {
            use super::root_mod;
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::bb_attester_supply`].
                pub struct BbAttesterSupply {
                    pub supply_args: bb_attester_supply::SupplyArgs,
                }
                pub mod bb_attester_supply {
                    use super::runtime_types;
                    pub type SupplyArgs = runtime_types :: nagara_registry_servicers :: pallet :: RemoteAttestationDeviceSupplyArgs ;
                }
                impl ::subxt::blocks::StaticExtrinsic for BbAttesterSupply {
                    const CALL: &'static str = "bb_attester_supply";
                    const PALLET: &'static str = "ServicerRegistry";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::bb_mediator_add`].
                pub struct BbMediatorAdd {
                    pub who: bb_mediator_add::Who,
                }
                pub mod bb_mediator_add {
                    use super::runtime_types;
                    pub type Who = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for BbMediatorAdd {
                    const CALL: &'static str = "bb_mediator_add";
                    const PALLET: &'static str = "ServicerRegistry";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::bb_mediator_remove`].
                pub struct BbMediatorRemove {
                    pub who: bb_mediator_remove::Who,
                }
                pub mod bb_mediator_remove {
                    use super::runtime_types;
                    pub type Who = ::subxt::utils::AccountId32;
                }
                impl ::subxt::blocks::StaticExtrinsic for BbMediatorRemove {
                    const CALL: &'static str = "bb_mediator_remove";
                    const PALLET: &'static str = "ServicerRegistry";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::bb_attester_recall`].
                pub struct BbAttesterRecall {
                    pub attester_id: bb_attester_recall::AttesterId,
                }
                pub mod bb_attester_recall {
                    use super::runtime_types;
                    pub type AttesterId = runtime_types::sp_core::ed25519::Public;
                }
                impl ::subxt::blocks::StaticExtrinsic for BbAttesterRecall {
                    const CALL: &'static str = "bb_attester_recall";
                    const PALLET: &'static str = "ServicerRegistry";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::sv_attester_bind`].
                pub struct SvAttesterBind {
                    pub peer_id: sv_attester_bind::PeerId,
                    pub attester_id: sv_attester_bind::AttesterId,
                }
                pub mod sv_attester_bind {
                    use super::runtime_types;
                    pub type PeerId = runtime_types::sp_core::ed25519::Public;
                    pub type AttesterId = runtime_types::sp_core::ed25519::Public;
                }
                impl ::subxt::blocks::StaticExtrinsic for SvAttesterBind {
                    const CALL: &'static str = "sv_attester_bind";
                    const PALLET: &'static str = "ServicerRegistry";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::md_rep_increase`].
                pub struct MdRepIncrease {
                    pub on: md_rep_increase::On,
                }
                pub mod md_rep_increase {
                    use super::runtime_types;
                    pub type On = runtime_types::sp_core::ed25519::Public;
                }
                impl ::subxt::blocks::StaticExtrinsic for MdRepIncrease {
                    const CALL: &'static str = "md_rep_increase";
                    const PALLET: &'static str = "ServicerRegistry";
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// See [`Pallet::md_rep_decrease`].
                pub struct MdRepDecrease {
                    pub on: md_rep_decrease::On,
                }
                pub mod md_rep_decrease {
                    use super::runtime_types;
                    pub type On = runtime_types::sp_core::ed25519::Public;
                }
                impl ::subxt::blocks::StaticExtrinsic for MdRepDecrease {
                    const CALL: &'static str = "md_rep_decrease";
                    const PALLET: &'static str = "ServicerRegistry";
                }
            }
            pub struct TransactionApi;
            impl TransactionApi {
                /// See [`Pallet::bb_attester_supply`].
                pub fn bb_attester_supply(
                    &self,
                    supply_args: types::bb_attester_supply::SupplyArgs,
                ) -> ::subxt::tx::Payload<types::BbAttesterSupply> {
                    ::subxt::tx::Payload::new_static(
                        "ServicerRegistry",
                        "bb_attester_supply",
                        types::BbAttesterSupply {
                            supply_args,
                        },
                        [
                            177u8, 151u8, 119u8, 242u8, 105u8, 141u8, 46u8, 166u8, 75u8, 200u8,
                            112u8, 49u8, 244u8, 238u8, 183u8, 78u8, 194u8, 83u8, 213u8, 131u8,
                            179u8, 208u8, 10u8, 42u8, 148u8, 1u8, 21u8, 114u8, 42u8, 202u8, 33u8,
                            114u8,
                        ],
                    )
                }

                /// See [`Pallet::bb_mediator_add`].
                pub fn bb_mediator_add(
                    &self,
                    who: types::bb_mediator_add::Who,
                ) -> ::subxt::tx::Payload<types::BbMediatorAdd> {
                    ::subxt::tx::Payload::new_static(
                        "ServicerRegistry",
                        "bb_mediator_add",
                        types::BbMediatorAdd {
                            who,
                        },
                        [
                            126u8, 161u8, 233u8, 19u8, 8u8, 244u8, 26u8, 172u8, 248u8, 108u8,
                            183u8, 192u8, 112u8, 108u8, 153u8, 194u8, 175u8, 207u8, 131u8, 33u8,
                            193u8, 193u8, 60u8, 128u8, 57u8, 80u8, 117u8, 96u8, 245u8, 40u8, 69u8,
                            19u8,
                        ],
                    )
                }

                /// See [`Pallet::bb_mediator_remove`].
                pub fn bb_mediator_remove(
                    &self,
                    who: types::bb_mediator_remove::Who,
                ) -> ::subxt::tx::Payload<types::BbMediatorRemove> {
                    ::subxt::tx::Payload::new_static(
                        "ServicerRegistry",
                        "bb_mediator_remove",
                        types::BbMediatorRemove {
                            who,
                        },
                        [
                            67u8, 254u8, 146u8, 27u8, 42u8, 99u8, 43u8, 93u8, 235u8, 201u8, 81u8,
                            60u8, 52u8, 71u8, 128u8, 72u8, 120u8, 165u8, 236u8, 121u8, 104u8,
                            169u8, 213u8, 73u8, 159u8, 143u8, 246u8, 51u8, 97u8, 180u8, 8u8, 1u8,
                        ],
                    )
                }

                /// See [`Pallet::bb_attester_recall`].
                pub fn bb_attester_recall(
                    &self,
                    attester_id: types::bb_attester_recall::AttesterId,
                ) -> ::subxt::tx::Payload<types::BbAttesterRecall> {
                    ::subxt::tx::Payload::new_static(
                        "ServicerRegistry",
                        "bb_attester_recall",
                        types::BbAttesterRecall {
                            attester_id,
                        },
                        [
                            206u8, 216u8, 61u8, 85u8, 184u8, 172u8, 41u8, 53u8, 16u8, 73u8, 3u8,
                            81u8, 142u8, 227u8, 52u8, 124u8, 149u8, 111u8, 91u8, 71u8, 180u8, 10u8,
                            236u8, 106u8, 35u8, 141u8, 34u8, 217u8, 208u8, 57u8, 211u8, 225u8,
                        ],
                    )
                }

                /// See [`Pallet::sv_attester_bind`].
                pub fn sv_attester_bind(
                    &self,
                    peer_id: types::sv_attester_bind::PeerId,
                    attester_id: types::sv_attester_bind::AttesterId,
                ) -> ::subxt::tx::Payload<types::SvAttesterBind> {
                    ::subxt::tx::Payload::new_static(
                        "ServicerRegistry",
                        "sv_attester_bind",
                        types::SvAttesterBind {
                            peer_id,
                            attester_id,
                        },
                        [
                            93u8, 17u8, 46u8, 96u8, 13u8, 217u8, 31u8, 96u8, 137u8, 65u8, 140u8,
                            245u8, 242u8, 245u8, 66u8, 63u8, 97u8, 73u8, 4u8, 121u8, 90u8, 242u8,
                            9u8, 63u8, 128u8, 182u8, 114u8, 176u8, 93u8, 79u8, 25u8, 134u8,
                        ],
                    )
                }

                /// See [`Pallet::md_rep_increase`].
                pub fn md_rep_increase(
                    &self,
                    on: types::md_rep_increase::On,
                ) -> ::subxt::tx::Payload<types::MdRepIncrease> {
                    ::subxt::tx::Payload::new_static(
                        "ServicerRegistry",
                        "md_rep_increase",
                        types::MdRepIncrease {
                            on,
                        },
                        [
                            159u8, 220u8, 55u8, 39u8, 41u8, 44u8, 244u8, 21u8, 243u8, 127u8, 58u8,
                            85u8, 140u8, 66u8, 49u8, 74u8, 102u8, 130u8, 185u8, 92u8, 225u8, 171u8,
                            200u8, 218u8, 183u8, 84u8, 141u8, 156u8, 131u8, 211u8, 108u8, 182u8,
                        ],
                    )
                }

                /// See [`Pallet::md_rep_decrease`].
                pub fn md_rep_decrease(
                    &self,
                    on: types::md_rep_decrease::On,
                ) -> ::subxt::tx::Payload<types::MdRepDecrease> {
                    ::subxt::tx::Payload::new_static(
                        "ServicerRegistry",
                        "md_rep_decrease",
                        types::MdRepDecrease {
                            on,
                        },
                        [
                            120u8, 216u8, 128u8, 93u8, 252u8, 214u8, 108u8, 16u8, 42u8, 212u8,
                            23u8, 45u8, 31u8, 249u8, 3u8, 113u8, 70u8, 160u8, 187u8, 184u8, 50u8,
                            22u8, 75u8, 0u8, 129u8, 86u8, 220u8, 86u8, 182u8, 89u8, 204u8, 114u8,
                        ],
                    )
                }
            }
        }
        /// The `Event` enum of this pallet
        pub type Event = runtime_types::nagara_registry_servicers::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Big Brother's unbinded attestation device supplied
            pub struct BigBrotherAttesterSupplied {
                pub id: big_brother_attester_supplied::Id,
                pub bb: big_brother_attester_supplied::Bb,
            }
            pub mod big_brother_attester_supplied {
                use super::runtime_types;
                pub type Id = runtime_types::sp_core::ed25519::Public;
                pub type Bb = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for BigBrotherAttesterSupplied {
                const EVENT: &'static str = "BigBrotherAttesterSupplied";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Big Brother's unbinded attestation device recalled
            pub struct BigBrotherAttesterRecalled {
                pub id: big_brother_attester_recalled::Id,
                pub bb: big_brother_attester_recalled::Bb,
            }
            pub mod big_brother_attester_recalled {
                use super::runtime_types;
                pub type Id = runtime_types::sp_core::ed25519::Public;
                pub type Bb = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for BigBrotherAttesterRecalled {
                const EVENT: &'static str = "BigBrotherAttesterRecalled";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Servicer's reputation increased
            pub struct ServicerReputationIncreased {
                pub by: servicer_reputation_increased::By,
                pub on: servicer_reputation_increased::On,
                pub who: servicer_reputation_increased::Who,
            }
            pub mod servicer_reputation_increased {
                use super::runtime_types;
                pub type By = ::subxt::utils::AccountId32;
                pub type On = runtime_types::sp_core::ed25519::Public;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for ServicerReputationIncreased {
                const EVENT: &'static str = "ServicerReputationIncreased";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Servicer's reputation decreased
            pub struct ServicerReputationDecreased {
                pub by: servicer_reputation_decreased::By,
                pub on: servicer_reputation_decreased::On,
                pub who: servicer_reputation_decreased::Who,
            }
            pub mod servicer_reputation_decreased {
                use super::runtime_types;
                pub type By = ::subxt::utils::AccountId32;
                pub type On = runtime_types::sp_core::ed25519::Public;
                pub type Who = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for ServicerReputationDecreased {
                const EVENT: &'static str = "ServicerReputationDecreased";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Attester binded
            pub struct AttesterBinded {
                pub to: attester_binded::To,
                pub which: attester_binded::Which,
                pub peer_id: attester_binded::PeerId,
            }
            pub mod attester_binded {
                use super::runtime_types;
                pub type To = ::subxt::utils::AccountId32;
                pub type Which = runtime_types::sp_core::ed25519::Public;
                pub type PeerId = runtime_types::sp_core::ed25519::Public;
            }
            impl ::subxt::events::StaticEvent for AttesterBinded {
                const EVENT: &'static str = "AttesterBinded";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Attester unbinded (this shouldn't happen in normal condition)
            pub struct AttesterUnbinded {
                pub by: attester_unbinded::By,
                pub from: attester_unbinded::From,
            }
            pub mod attester_unbinded {
                use super::runtime_types;
                pub type By = ::core::option::Option<::subxt::utils::AccountId32>;
                pub type From = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for AttesterUnbinded {
                const EVENT: &'static str = "AttesterUnbinded";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Mediator added
            pub struct MediatorAdded {
                pub who: mediator_added::Who,
                pub by: mediator_added::By,
            }
            pub mod mediator_added {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type By = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for MediatorAdded {
                const EVENT: &'static str = "MediatorAdded";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Mediator added
            pub struct MediatorRemoved {
                pub who: mediator_removed::Who,
                pub by: mediator_removed::By,
            }
            pub mod mediator_removed {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type By = ::subxt::utils::AccountId32;
            }
            impl ::subxt::events::StaticEvent for MediatorRemoved {
                const EVENT: &'static str = "MediatorRemoved";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Servicer registration fee paid
            pub struct ServicerRegistrationFeePaid {
                pub who: servicer_registration_fee_paid::Who,
                pub amount: servicer_registration_fee_paid::Amount,
            }
            pub mod servicer_registration_fee_paid {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for ServicerRegistrationFeePaid {
                const EVENT: &'static str = "ServicerRegistrationFeePaid";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Servicer's balance held for binding
            pub struct ServicerBalanceHeldForBinding {
                pub who: servicer_balance_held_for_binding::Who,
                pub amount: servicer_balance_held_for_binding::Amount,
            }
            pub mod servicer_balance_held_for_binding {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for ServicerBalanceHeldForBinding {
                const EVENT: &'static str = "ServicerBalanceHeldForBinding";
                const PALLET: &'static str = "ServicerRegistry";
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            /// Servicer's balance held released
            pub struct ServicerBalanceHeldForBindingReleased {
                pub who: servicer_balance_held_for_binding_released::Who,
                pub amount: servicer_balance_held_for_binding_released::Amount,
            }
            pub mod servicer_balance_held_for_binding_released {
                use super::runtime_types;
                pub type Who = ::subxt::utils::AccountId32;
                pub type Amount = ::core::primitive::u128;
            }
            impl ::subxt::events::StaticEvent for ServicerBalanceHeldForBindingReleased {
                const EVENT: &'static str = "ServicerBalanceHeldForBindingReleased";
                const PALLET: &'static str = "ServicerRegistry";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub mod types {
                use super::runtime_types;
                pub mod attesters {
                    use super::runtime_types;
                    pub type Attesters =
                        runtime_types::nagara_registry_servicers::pallet::RemoteAttestationDevice<
                            ::subxt::utils::AccountId32,
                        >;
                    pub type Param0 = runtime_types::sp_core::ed25519::Public;
                }
                pub mod mediators {
                    use super::runtime_types;
                    pub type Mediators =
                        runtime_types::bounded_collections::bounded_btree_set::BoundedBTreeSet<
                            ::subxt::utils::AccountId32,
                        >;
                }
                pub mod servicers {
                    use super::runtime_types;
                    pub type Servicers =
                        runtime_types::nagara_registry_servicers::pallet::ServicerInformation;
                    pub type Param0 = ::subxt::utils::AccountId32;
                }
            }
            pub struct StorageApi;
            impl StorageApi {
                pub fn attesters_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::attesters::Attesters,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ServicerRegistry",
                        "Attesters",
                        vec![],
                        [
                            212u8, 167u8, 97u8, 226u8, 221u8, 83u8, 216u8, 76u8, 72u8, 126u8,
                            219u8, 10u8, 129u8, 220u8, 167u8, 109u8, 179u8, 169u8, 75u8, 29u8,
                            210u8, 180u8, 226u8, 115u8, 249u8, 91u8, 174u8, 122u8, 18u8, 119u8,
                            239u8, 250u8,
                        ],
                    )
                }

                pub fn attesters(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::attesters::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::attesters::Attesters,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ServicerRegistry",
                        "Attesters",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            212u8, 167u8, 97u8, 226u8, 221u8, 83u8, 216u8, 76u8, 72u8, 126u8,
                            219u8, 10u8, 129u8, 220u8, 167u8, 109u8, 179u8, 169u8, 75u8, 29u8,
                            210u8, 180u8, 226u8, 115u8, 249u8, 91u8, 174u8, 122u8, 18u8, 119u8,
                            239u8, 250u8,
                        ],
                    )
                }

                pub fn mediators(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::mediators::Mediators,
                    ::subxt::storage::address::Yes,
                    ::subxt::storage::address::Yes,
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ServicerRegistry",
                        "Mediators",
                        vec![],
                        [
                            106u8, 65u8, 151u8, 110u8, 114u8, 125u8, 162u8, 115u8, 230u8, 96u8,
                            138u8, 255u8, 127u8, 105u8, 76u8, 140u8, 93u8, 211u8, 96u8, 205u8,
                            56u8, 129u8, 66u8, 183u8, 214u8, 56u8, 170u8, 52u8, 251u8, 62u8, 37u8,
                            5u8,
                        ],
                    )
                }

                pub fn servicers_iter(
                    &self,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::servicers::Servicers,
                    (),
                    (),
                    ::subxt::storage::address::Yes,
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ServicerRegistry",
                        "Servicers",
                        vec![],
                        [
                            162u8, 239u8, 45u8, 71u8, 156u8, 30u8, 204u8, 136u8, 217u8, 255u8,
                            227u8, 200u8, 249u8, 140u8, 54u8, 48u8, 229u8, 206u8, 238u8, 233u8,
                            219u8, 107u8, 135u8, 60u8, 188u8, 121u8, 252u8, 220u8, 238u8, 189u8,
                            44u8, 211u8,
                        ],
                    )
                }

                pub fn servicers(
                    &self,
                    _0: impl ::std::borrow::Borrow<types::servicers::Param0>,
                ) -> ::subxt::storage::address::Address<
                    ::subxt::storage::address::StaticStorageMapKey,
                    types::servicers::Servicers,
                    ::subxt::storage::address::Yes,
                    (),
                    (),
                > {
                    ::subxt::storage::address::Address::new_static(
                        "ServicerRegistry",
                        "Servicers",
                        vec![::subxt::storage::address::make_static_storage_map_key(
                            _0.borrow(),
                        )],
                        [
                            162u8, 239u8, 45u8, 71u8, 156u8, 30u8, 204u8, 136u8, 217u8, 255u8,
                            227u8, 200u8, 249u8, 140u8, 54u8, 48u8, 229u8, 206u8, 238u8, 233u8,
                            219u8, 107u8, 135u8, 60u8, 188u8, 121u8, 252u8, 220u8, 238u8, 189u8,
                            44u8, 211u8,
                        ],
                    )
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                /// Attester binding deposit amount, once per attester
                pub fn binding_deposit_amount(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "ServicerRegistry",
                        "BindingDepositAmount",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// Servicer registry fee (this to prevent cheap reputation
                /// reset) paid
                /// only once per existence
                pub fn registration_fee_amount(
                    &self,
                ) -> ::subxt::constants::Address<::core::primitive::u128> {
                    ::subxt::constants::Address::new_static(
                        "ServicerRegistry",
                        "RegistrationFeeAmount",
                        [
                            84u8, 157u8, 140u8, 4u8, 93u8, 57u8, 29u8, 133u8, 105u8, 200u8, 214u8,
                            27u8, 144u8, 208u8, 218u8, 160u8, 130u8, 109u8, 101u8, 54u8, 210u8,
                            136u8, 71u8, 63u8, 49u8, 237u8, 234u8, 15u8, 178u8, 98u8, 148u8, 156u8,
                        ],
                    )
                }

                /// Maximum mediator for servicers, mediator is a role that can
                /// be
                /// filled by smart contracts to mediate services between the
                /// chain and dApps
                pub fn max_mediators(&self) -> ::subxt::constants::Address<::core::primitive::u32> {
                    ::subxt::constants::Address::new_static(
                        "ServicerRegistry",
                        "MaxMediators",
                        [
                            98u8, 252u8, 116u8, 72u8, 26u8, 180u8, 225u8, 83u8, 200u8, 157u8,
                            125u8, 151u8, 53u8, 76u8, 168u8, 26u8, 10u8, 9u8, 98u8, 68u8, 9u8,
                            178u8, 197u8, 113u8, 31u8, 79u8, 200u8, 90u8, 203u8, 100u8, 41u8,
                            145u8,
                        ],
                    )
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod bounded_collections {
            use super::runtime_types;
            pub mod bounded_btree_set {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BoundedBTreeSet<_0>(pub ::std::vec::Vec<_0>);
            }
            pub mod bounded_vec {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
            pub mod weak_bounded_vec {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod dispatch {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DispatchInfo {
                    pub weight: runtime_types::sp_weights::weight_v2::Weight,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::dispatch::Pays,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum RawOrigin<_0> {
                    #[codec(index = 0)]
                    Root,
                    #[codec(index = 1)]
                    Signed(_0),
                    #[codec(index = 2)]
                    None,
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            ::subxt::ext::codec::Decode,
                            ::subxt::ext::codec::Encode,
                            ::subxt::ext::scale_decode::DecodeAsType,
                            ::subxt::ext::scale_encode::EncodeAsType,
                            Debug
                        )]
                        # [codec (crate = :: subxt :: ext :: codec)]
                        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonZeroSender;
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BlockWeights {
                    pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                    pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct WeightsPerClass {
                    pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                    pub max_extrinsic:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub max_total:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                    pub reserved:
                        ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::remark`].
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::set_heap_pages`].
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 2)]
                    /// See [`Pallet::set_code`].
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    /// See [`Pallet::set_code_without_checks`].
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    /// See [`Pallet::set_storage`].
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 5)]
                    /// See [`Pallet::kill_storage`].
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 6)]
                    /// See [`Pallet::kill_prefix`].
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    /// See [`Pallet::remark_with_event`].
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Error for the System pallet
                pub enum Error {
                    #[codec(index = 0)]
                    /// The name of specification does not match between the
                    /// current runtime
                    /// and the new runtime.
                    InvalidSpecName,
                    #[codec(index = 1)]
                    /// The specification version is not allowed to decrease
                    /// between the current runtime
                    /// and the new runtime.
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    /// Failed to extract the runtime version from the new
                    /// runtime.
                    ///
                    /// Either calling `Core_version` or decoding
                    /// `RuntimeVersion` failed.
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    /// Suicide called when the account has non-default
                    /// composite data.
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    /// There is a non-zero reference count preventing the
                    /// account from being purged.
                    NonZeroRefCount,
                    #[codec(index = 5)]
                    /// The origin filter prevent the call to be dispatched.
                    CallFiltered,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Event for the System pallet.
                pub enum Event {
                    #[codec(index = 0)]
                    /// An extrinsic completed successfully.
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    /// An extrinsic failed.
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    /// `:code` was updated.
                    CodeUpdated,
                    #[codec(index = 3)]
                    /// A new account was created.
                    NewAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    /// An account was reaped.
                    KilledAccount {
                        account: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    /// On on-chain remark happened.
                    Remarked {
                        sender: ::subxt::utils::AccountId32,
                        hash: ::subxt::utils::H256,
                    },
                }
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: ::core::primitive::u32,
                pub providers: ::core::primitive::u32,
                pub sufficients: ::core::primitive::u32,
                pub data: _1,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
        }
        pub mod nagara_core_runtime {
            use super::runtime_types;
            pub mod opaque {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct SessionKeys {
                    pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
                }
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_support::dispatch::RawOrigin<::subxt::utils::AccountId32>,
                ),
                #[codec(index = 1)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Runtime;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeCall {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 2)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 3)]
                ValidatorSet(runtime_types::substrate_validator_set::pallet::Call),
                #[codec(index = 4)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 6)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 8)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 9)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 11)]
                Contracts(runtime_types::pallet_contracts::pallet::Call),
                #[codec(index = 12)]
                Assets(runtime_types::pallet_assets::pallet::Call),
                #[codec(index = 13)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 14)]
                Identity(runtime_types::pallet_identity::pallet::Call),
                #[codec(index = 15)]
                BigBrotherCouncil(runtime_types::nagara_council_bigbrothers::pallet::Call),
                #[codec(index = 16)]
                ServicerRegistry(runtime_types::nagara_registry_servicers::pallet::Call),
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeError {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Error),
                #[codec(index = 2)]
                Balances(runtime_types::pallet_balances::pallet::Error),
                #[codec(index = 3)]
                ValidatorSet(runtime_types::substrate_validator_set::pallet::Error),
                #[codec(index = 4)]
                Session(runtime_types::pallet_session::pallet::Error),
                #[codec(index = 6)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Error),
                #[codec(index = 8)]
                Sudo(runtime_types::pallet_sudo::pallet::Error),
                #[codec(index = 9)]
                Utility(runtime_types::pallet_utility::pallet::Error),
                #[codec(index = 11)]
                Contracts(runtime_types::pallet_contracts::pallet::Error),
                #[codec(index = 12)]
                Assets(runtime_types::pallet_assets::pallet::Error),
                #[codec(index = 13)]
                Multisig(runtime_types::pallet_multisig::pallet::Error),
                #[codec(index = 14)]
                Identity(runtime_types::pallet_identity::pallet::Error),
                #[codec(index = 15)]
                BigBrotherCouncil(runtime_types::nagara_council_bigbrothers::pallet::Error),
                #[codec(index = 16)]
                ServicerRegistry(runtime_types::nagara_registry_servicers::pallet::Error),
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeEvent {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 2)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 3)]
                ValidatorSet(runtime_types::substrate_validator_set::pallet::Event),
                #[codec(index = 4)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 6)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 7)]
                TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
                #[codec(index = 8)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 9)]
                Utility(runtime_types::pallet_utility::pallet::Event),
                #[codec(index = 11)]
                Contracts(runtime_types::pallet_contracts::pallet::Event),
                #[codec(index = 12)]
                Assets(runtime_types::pallet_assets::pallet::Event),
                #[codec(index = 13)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                #[codec(index = 14)]
                Identity(runtime_types::pallet_identity::pallet::Event),
                #[codec(index = 15)]
                BigBrotherCouncil(runtime_types::nagara_council_bigbrothers::pallet::Event),
                #[codec(index = 16)]
                ServicerRegistry(runtime_types::nagara_registry_servicers::pallet::Event),
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeFreezeReason {}
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RuntimeHoldReason {
                #[codec(index = 15)]
                BigBrotherCouncil(runtime_types::nagara_council_bigbrothers::pallet::HoldReason),
                #[codec(index = 16)]
                ServicerRegistry(runtime_types::nagara_registry_servicers::pallet::HoldReason),
            }
        }
        pub mod nagara_council_bigbrothers {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Dispatchable functions.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::su_elder_replace`].
                    su_elder_replace {
                        new_elder: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::se_membership_add`].
                    se_membership_add {
                        new_council_member: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    /// See [`Pallet::se_membership_remove`].
                    se_membership_remove {
                        council_member: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    /// See [`Pallet::se_currency_mint_into`].
                    se_currency_mint_into {
                        dest: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    /// See [`Pallet::se_currency_burn`].
                    se_currency_burn { amount: ::core::primitive::u128 },
                    #[codec(index = 5)]
                    /// See [`Pallet::se_currency_burn_all`].
                    se_currency_burn_all,
                    #[codec(index = 6)]
                    /// See [`Pallet::cm_proposal_new`].
                    cm_proposal_new {
                        new_multiplier: ::core::option::Option<::core::primitive::u64>,
                        new_divider: ::core::option::Option<::core::primitive::u64>,
                        new_minimum_fee: ::core::option::Option<::core::primitive::u128>,
                    },
                    #[codec(index = 7)]
                    /// See [`Pallet::cm_proposal_vote`].
                    cm_proposal_vote {
                        is_approving: ::core::primitive::bool,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Fatal error, chain storage compromised
                    FatalError,
                    #[codec(index = 1)]
                    /// Elder is undefined
                    UndefinedElder,
                    #[codec(index = 2)]
                    /// Restricted call, only for Elder Brother (Sudo)
                    SudoOrElderOnly,
                    #[codec(index = 3)]
                    /// Council member only
                    CouncilMemberOnly,
                    #[codec(index = 4)]
                    /// Council membership full
                    CouncilMembershipFull,
                    #[codec(index = 5)]
                    /// Account has no verified legality
                    AccountIsNotVerifiedLegally,
                    #[codec(index = 6)]
                    /// Account has no legal name
                    AccountHasNoLegalName,
                    #[codec(index = 7)]
                    /// Account already a member
                    AccountAlreadyAMember,
                    #[codec(index = 8)]
                    /// Account is not a member
                    AccountIsNotAMember,
                    #[codec(index = 9)]
                    /// Account already an Elder
                    AccountAlreadyAnElder,
                    #[codec(index = 10)]
                    /// No proposal exists
                    NoProposalExists,
                    #[codec(index = 11)]
                    /// Incorrect proposal
                    IncorrectProposal,
                    #[codec(index = 12)]
                    /// Vote already counted
                    VoteAlreadyCounted,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// Big Brother Registered/Added
                    BigBrotherAdded {
                        who: ::subxt::utils::AccountId32,
                        by: ::core::option::Option<::subxt::utils::AccountId32>,
                        hold: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    /// Big Brother Unregistered/Removed
                    BigBrotherRemoved {
                        who: ::subxt::utils::AccountId32,
                        by: ::core::option::Option<::subxt::utils::AccountId32>,
                        release: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    /// Elder ascended
                    ElderAscended { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 3)]
                    /// Elder descended
                    ElderDescended { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 4)]
                    /// Token circulation increased
                    CirculationIncreased {
                        increase: ::core::primitive::u128,
                        by: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 5)]
                    /// Token circulation decreased
                    CirculationDecreased {
                        decrease: ::core::primitive::u128,
                        by: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 6)]
                    /// Transaction Fee parameters changed
                    TxFeeParametersChange {
                        old: runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<
                            ::core::primitive::u128,
                        >,
                        new: runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<
                            ::core::primitive::u128,
                        >,
                    },
                    #[codec(index = 7)]
                    /// Transaction fee parameters proposal rejected
                    TxFeeParametersRejected {
                        rejected:
                            runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<
                                ::core::primitive::u128,
                            >,
                        by: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 8)]
                    /// New transaction fee parameters proposal
                    TxFeeParametersChangeProposed {
                        proposal:
                            runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<
                                ::core::primitive::u128,
                            >,
                        by: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 9)]
                    /// Transaction fee parameters proposal vote count
                    TxFeeParametersChangeVoted {
                        by: ::subxt::utils::AccountId32,
                        remaining_count: ::core::primitive::u32,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum HoldReason {
                    #[codec(index = 0)]
                    CouncilMembership,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransactionFeeChangeProposal<_0, _1, _2> {
                    pub initiator: _0,
                    pub initiated_at: _2,
                    pub new_parameters:
                        runtime_types::nagara_council_bigbrothers::pallet::TransactionFeeInfo<_1>,
                    pub required_vote_count: ::core::primitive::u32,
                    pub approvers: ::std::vec::Vec<_0>,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct TransactionFeeInfo<_0> {
                    pub weight_to_fee_divider: ::core::primitive::u64,
                    pub weight_to_fee_multiplier: ::core::primitive::u64,
                    pub minimum_transaction_fee: _0,
                }
            }
        }
        pub mod nagara_registry_servicers {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Dispatchable functions.
                pub enum Call {
                    # [codec (index = 0)] # [doc = "See [`Pallet::bb_attester_supply`]."] bb_attester_supply { supply_args : runtime_types :: nagara_registry_servicers :: pallet :: RemoteAttestationDeviceSupplyArgs , } , # [codec (index = 1)] # [doc = "See [`Pallet::bb_mediator_add`]."] bb_mediator_add { who : :: subxt :: utils :: AccountId32 , } , # [codec (index = 2)] # [doc = "See [`Pallet::bb_mediator_remove`]."] bb_mediator_remove { who : :: subxt :: utils :: AccountId32 , } , # [codec (index = 3)] # [doc = "See [`Pallet::bb_attester_recall`]."] bb_attester_recall { attester_id : runtime_types :: sp_core :: ed25519 :: Public , } , # [codec (index = 4)] # [doc = "See [`Pallet::sv_attester_bind`]."] sv_attester_bind { peer_id : runtime_types :: sp_core :: ed25519 :: Public , attester_id : runtime_types :: sp_core :: ed25519 :: Public , } , # [codec (index = 5)] # [doc = "See [`Pallet::md_rep_increase`]."] md_rep_increase { on : runtime_types :: sp_core :: ed25519 :: Public , } , # [codec (index = 6)] # [doc = "See [`Pallet::md_rep_decrease`]."] md_rep_decrease { on : runtime_types :: sp_core :: ed25519 :: Public , } , }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Fatal error, chain storage compromised
                    FatalError,
                    #[codec(index = 1)]
                    /// Restricted call (sudo or big brothers only)
                    RestrictedCall,
                    #[codec(index = 2)]
                    /// Attester already binded
                    AttesterAlreadyBinded,
                    #[codec(index = 3)]
                    /// Attester doesn't exist
                    AttesterDoesntExist,
                    #[codec(index = 4)]
                    /// Attester already supplied
                    AttesterAlreadySupplied,
                    #[codec(index = 5)]
                    /// Attester is binded to no one
                    AttesterIsUnbinded,
                    #[codec(index = 6)]
                    /// Servicer cannot pay registration fee
                    ServicerCannotPayRegistrationFee,
                    #[codec(index = 7)]
                    /// Mediator already registered
                    MediatorAlreadyRegistered,
                    #[codec(index = 8)]
                    /// Mediator is not registered
                    MediatorNotFound,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// Big Brother's unbinded attestation device supplied
                    BigBrotherAttesterSupplied {
                        id: runtime_types::sp_core::ed25519::Public,
                        bb: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    /// Big Brother's unbinded attestation device recalled
                    BigBrotherAttesterRecalled {
                        id: runtime_types::sp_core::ed25519::Public,
                        bb: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    /// Servicer's reputation increased
                    ServicerReputationIncreased {
                        by: ::subxt::utils::AccountId32,
                        on: runtime_types::sp_core::ed25519::Public,
                        who: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    /// Servicer's reputation decreased
                    ServicerReputationDecreased {
                        by: ::subxt::utils::AccountId32,
                        on: runtime_types::sp_core::ed25519::Public,
                        who: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 4)]
                    /// Attester binded
                    AttesterBinded {
                        to: ::subxt::utils::AccountId32,
                        which: runtime_types::sp_core::ed25519::Public,
                        peer_id: runtime_types::sp_core::ed25519::Public,
                    },
                    #[codec(index = 5)]
                    /// Attester unbinded (this shouldn't happen in normal
                    /// condition)
                    AttesterUnbinded {
                        by: ::core::option::Option<::subxt::utils::AccountId32>,
                        from: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 6)]
                    /// Mediator added
                    MediatorAdded {
                        who: ::subxt::utils::AccountId32,
                        by: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    /// Mediator added
                    MediatorRemoved {
                        who: ::subxt::utils::AccountId32,
                        by: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 8)]
                    /// Servicer registration fee paid
                    ServicerRegistrationFeePaid {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    /// Servicer's balance held for binding
                    ServicerBalanceHeldForBinding {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    /// Servicer's balance held released
                    ServicerBalanceHeldForBindingReleased {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum HoldReason {
                    #[codec(index = 0)]
                    Binding,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoteAttestationDevice<_0> {
                    pub big_brother: _0,
                    pub serial_number: ::core::primitive::u32,
                    pub guid: [::core::primitive::u8; 16usize],
                    pub binder: ::core::option::Option<_0>,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RemoteAttestationDeviceSupplyArgs {
                    pub id: runtime_types::sp_core::ed25519::Public,
                    pub guid: [::core::primitive::u8; 16usize],
                    pub serial_number: ::core::primitive::u32,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ServicerInformation {
                    pub rep_positive: ::core::primitive::u32,
                    pub rep_negative: ::core::primitive::u32,
                    pub bindings: ::subxt::utils::KeyedVec<
                        runtime_types::sp_core::ed25519::Public,
                        runtime_types::sp_core::ed25519::Public,
                    >,
                }
            }
        }
        pub mod pallet_assets {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::create`].
                    create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::force_create`].
                    force_create {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        is_sufficient: ::core::primitive::bool,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    /// See [`Pallet::start_destroy`].
                    start_destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    /// See [`Pallet::destroy_accounts`].
                    destroy_accounts {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    /// See [`Pallet::destroy_approvals`].
                    destroy_approvals {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    /// See [`Pallet::finish_destroy`].
                    finish_destroy {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    /// See [`Pallet::mint`].
                    mint {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    /// See [`Pallet::burn`].
                    burn {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    /// See [`Pallet::transfer`].
                    transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    /// See [`Pallet::transfer_keep_alive`].
                    transfer_keep_alive {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    /// See [`Pallet::force_transfer`].
                    force_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    /// See [`Pallet::freeze`].
                    freeze {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 12)]
                    /// See [`Pallet::thaw`].
                    thaw {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 13)]
                    /// See [`Pallet::freeze_asset`].
                    freeze_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 14)]
                    /// See [`Pallet::thaw_asset`].
                    thaw_asset {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 15)]
                    /// See [`Pallet::transfer_ownership`].
                    transfer_ownership {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 16)]
                    /// See [`Pallet::set_team`].
                    set_team {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 17)]
                    /// See [`Pallet::set_metadata`].
                    set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                    },
                    #[codec(index = 18)]
                    /// See [`Pallet::clear_metadata`].
                    clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 19)]
                    /// See [`Pallet::force_set_metadata`].
                    force_set_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 20)]
                    /// See [`Pallet::force_clear_metadata`].
                    force_clear_metadata {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 21)]
                    /// See [`Pallet::force_asset_status`].
                    force_asset_status {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        issuer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        admin: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        freezer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        min_balance: ::core::primitive::u128,
                        is_sufficient: ::core::primitive::bool,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 22)]
                    /// See [`Pallet::approve_transfer`].
                    approve_transfer {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 23)]
                    /// See [`Pallet::cancel_approval`].
                    cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 24)]
                    /// See [`Pallet::force_cancel_approval`].
                    force_cancel_approval {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 25)]
                    /// See [`Pallet::transfer_approved`].
                    transfer_approved {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        owner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        destination: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 26)]
                    /// See [`Pallet::touch`].
                    touch {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                    },
                    #[codec(index = 27)]
                    /// See [`Pallet::refund`].
                    refund {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        allow_burn: ::core::primitive::bool,
                    },
                    #[codec(index = 28)]
                    /// See [`Pallet::set_min_balance`].
                    set_min_balance {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 29)]
                    /// See [`Pallet::touch_other`].
                    touch_other {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 30)]
                    /// See [`Pallet::refund_other`].
                    refund_other {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 31)]
                    /// See [`Pallet::block`].
                    block {
                        #[codec(compact)]
                        id: ::core::primitive::u32,
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Account balance must be greater than or equal to the
                    /// transfer amount.
                    BalanceLow,
                    #[codec(index = 1)]
                    /// The account to alter does not exist.
                    NoAccount,
                    #[codec(index = 2)]
                    /// The signing account has no permission to do the
                    /// operation.
                    NoPermission,
                    #[codec(index = 3)]
                    /// The given asset ID is unknown.
                    Unknown,
                    #[codec(index = 4)]
                    /// The origin account is frozen.
                    Frozen,
                    #[codec(index = 5)]
                    /// The asset ID is already taken.
                    InUse,
                    #[codec(index = 6)]
                    /// Invalid witness data given.
                    BadWitness,
                    #[codec(index = 7)]
                    /// Minimum balance should be non-zero.
                    MinBalanceZero,
                    #[codec(index = 8)]
                    /// Unable to increment the consumer reference counters on
                    /// the account. Either no provider
                    /// reference exists to allow a non-zero balance of a
                    /// non-self-sufficient asset, or one
                    /// fewer then the maximum number of consumers has been
                    /// reached.
                    UnavailableConsumer,
                    #[codec(index = 9)]
                    /// Invalid metadata given.
                    BadMetadata,
                    #[codec(index = 10)]
                    /// No approval exists that would allow the transfer.
                    Unapproved,
                    #[codec(index = 11)]
                    /// The source account would not survive the transfer and it
                    /// needs to stay alive.
                    WouldDie,
                    #[codec(index = 12)]
                    /// The asset-account already exists.
                    AlreadyExists,
                    #[codec(index = 13)]
                    /// The asset-account doesn't have an associated deposit.
                    NoDeposit,
                    #[codec(index = 14)]
                    /// The operation would result in funds being burned.
                    WouldBurn,
                    #[codec(index = 15)]
                    /// The asset is a live asset and is actively being used.
                    /// Usually emit for operations such
                    /// as `start_destroy` which require the asset to be in a
                    /// destroying state.
                    LiveAsset,
                    #[codec(index = 16)]
                    /// The asset is not live, and likely being destroyed.
                    AssetNotLive,
                    #[codec(index = 17)]
                    /// The asset status is not the expected status.
                    IncorrectStatus,
                    #[codec(index = 18)]
                    /// The asset should be frozen before the given operation.
                    NotFrozen,
                    #[codec(index = 19)]
                    /// Callback action resulted in error
                    CallbackFailed,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// Some asset class was created.
                    Created {
                        asset_id: ::core::primitive::u32,
                        creator: ::subxt::utils::AccountId32,
                        owner: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    /// Some assets were issued.
                    Issued {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    /// Some assets were transferred.
                    Transferred {
                        asset_id: ::core::primitive::u32,
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    /// Some assets were destroyed.
                    Burned {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::utils::AccountId32,
                        balance: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    /// The management team changed.
                    TeamChanged {
                        asset_id: ::core::primitive::u32,
                        issuer: ::subxt::utils::AccountId32,
                        admin: ::subxt::utils::AccountId32,
                        freezer: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 5)]
                    /// The owner changed.
                    OwnerChanged {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 6)]
                    /// Some account `who` was frozen.
                    Frozen {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    /// Some account `who` was thawed.
                    Thawed {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 8)]
                    /// Some asset `asset_id` was frozen.
                    AssetFrozen { asset_id: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    /// Some asset `asset_id` was thawed.
                    AssetThawed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    /// Accounts were destroyed for given asset.
                    AccountsDestroyed {
                        asset_id: ::core::primitive::u32,
                        accounts_destroyed: ::core::primitive::u32,
                        accounts_remaining: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    /// Approvals were destroyed for given asset.
                    ApprovalsDestroyed {
                        asset_id: ::core::primitive::u32,
                        approvals_destroyed: ::core::primitive::u32,
                        approvals_remaining: ::core::primitive::u32,
                    },
                    #[codec(index = 12)]
                    /// An asset class is in the process of being destroyed.
                    DestructionStarted { asset_id: ::core::primitive::u32 },
                    #[codec(index = 13)]
                    /// An asset class was destroyed.
                    Destroyed { asset_id: ::core::primitive::u32 },
                    #[codec(index = 14)]
                    /// Some asset class was force-created.
                    ForceCreated {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 15)]
                    /// New metadata has been set for an asset.
                    MetadataSet {
                        asset_id: ::core::primitive::u32,
                        name: ::std::vec::Vec<::core::primitive::u8>,
                        symbol: ::std::vec::Vec<::core::primitive::u8>,
                        decimals: ::core::primitive::u8,
                        is_frozen: ::core::primitive::bool,
                    },
                    #[codec(index = 16)]
                    /// Metadata has been cleared for an asset.
                    MetadataCleared { asset_id: ::core::primitive::u32 },
                    #[codec(index = 17)]
                    /// (Additional) funds have been approved for transfer to a
                    /// destination account.
                    ApprovedTransfer {
                        asset_id: ::core::primitive::u32,
                        source: ::subxt::utils::AccountId32,
                        delegate: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    /// An approval for account `delegate` was cancelled by
                    /// `owner`.
                    ApprovalCancelled {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::utils::AccountId32,
                        delegate: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 19)]
                    /// An `amount` was transferred in its entirety from `owner`
                    /// to `destination` by
                    /// the approved `delegate`.
                    TransferredApproved {
                        asset_id: ::core::primitive::u32,
                        owner: ::subxt::utils::AccountId32,
                        delegate: ::subxt::utils::AccountId32,
                        destination: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    /// An asset has had its attributes changed by the `Force`
                    /// origin.
                    AssetStatusChanged { asset_id: ::core::primitive::u32 },
                    #[codec(index = 21)]
                    /// The min_balance of an asset has been updated by the
                    /// asset owner.
                    AssetMinBalanceChanged {
                        asset_id: ::core::primitive::u32,
                        new_min_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 22)]
                    /// Some account `who` was created with a deposit from
                    /// `depositor`.
                    Touched {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::utils::AccountId32,
                        depositor: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 23)]
                    /// Some account `who` was blocked.
                    Blocked {
                        asset_id: ::core::primitive::u32,
                        who: ::subxt::utils::AccountId32,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum AccountStatus {
                    #[codec(index = 0)]
                    Liquid,
                    #[codec(index = 1)]
                    Frozen,
                    #[codec(index = 2)]
                    Blocked,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Approval<_0, _1> {
                    pub amount: _0,
                    pub deposit: _1,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AssetAccount<_0, _1, _2, _3> {
                    pub balance: _0,
                    pub status: runtime_types::pallet_assets::types::AccountStatus,
                    pub reason: runtime_types::pallet_assets::types::ExistenceReason<_0, _3>,
                    pub extra: _2,
                    #[codec(skip)]
                    pub __ignore: ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AssetDetails<_0, _1, _2> {
                    pub owner: _1,
                    pub issuer: _1,
                    pub admin: _1,
                    pub freezer: _1,
                    pub supply: _0,
                    pub deposit: _2,
                    pub min_balance: _0,
                    pub is_sufficient: ::core::primitive::bool,
                    pub accounts: ::core::primitive::u32,
                    pub sufficients: ::core::primitive::u32,
                    pub approvals: ::core::primitive::u32,
                    pub status: runtime_types::pallet_assets::types::AssetStatus,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AssetMetadata<_0, _1> {
                    pub deposit: _0,
                    pub name: _1,
                    pub symbol: _1,
                    pub decimals: ::core::primitive::u8,
                    pub is_frozen: ::core::primitive::bool,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum AssetStatus {
                    #[codec(index = 0)]
                    Live,
                    #[codec(index = 1)]
                    Frozen,
                    #[codec(index = 2)]
                    Destroying,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum ExistenceReason<_0, _1> {
                    #[codec(index = 0)]
                    Consumer,
                    #[codec(index = 1)]
                    Sufficient,
                    #[codec(index = 2)]
                    DepositHeld(_0),
                    #[codec(index = 3)]
                    DepositRefunded,
                    #[codec(index = 4)]
                    DepositFrom(_1, _0),
                }
            }
        }
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::transfer_allow_death`].
                    transfer_allow_death {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::set_balance_deprecated`].
                    set_balance_deprecated {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        old_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    /// See [`Pallet::force_transfer`].
                    force_transfer {
                        source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    /// See [`Pallet::transfer_keep_alive`].
                    transfer_keep_alive {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    /// See [`Pallet::transfer_all`].
                    transfer_all {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    /// See [`Pallet::force_unreserve`].
                    force_unreserve {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    /// See [`Pallet::upgrade_accounts`].
                    upgrade_accounts {
                        who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 7)]
                    /// See [`Pallet::transfer`].
                    transfer {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    /// See [`Pallet::force_set_balance`].
                    force_set_balance {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Vesting balance too high to send value.
                    VestingBalance,
                    #[codec(index = 1)]
                    /// Account liquidity restrictions prevent withdrawal.
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    /// Balance too low to send value.
                    InsufficientBalance,
                    #[codec(index = 3)]
                    /// Value too low to create account due to existential
                    /// deposit.
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    /// Transfer/payment would kill account.
                    Expendability,
                    #[codec(index = 5)]
                    /// A vesting schedule already exists for this account.
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    /// Beneficiary account must pre-exist.
                    DeadAccount,
                    #[codec(index = 7)]
                    /// Number of named reserves exceed `MaxReserves`.
                    TooManyReserves,
                    #[codec(index = 8)]
                    /// Number of holds exceed `MaxHolds`.
                    TooManyHolds,
                    #[codec(index = 9)]
                    /// Number of freezes exceed `MaxFreezes`.
                    TooManyFreezes,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// An account was created with some free balance.
                    Endowed {
                        account: ::subxt::utils::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    /// An account was removed whose balance was non-zero but
                    /// below ExistentialDeposit,
                    /// resulting in an outright loss.
                    DustLost {
                        account: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    /// Transfer succeeded.
                    Transfer {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    /// A balance was set by root.
                    BalanceSet {
                        who: ::subxt::utils::AccountId32,
                        free: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    /// Some balance was reserved (moved from free to reserved).
                    Reserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    /// Some balance was unreserved (moved from reserved to
                    /// free).
                    Unreserved {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    /// Some balance was moved from the reserve of the first
                    /// account to the second account.
                    /// Final argument indicates the destination balance type.
                    ReserveRepatriated {
                        from: ::subxt::utils::AccountId32,
                        to: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    /// Some amount was deposited (e.g. for transaction fees).
                    Deposit {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    /// Some amount was withdrawn from the account (e.g. for
                    /// transaction fees).
                    Withdraw {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    /// Some amount was removed from the account (e.g. for
                    /// misbehavior).
                    Slashed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 10)]
                    /// Some amount was minted into an account.
                    Minted {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 11)]
                    /// Some amount was burned from an account.
                    Burned {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 12)]
                    /// Some amount was suspended from an account (it can be
                    /// restored later).
                    Suspended {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 13)]
                    /// Some amount was restored into an account.
                    Restored {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 14)]
                    /// An account was upgraded.
                    Upgraded { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 15)]
                    /// Total issuance was increased by `amount`, creating a
                    /// credit to be balanced.
                    Issued { amount: ::core::primitive::u128 },
                    #[codec(index = 16)]
                    /// Total issuance was decreased by `amount`, creating a
                    /// debt to be balanced.
                    Rescinded { amount: ::core::primitive::u128 },
                    #[codec(index = 17)]
                    /// Some balance was locked.
                    Locked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 18)]
                    /// Some balance was unlocked.
                    Unlocked {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 19)]
                    /// Some balance was frozen.
                    Frozen {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    /// Some balance was thawed.
                    Thawed {
                        who: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub frozen: _0,
                    pub flags: runtime_types::pallet_balances::types::ExtraFlags,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::pallet_balances::types::Reasons,
                }
                #[derive(
                    ::subxt::ext::codec::CompactAs,
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExtraFlags(pub ::core::primitive::u128);
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IdAmount<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod pallet_contracts {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::call_old_weight`].
                    call_old_weight {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::instantiate_with_code_old_weight`].
                    instantiate_with_code_old_weight {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    /// See [`Pallet::instantiate_old_weight`].
                    instantiate_old_weight {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        gas_limit: ::core::primitive::u64,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code_hash: ::subxt::utils::H256,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    /// See [`Pallet::upload_code`].
                    upload_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        determinism: runtime_types::pallet_contracts::wasm::Determinism,
                    },
                    #[codec(index = 4)]
                    /// See [`Pallet::remove_code`].
                    remove_code { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 5)]
                    /// See [`Pallet::set_code`].
                    set_code {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        code_hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 6)]
                    /// See [`Pallet::call`].
                    call {
                        dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 7)]
                    /// See [`Pallet::instantiate_with_code`].
                    instantiate_with_code {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code: ::std::vec::Vec<::core::primitive::u8>,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 8)]
                    /// See [`Pallet::instantiate`].
                    instantiate {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        gas_limit: runtime_types::sp_weights::weight_v2::Weight,
                        storage_deposit_limit: ::core::option::Option<
                            ::subxt::ext::codec::Compact<::core::primitive::u128>,
                        >,
                        code_hash: ::subxt::utils::H256,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    /// See [`Pallet::migrate`].
                    migrate {
                        weight_limit: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 10)]
                    /// See [`Pallet::replace_contract_master`].
                    replace_contract_master {
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 11)]
                    /// See [`Pallet::remove_contract_master`].
                    remove_contract_master,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Invalid schedule supplied, e.g. with zero weight of a
                    /// basic operation.
                    InvalidSchedule,
                    #[codec(index = 1)]
                    /// Invalid combination of flags supplied to `seal_call` or
                    /// `seal_delegate_call`.
                    InvalidCallFlags,
                    #[codec(index = 2)]
                    /// The executed contract exhausted its gas limit.
                    OutOfGas,
                    #[codec(index = 3)]
                    /// The output buffer supplied to a contract API call was
                    /// too small.
                    OutputBufferTooSmall,
                    #[codec(index = 4)]
                    /// Performing the requested transfer failed. Probably
                    /// because there isn't enough
                    /// free balance in the sender's account.
                    TransferFailed,
                    #[codec(index = 5)]
                    /// Performing a call was denied because the calling depth
                    /// reached the limit
                    /// of what is specified in the schedule.
                    MaxCallDepthReached,
                    #[codec(index = 6)]
                    /// No contract was found at the specified address.
                    ContractNotFound,
                    #[codec(index = 7)]
                    /// The code supplied to `instantiate_with_code` exceeds the
                    /// limit specified in the
                    /// current schedule.
                    CodeTooLarge,
                    #[codec(index = 8)]
                    /// No code could be found at the supplied code hash.
                    CodeNotFound,
                    #[codec(index = 9)]
                    /// No code info could be found at the supplied code hash.
                    CodeInfoNotFound,
                    #[codec(index = 10)]
                    /// A buffer outside of sandbox memory was passed to a
                    /// contract API function.
                    OutOfBounds,
                    #[codec(index = 11)]
                    /// Input passed to a contract API function failed to decode
                    /// as expected type.
                    DecodingFailed,
                    #[codec(index = 12)]
                    /// Contract trapped during execution.
                    ContractTrapped,
                    #[codec(index = 13)]
                    /// The size defined in `T::MaxValueSize` was exceeded.
                    ValueTooLarge,
                    #[codec(index = 14)]
                    /// Termination of a contract is not allowed while the
                    /// contract is already
                    /// on the call stack. Can be triggered by `seal_terminate`.
                    TerminatedWhileReentrant,
                    #[codec(index = 15)]
                    /// `seal_call` forwarded this contracts input. It therefore
                    /// is no longer available.
                    InputForwarded,
                    #[codec(index = 16)]
                    /// The subject passed to `seal_random` exceeds the limit.
                    RandomSubjectTooLong,
                    #[codec(index = 17)]
                    /// The amount of topics passed to `seal_deposit_events`
                    /// exceeds the limit.
                    TooManyTopics,
                    #[codec(index = 18)]
                    /// The chain does not provide a chain extension. Calling
                    /// the chain extension results
                    /// in this error. Note that this usually  shouldn't happen
                    /// as deploying such contracts
                    /// is rejected.
                    NoChainExtension,
                    #[codec(index = 19)]
                    /// A contract with the same AccountId already exists.
                    DuplicateContract,
                    #[codec(index = 20)]
                    /// A contract self destructed in its constructor.
                    ///
                    /// This can be triggered by a call to `seal_terminate`.
                    TerminatedInConstructor,
                    #[codec(index = 21)]
                    /// A call tried to invoke a contract that is flagged as
                    /// non-reentrant.
                    /// The only other cause is that a call from a contract into
                    /// the runtime tried to call back
                    /// into `pallet-contracts`. This would make the whole
                    /// pallet reentrant with regard to
                    /// contract code execution which is not supported.
                    ReentranceDenied,
                    #[codec(index = 22)]
                    /// Origin doesn't have enough balance to pay the required
                    /// storage deposits.
                    StorageDepositNotEnoughFunds,
                    #[codec(index = 23)]
                    /// More storage was created than allowed by the storage
                    /// deposit limit.
                    StorageDepositLimitExhausted,
                    #[codec(index = 24)]
                    /// Code removal was denied because the code is still in use
                    /// by at least one contract.
                    CodeInUse,
                    #[codec(index = 25)]
                    /// The contract ran to completion but decided to revert its
                    /// storage changes.
                    /// Please note that this error is only returned from
                    /// extrinsics. When called directly
                    /// or via RPC an `Ok` will be returned. In this case the
                    /// caller needs to inspect the flags
                    /// to determine whether a reversion has taken place.
                    ContractReverted,
                    #[codec(index = 26)]
                    /// The contract's code was found to be invalid during
                    /// validation.
                    ///
                    /// The most likely cause of this is that an API was used
                    /// which is not supported by the
                    /// node. This happens if an older node is used with a new
                    /// version of ink!. Try updating
                    /// your node to the newest available version.
                    ///
                    /// A more detailed error can be found on the node console
                    /// if debug messages are enabled
                    /// by supplying `-lruntime::contracts=debug`.
                    CodeRejected,
                    #[codec(index = 27)]
                    /// An indetermistic code was used in a context where this
                    /// is not permitted.
                    Indeterministic,
                    #[codec(index = 28)]
                    /// A pending migration needs to complete before the
                    /// extrinsic can be called.
                    MigrationInProgress,
                    #[codec(index = 29)]
                    /// Migrate dispatch call was attempted but no migration was
                    /// performed.
                    NoMigrationPerformed,
                    #[codec(index = 30)]
                    /// Require ContractMaster for uploading/instantiating
                    /// contract
                    RequiresContractMaster,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// Contract deployed by address at the specified address.
                    Instantiated {
                        deployer: ::subxt::utils::AccountId32,
                        contract: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    /// Contract has been removed.
                    ///
                    /// # Note
                    ///
                    /// The only way for a contract to be removed and emitting
                    /// this event is by calling
                    /// `seal_terminate`.
                    Terminated {
                        contract: ::subxt::utils::AccountId32,
                        beneficiary: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 2)]
                    /// Code with the specified hash has been stored.
                    CodeStored { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 3)]
                    /// A custom event emitted by the contract.
                    ContractEmitted {
                        contract: ::subxt::utils::AccountId32,
                        data: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    /// A code with the specified hash was removed.
                    CodeRemoved { code_hash: ::subxt::utils::H256 },
                    #[codec(index = 5)]
                    /// A contract's code was updated.
                    ContractCodeUpdated {
                        contract: ::subxt::utils::AccountId32,
                        new_code_hash: ::subxt::utils::H256,
                        old_code_hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 6)]
                    /// A contract was called either by a plain account or
                    /// another contract.
                    ///
                    /// # Note
                    ///
                    /// Please keep in mind that like all events this is only
                    /// emitted for successful
                    /// calls. This is because on failure all storage changes
                    /// including events are
                    /// rolled back.
                    Called {
                        caller: runtime_types::pallet_contracts::Origin<
                            runtime_types::nagara_core_runtime::Runtime,
                        >,
                        contract: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 7)]
                    /// A contract delegate called a code hash.
                    ///
                    /// # Note
                    ///
                    /// Please keep in mind that like all events this is only
                    /// emitted for successful
                    /// calls. This is because on failure all storage changes
                    /// including events are
                    /// rolled back.
                    DelegateCalled {
                        contract: ::subxt::utils::AccountId32,
                        code_hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 8)]
                    /// New Contract Master has been replaced.
                    ContractMasterReplaced {
                        from: ::core::option::Option<::subxt::utils::AccountId32>,
                        new: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 9)]
                    /// Contract Master killed.
                    ContractMasterKilled {
                        who: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                }
            }
            pub mod schedule {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct HostFnWeights {
                    pub caller: runtime_types::sp_weights::weight_v2::Weight,
                    pub is_contract: runtime_types::sp_weights::weight_v2::Weight,
                    pub code_hash: runtime_types::sp_weights::weight_v2::Weight,
                    pub own_code_hash: runtime_types::sp_weights::weight_v2::Weight,
                    pub caller_is_origin: runtime_types::sp_weights::weight_v2::Weight,
                    pub caller_is_root: runtime_types::sp_weights::weight_v2::Weight,
                    pub address: runtime_types::sp_weights::weight_v2::Weight,
                    pub gas_left: runtime_types::sp_weights::weight_v2::Weight,
                    pub balance: runtime_types::sp_weights::weight_v2::Weight,
                    pub value_transferred: runtime_types::sp_weights::weight_v2::Weight,
                    pub minimum_balance: runtime_types::sp_weights::weight_v2::Weight,
                    pub block_number: runtime_types::sp_weights::weight_v2::Weight,
                    pub now: runtime_types::sp_weights::weight_v2::Weight,
                    pub weight_to_fee: runtime_types::sp_weights::weight_v2::Weight,
                    pub input: runtime_types::sp_weights::weight_v2::Weight,
                    pub input_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub r#return: runtime_types::sp_weights::weight_v2::Weight,
                    pub return_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub terminate: runtime_types::sp_weights::weight_v2::Weight,
                    pub random: runtime_types::sp_weights::weight_v2::Weight,
                    pub deposit_event: runtime_types::sp_weights::weight_v2::Weight,
                    pub deposit_event_per_topic: runtime_types::sp_weights::weight_v2::Weight,
                    pub deposit_event_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub debug_message: runtime_types::sp_weights::weight_v2::Weight,
                    pub debug_message_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_storage_per_new_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_storage_per_old_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub set_code_hash: runtime_types::sp_weights::weight_v2::Weight,
                    pub clear_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub clear_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub contains_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub contains_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub get_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub get_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub take_storage: runtime_types::sp_weights::weight_v2::Weight,
                    pub take_storage_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub transfer: runtime_types::sp_weights::weight_v2::Weight,
                    pub call: runtime_types::sp_weights::weight_v2::Weight,
                    pub delegate_call: runtime_types::sp_weights::weight_v2::Weight,
                    pub call_transfer_surcharge: runtime_types::sp_weights::weight_v2::Weight,
                    pub call_per_cloned_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate_transfer_surcharge:
                        runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate_per_input_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiate_per_salt_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_sha2_256: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_sha2_256_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_keccak_256: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_keccak_256_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_256: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_256_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_128: runtime_types::sp_weights::weight_v2::Weight,
                    pub hash_blake2_128_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub ecdsa_recover: runtime_types::sp_weights::weight_v2::Weight,
                    pub ecdsa_to_eth_address: runtime_types::sp_weights::weight_v2::Weight,
                    pub sr25519_verify: runtime_types::sp_weights::weight_v2::Weight,
                    pub sr25519_verify_per_byte: runtime_types::sp_weights::weight_v2::Weight,
                    pub reentrance_count: runtime_types::sp_weights::weight_v2::Weight,
                    pub account_reentrance_count: runtime_types::sp_weights::weight_v2::Weight,
                    pub instantiation_nonce: runtime_types::sp_weights::weight_v2::Weight,
                }
                #[derive(
                    ::subxt::ext::codec::CompactAs,
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InstructionWeights {
                    pub base: ::core::primitive::u32,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Limits {
                    pub event_topics: ::core::primitive::u32,
                    pub globals: ::core::primitive::u32,
                    pub locals: ::core::primitive::u32,
                    pub parameters: ::core::primitive::u32,
                    pub memory_pages: ::core::primitive::u32,
                    pub table_size: ::core::primitive::u32,
                    pub br_table_size: ::core::primitive::u32,
                    pub subject_len: ::core::primitive::u32,
                    pub payload_len: ::core::primitive::u32,
                    pub runtime_memory: ::core::primitive::u32,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Schedule {
                    pub limits: runtime_types::pallet_contracts::schedule::Limits,
                    pub instruction_weights:
                        runtime_types::pallet_contracts::schedule::InstructionWeights,
                    pub host_fn_weights: runtime_types::pallet_contracts::schedule::HostFnWeights,
                }
            }
            pub mod storage {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ContractInfo {
                    pub trie_id: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                    pub deposit_account: runtime_types::pallet_contracts::storage::DepositAccount,
                    pub code_hash: ::subxt::utils::H256,
                    pub storage_bytes: ::core::primitive::u32,
                    pub storage_items: ::core::primitive::u32,
                    pub storage_byte_deposit: ::core::primitive::u128,
                    pub storage_item_deposit: ::core::primitive::u128,
                    pub storage_base_deposit: ::core::primitive::u128,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DeletionQueueManager {
                    pub insert_counter: ::core::primitive::u32,
                    pub delete_counter: ::core::primitive::u32,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DepositAccount(pub ::subxt::utils::AccountId32);
            }
            pub mod wasm {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CodeInfo {
                    pub owner: ::subxt::utils::AccountId32,
                    #[codec(compact)]
                    pub deposit: ::core::primitive::u128,
                    #[codec(compact)]
                    pub refcount: ::core::primitive::u64,
                    pub determinism: runtime_types::pallet_contracts::wasm::Determinism,
                    pub code_len: ::core::primitive::u32,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Determinism {
                    #[codec(index = 0)]
                    Enforced,
                    #[codec(index = 1)]
                    Relaxed,
                }
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Origin<_0> {
                #[codec(index = 0)]
                Root,
                #[codec(index = 1)]
                Signed(::subxt::utils::AccountId32),
                __Ignore(::core::marker::PhantomData<_0>),
            }
        }
        pub mod pallet_contracts_primitives {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Code<_0> {
                #[codec(index = 0)]
                Upload(::std::vec::Vec<::core::primitive::u8>),
                #[codec(index = 1)]
                Existing(_0),
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CodeUploadReturnValue<_0, _1> {
                pub code_hash: _0,
                pub deposit: _1,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ContractAccessError {
                #[codec(index = 0)]
                DoesntExist,
                #[codec(index = 1)]
                KeyDecodingFailed,
                #[codec(index = 2)]
                MigrationInProgress,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ContractResult<_0, _1, _2> {
                pub gas_consumed: runtime_types::sp_weights::weight_v2::Weight,
                pub gas_required: runtime_types::sp_weights::weight_v2::Weight,
                pub storage_deposit: runtime_types::pallet_contracts_primitives::StorageDeposit<_1>,
                pub debug_message: ::std::vec::Vec<::core::primitive::u8>,
                pub result: _0,
                pub events: ::core::option::Option<::std::vec::Vec<_2>>,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ExecReturnValue {
                pub flags: runtime_types::pallet_contracts_primitives::ReturnFlags,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InstantiateReturnValue<_0> {
                pub result: runtime_types::pallet_contracts_primitives::ExecReturnValue,
                pub account_id: _0,
            }
            #[derive(
                ::subxt::ext::codec::CompactAs,
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ReturnFlags {
                pub bits: ::core::primitive::u32,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum StorageDeposit<_0> {
                #[codec(index = 0)]
                Refund(_0),
                #[codec(index = 1)]
                Charge(_0),
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::report_equivocation`].
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::report_equivocation_unsigned`].
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_grandpa::EquivocationProof<
                                ::subxt::utils::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    /// See [`Pallet::note_stalled`].
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Attempt to signal GRANDPA pause when the authority set
                    /// isn't live
                    /// (either paused or already pending pause).
                    PauseFailed,
                    #[codec(index = 1)]
                    /// Attempt to signal GRANDPA resume when the authority set
                    /// isn't paused
                    /// (either live or already pending resume).
                    ResumeFailed,
                    #[codec(index = 2)]
                    /// Attempt to signal GRANDPA change with one already
                    /// pending.
                    ChangePending,
                    #[codec(index = 3)]
                    /// Cannot signal forced change so soon after last.
                    TooSoon,
                    #[codec(index = 4)]
                    /// A key ownership proof provided as part of an
                    /// equivocation report is invalid.
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    /// An equivocation proof provided as part of an
                    /// equivocation report is invalid.
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    /// A given equivocation report is valid but already
                    /// previously reported.
                    DuplicateOffenceReport,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// New authority set has been applied.
                    NewAuthorities {
                        authority_set: ::std::vec::Vec<(
                            runtime_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    },
                    #[codec(index = 1)]
                    /// Current authority set has been paused.
                    Paused,
                    #[codec(index = 2)]
                    /// Current authority set has been resumed.
                    Resumed,
                }
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_identity {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Identity pallet declaration.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::add_registrar`].
                    add_registrar {
                        account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::set_identity`].
                    set_identity {
                        info:
                            ::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
                    },
                    #[codec(index = 2)]
                    /// See [`Pallet::set_subs`].
                    set_subs {
                        subs: ::std::vec::Vec<(
                            ::subxt::utils::AccountId32,
                            runtime_types::pallet_identity::types::Data,
                        )>,
                    },
                    #[codec(index = 3)]
                    /// See [`Pallet::clear_identity`].
                    clear_identity,
                    #[codec(index = 4)]
                    /// See [`Pallet::request_judgement`].
                    request_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        #[codec(compact)]
                        max_fee: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    /// See [`Pallet::cancel_request`].
                    cancel_request { reg_index: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    /// See [`Pallet::set_fee`].
                    set_fee {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    /// See [`Pallet::set_account_id`].
                    set_account_id {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 8)]
                    /// See [`Pallet::set_fields`].
                    set_fields {
                        #[codec(compact)]
                        index: ::core::primitive::u32,
                        fields: runtime_types::pallet_identity::types::BitFlags<
                            runtime_types::pallet_identity::types::IdentityField,
                        >,
                    },
                    #[codec(index = 9)]
                    /// See [`Pallet::provide_judgement`].
                    provide_judgement {
                        #[codec(compact)]
                        reg_index: ::core::primitive::u32,
                        target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        judgement: runtime_types::pallet_identity::types::Judgement<
                            ::core::primitive::u128,
                        >,
                        identity: ::subxt::utils::H256,
                    },
                    #[codec(index = 10)]
                    /// See [`Pallet::kill_identity`].
                    kill_identity {
                        target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 11)]
                    /// See [`Pallet::add_sub`].
                    add_sub {
                        sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 12)]
                    /// See [`Pallet::rename_sub`].
                    rename_sub {
                        sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        data: runtime_types::pallet_identity::types::Data,
                    },
                    #[codec(index = 13)]
                    /// See [`Pallet::remove_sub`].
                    remove_sub {
                        sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 14)]
                    /// See [`Pallet::quit_sub`].
                    quit_sub,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Too many subs-accounts.
                    TooManySubAccounts,
                    #[codec(index = 1)]
                    /// Account isn't found.
                    NotFound,
                    #[codec(index = 2)]
                    /// Account isn't named.
                    NotNamed,
                    #[codec(index = 3)]
                    /// Empty index.
                    EmptyIndex,
                    #[codec(index = 4)]
                    /// Fee is changed.
                    FeeChanged,
                    #[codec(index = 5)]
                    /// No identity found.
                    NoIdentity,
                    #[codec(index = 6)]
                    /// Sticky judgement.
                    StickyJudgement,
                    #[codec(index = 7)]
                    /// Judgement given.
                    JudgementGiven,
                    #[codec(index = 8)]
                    /// Invalid judgement.
                    InvalidJudgement,
                    #[codec(index = 9)]
                    /// The index is invalid.
                    InvalidIndex,
                    #[codec(index = 10)]
                    /// The target is invalid.
                    InvalidTarget,
                    #[codec(index = 11)]
                    /// Too many additional fields.
                    TooManyFields,
                    #[codec(index = 12)]
                    /// Maximum amount of registrars reached. Cannot add any
                    /// more.
                    TooManyRegistrars,
                    #[codec(index = 13)]
                    /// Account ID is already named.
                    AlreadyClaimed,
                    #[codec(index = 14)]
                    /// Sender is not a sub-account.
                    NotSub,
                    #[codec(index = 15)]
                    /// Sub-account isn't owned by sender.
                    NotOwned,
                    #[codec(index = 16)]
                    /// The provided judgement was for a different identity.
                    JudgementForDifferentIdentity,
                    #[codec(index = 17)]
                    /// Error that occurs when there is an issue paying for
                    /// judgement.
                    JudgementPaymentFailed,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// A name was set or reset (which will remove all
                    /// judgements).
                    IdentitySet { who: ::subxt::utils::AccountId32 },
                    #[codec(index = 1)]
                    /// A name was cleared, and the given balance returned.
                    IdentityCleared {
                        who: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    /// A name was removed and the given balance slashed.
                    IdentityKilled {
                        who: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    /// A judgement was asked from a registrar.
                    JudgementRequested {
                        who: ::subxt::utils::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    /// A judgement request was retracted.
                    JudgementUnrequested {
                        who: ::subxt::utils::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    /// A judgement was given by a registrar.
                    JudgementGiven {
                        target: ::subxt::utils::AccountId32,
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    /// A registrar was added.
                    RegistrarAdded {
                        registrar_index: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    /// A sub-identity was added to an identity and the deposit
                    /// paid.
                    SubIdentityAdded {
                        sub: ::subxt::utils::AccountId32,
                        main: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    /// A sub-identity was removed from an identity and the
                    /// deposit freed.
                    SubIdentityRemoved {
                        sub: ::subxt::utils::AccountId32,
                        main: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    /// A sub-identity was cleared, and the given deposit
                    /// repatriated from the
                    /// main identity account to the sub-identity account.
                    SubIdentityRevoked {
                        sub: ::subxt::utils::AccountId32,
                        main: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::CompactAs,
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct BitFlags<_0>(
                    pub ::core::primitive::u64,
                    #[codec(skip)] pub ::core::marker::PhantomData<_0>,
                );
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Data {
                    #[codec(index = 0)]
                    None,
                    #[codec(index = 1)]
                    Raw0([::core::primitive::u8; 0usize]),
                    #[codec(index = 2)]
                    Raw1([::core::primitive::u8; 1usize]),
                    #[codec(index = 3)]
                    Raw2([::core::primitive::u8; 2usize]),
                    #[codec(index = 4)]
                    Raw3([::core::primitive::u8; 3usize]),
                    #[codec(index = 5)]
                    Raw4([::core::primitive::u8; 4usize]),
                    #[codec(index = 6)]
                    Raw5([::core::primitive::u8; 5usize]),
                    #[codec(index = 7)]
                    Raw6([::core::primitive::u8; 6usize]),
                    #[codec(index = 8)]
                    Raw7([::core::primitive::u8; 7usize]),
                    #[codec(index = 9)]
                    Raw8([::core::primitive::u8; 8usize]),
                    #[codec(index = 10)]
                    Raw9([::core::primitive::u8; 9usize]),
                    #[codec(index = 11)]
                    Raw10([::core::primitive::u8; 10usize]),
                    #[codec(index = 12)]
                    Raw11([::core::primitive::u8; 11usize]),
                    #[codec(index = 13)]
                    Raw12([::core::primitive::u8; 12usize]),
                    #[codec(index = 14)]
                    Raw13([::core::primitive::u8; 13usize]),
                    #[codec(index = 15)]
                    Raw14([::core::primitive::u8; 14usize]),
                    #[codec(index = 16)]
                    Raw15([::core::primitive::u8; 15usize]),
                    #[codec(index = 17)]
                    Raw16([::core::primitive::u8; 16usize]),
                    #[codec(index = 18)]
                    Raw17([::core::primitive::u8; 17usize]),
                    #[codec(index = 19)]
                    Raw18([::core::primitive::u8; 18usize]),
                    #[codec(index = 20)]
                    Raw19([::core::primitive::u8; 19usize]),
                    #[codec(index = 21)]
                    Raw20([::core::primitive::u8; 20usize]),
                    #[codec(index = 22)]
                    Raw21([::core::primitive::u8; 21usize]),
                    #[codec(index = 23)]
                    Raw22([::core::primitive::u8; 22usize]),
                    #[codec(index = 24)]
                    Raw23([::core::primitive::u8; 23usize]),
                    #[codec(index = 25)]
                    Raw24([::core::primitive::u8; 24usize]),
                    #[codec(index = 26)]
                    Raw25([::core::primitive::u8; 25usize]),
                    #[codec(index = 27)]
                    Raw26([::core::primitive::u8; 26usize]),
                    #[codec(index = 28)]
                    Raw27([::core::primitive::u8; 27usize]),
                    #[codec(index = 29)]
                    Raw28([::core::primitive::u8; 28usize]),
                    #[codec(index = 30)]
                    Raw29([::core::primitive::u8; 29usize]),
                    #[codec(index = 31)]
                    Raw30([::core::primitive::u8; 30usize]),
                    #[codec(index = 32)]
                    Raw31([::core::primitive::u8; 31usize]),
                    #[codec(index = 33)]
                    Raw32([::core::primitive::u8; 32usize]),
                    #[codec(index = 34)]
                    BlakeTwo256([::core::primitive::u8; 32usize]),
                    #[codec(index = 35)]
                    Sha256([::core::primitive::u8; 32usize]),
                    #[codec(index = 36)]
                    Keccak256([::core::primitive::u8; 32usize]),
                    #[codec(index = 37)]
                    ShaThree256([::core::primitive::u8; 32usize]),
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum IdentityField {
                    #[codec(index = 1)]
                    Display,
                    #[codec(index = 2)]
                    Legal,
                    #[codec(index = 4)]
                    Web,
                    #[codec(index = 8)]
                    Riot,
                    #[codec(index = 16)]
                    Email,
                    #[codec(index = 32)]
                    PgpFingerprint,
                    #[codec(index = 64)]
                    Image,
                    #[codec(index = 128)]
                    Twitter,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct IdentityInfo {
                    pub additional: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        runtime_types::pallet_identity::types::Data,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                    pub display: runtime_types::pallet_identity::types::Data,
                    pub legal: runtime_types::pallet_identity::types::Data,
                    pub web: runtime_types::pallet_identity::types::Data,
                    pub riot: runtime_types::pallet_identity::types::Data,
                    pub email: runtime_types::pallet_identity::types::Data,
                    pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
                    pub image: runtime_types::pallet_identity::types::Data,
                    pub twitter: runtime_types::pallet_identity::types::Data,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Judgement<_0> {
                    #[codec(index = 0)]
                    Unknown,
                    #[codec(index = 1)]
                    FeePaid(_0),
                    #[codec(index = 2)]
                    Reasonable,
                    #[codec(index = 3)]
                    KnownGood,
                    #[codec(index = 4)]
                    OutOfDate,
                    #[codec(index = 5)]
                    LowQuality,
                    #[codec(index = 6)]
                    Erroneous,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RegistrarInfo<_0, _1> {
                    pub account: _1,
                    pub fee: _0,
                    pub fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Registration<_0> {
                    pub judgements: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        ::core::primitive::u32,
                        runtime_types::pallet_identity::types::Judgement<_0>,
                    )>,
                    pub deposit: _0,
                    pub info: runtime_types::pallet_identity::types::IdentityInfo,
                }
            }
        }
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::as_multi_threshold_1`].
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        call: ::std::boxed::Box<runtime_types::nagara_core_runtime::RuntimeCall>,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::as_multi`].
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call: ::std::boxed::Box<runtime_types::nagara_core_runtime::RuntimeCall>,
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    /// See [`Pallet::approve_as_multi`].
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 3)]
                    /// See [`Pallet::cancel_as_multi`].
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Threshold must be 2 or greater.
                    MinimumThreshold,
                    #[codec(index = 1)]
                    /// Call is already approved by this signatory.
                    AlreadyApproved,
                    #[codec(index = 2)]
                    /// Call doesn't need any (more) approvals.
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    /// There are too few signatories in the list.
                    TooFewSignatories,
                    #[codec(index = 4)]
                    /// There are too many signatories in the list.
                    TooManySignatories,
                    #[codec(index = 5)]
                    /// The signatories were provided out of order; they should
                    /// be ordered.
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    /// The sender was contained in the other signatories; it
                    /// shouldn't be.
                    SenderInSignatories,
                    #[codec(index = 7)]
                    /// Multisig operation not found when attempting to cancel.
                    NotFound,
                    #[codec(index = 8)]
                    /// Only the account that originally created the multisig is
                    /// able to cancel it.
                    NotOwner,
                    #[codec(index = 9)]
                    /// No timepoint was given, yet the multisig operation is
                    /// already underway.
                    NoTimepoint,
                    #[codec(index = 10)]
                    /// A different timepoint was given to the multisig
                    /// operation that is underway.
                    WrongTimepoint,
                    #[codec(index = 11)]
                    /// A timepoint was given, yet no multisig operation is
                    /// underway.
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    /// The maximum weight information provided was too low.
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    /// The data to be stored is already stored.
                    AlreadyStored,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// A new multisig operation has begun.
                    NewMultisig {
                        approving: ::subxt::utils::AccountId32,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    /// A multisig operation has been approved by someone.
                    MultisigApproval {
                        approving: ::subxt::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    /// A multisig operation has been executed.
                    MultisigExecuted {
                        approving: ::subxt::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    /// A multisig operation has been cancelled.
                    MultisigCancelled {
                        cancelling: ::subxt::utils::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::utils::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: runtime_types::bounded_collections::bounded_vec::BoundedVec<_2>,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: ::core::primitive::u32,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::set_keys`].
                    set_keys {
                        keys: runtime_types::nagara_core_runtime::opaque::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::purge_keys`].
                    purge_keys,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Error for the session pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Invalid ownership proof.
                    InvalidProof,
                    #[codec(index = 1)]
                    /// No associated validator ID for account.
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    /// Registered duplicate key.
                    DuplicatedKey,
                    #[codec(index = 3)]
                    /// No keys are associated with this account.
                    NoKeys,
                    #[codec(index = 4)]
                    /// Key setting account is not live, so it's impossible to
                    /// associate keys.
                    NoAccount,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// New session has happened. Note that the argument is the
                    /// session index, not the
                    /// block number as the type might suggest.
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::sudo`].
                    sudo {
                        call: ::std::boxed::Box<runtime_types::nagara_core_runtime::RuntimeCall>,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::sudo_unchecked_weight`].
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::nagara_core_runtime::RuntimeCall>,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                    #[codec(index = 2)]
                    /// See [`Pallet::set_key`].
                    set_key {
                        new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    },
                    #[codec(index = 3)]
                    /// See [`Pallet::sudo_as`].
                    sudo_as {
                        who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        call: ::std::boxed::Box<runtime_types::nagara_core_runtime::RuntimeCall>,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Error for the Sudo pallet
                pub enum Error {
                    #[codec(index = 0)]
                    /// Sender must be the Sudo account
                    RequireSudo,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// A sudo just took place. \[result\]
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    /// The \[sudoer\] just switched identity; the old key is
                    /// supplied if one existed.
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 2)]
                    /// A sudo just took place. \[result\]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::set`].
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// A transaction fee `actual_fee`, of which `tip` was added
                    /// to the minimum inclusion fee,
                    /// has been paid by `who`.
                    TransactionFeePaid {
                        who: ::subxt::utils::AccountId32,
                        actual_fee: ::core::primitive::u128,
                        tip: ::core::primitive::u128,
                    },
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FeeDetails<_0> {
                    pub inclusion_fee: ::core::option::Option<
                        runtime_types::pallet_transaction_payment::types::InclusionFee<_0>,
                    >,
                    pub tip: _0,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct InclusionFee<_0> {
                    pub base_fee: _0,
                    pub len_fee: _0,
                    pub adjusted_weight_fee: _0,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct RuntimeDispatchInfo<_0, _1> {
                    pub weight: _1,
                    pub class: runtime_types::frame_support::dispatch::DispatchClass,
                    pub partial_fee: _0,
                }
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::batch`].
                    batch {
                        calls: ::std::vec::Vec<runtime_types::nagara_core_runtime::RuntimeCall>,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::as_derivative`].
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<runtime_types::nagara_core_runtime::RuntimeCall>,
                    },
                    #[codec(index = 2)]
                    /// See [`Pallet::batch_all`].
                    batch_all {
                        calls: ::std::vec::Vec<runtime_types::nagara_core_runtime::RuntimeCall>,
                    },
                    #[codec(index = 3)]
                    /// See [`Pallet::dispatch_as`].
                    dispatch_as {
                        as_origin:
                            ::std::boxed::Box<runtime_types::nagara_core_runtime::OriginCaller>,
                        call: ::std::boxed::Box<runtime_types::nagara_core_runtime::RuntimeCall>,
                    },
                    #[codec(index = 4)]
                    /// See [`Pallet::force_batch`].
                    force_batch {
                        calls: ::std::vec::Vec<runtime_types::nagara_core_runtime::RuntimeCall>,
                    },
                    #[codec(index = 5)]
                    /// See [`Pallet::with_weight`].
                    with_weight {
                        call: ::std::boxed::Box<runtime_types::nagara_core_runtime::RuntimeCall>,
                        weight: runtime_types::sp_weights::weight_v2::Weight,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Too many calls batched.
                    TooManyCalls,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// Batch of dispatches did not complete fully. Index of
                    /// first failing dispatch given, as
                    /// well as the error.
                    BatchInterrupted {
                        index: ::core::primitive::u32,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 1)]
                    /// Batch of dispatches completed fully with no error.
                    BatchCompleted,
                    #[codec(index = 2)]
                    /// Batch of dispatches completed but has errors.
                    BatchCompletedWithErrors,
                    #[codec(index = 3)]
                    /// A single item within a Batch of dispatches has completed
                    /// with no error.
                    ItemCompleted,
                    #[codec(index = 4)]
                    /// A single item within a Batch of dispatches has completed
                    /// with error.
                    ItemFailed {
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 5)]
                    /// A call was dispatched.
                    DispatchedAs {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::CompactAs,
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
        }
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_consensus_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_consensus_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::CompactAs,
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Slot(pub ::core::primitive::u64);
            #[derive(
                ::subxt::ext::codec::CompactAs,
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SlotDuration(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OpaqueMetadata(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Void {}
        }
        pub mod sp_inherents {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CheckInherentsResult {
                pub okay: ::core::primitive::bool,
                pub fatal_error: ::core::primitive::bool,
                pub errors: runtime_types::sp_inherents::InherentData,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InherentData {
                pub data: ::subxt::utils::KeyedVec<
                    [::core::primitive::u8; 8usize],
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod block {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Block<_0, _1> {
                        pub header: _0,
                        pub extrinsics: ::std::vec::Vec<_1>,
                    }
                }
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum DigestItem {
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        ::subxt::ext::codec::Decode,
                        ::subxt::ext::codec::Encode,
                        ::subxt::ext::scale_decode::DecodeAsType,
                        ::subxt::ext::scale_encode::EncodeAsType,
                        Debug
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub struct Header<_0> {
                        pub parent_hash: ::subxt::utils::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::utils::H256,
                        pub extrinsics_root: ::subxt::utils::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                    }
                }
            }
            pub mod transaction_validity {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum InvalidTransaction {
                    #[codec(index = 0)]
                    Call,
                    #[codec(index = 1)]
                    Payment,
                    #[codec(index = 2)]
                    Future,
                    #[codec(index = 3)]
                    Stale,
                    #[codec(index = 4)]
                    BadProof,
                    #[codec(index = 5)]
                    AncientBirthBlock,
                    #[codec(index = 6)]
                    ExhaustsResources,
                    #[codec(index = 7)]
                    Custom(::core::primitive::u8),
                    #[codec(index = 8)]
                    BadMandatory,
                    #[codec(index = 9)]
                    MandatoryValidation,
                    #[codec(index = 10)]
                    BadSigner,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum TransactionSource {
                    #[codec(index = 0)]
                    InBlock,
                    #[codec(index = 1)]
                    Local,
                    #[codec(index = 2)]
                    External,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum TransactionValidityError {
                    #[codec(index = 0)]
                    Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
                    #[codec(index = 1)]
                    Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum UnknownTransaction {
                    #[codec(index = 0)]
                    CannotLookup,
                    #[codec(index = 1)]
                    NoUnsignedValidator,
                    #[codec(index = 2)]
                    Custom(::core::primitive::u8),
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ValidTransaction {
                    pub priority: ::core::primitive::u64,
                    pub requires: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    pub provides: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    pub longevity: ::core::primitive::u64,
                    pub propagate: ::core::primitive::bool,
                }
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module(runtime_types::sp_runtime::ModuleError),
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
                Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
                #[codec(index = 9)]
                Transactional(runtime_types::sp_runtime::TransactionalError),
                #[codec(index = 10)]
                Exhausted,
                #[codec(index = 11)]
                Corruption,
                #[codec(index = 12)]
                Unavailable,
                #[codec(index = 13)]
                RootNotAllowed,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ModuleError {
                pub index: ::core::primitive::u8,
                pub error: [::core::primitive::u8; 4usize],
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TokenError {
                #[codec(index = 0)]
                FundsUnavailable,
                #[codec(index = 1)]
                OnlyProvider,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
                #[codec(index = 7)]
                CannotCreateHold,
                #[codec(index = 8)]
                NotExpendable,
                #[codec(index = 9)]
                Blocked,
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TransactionalError {
                #[codec(index = 0)]
                LimitReached,
                #[codec(index = 1)]
                NoLayer,
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod sp_weights {
            use super::runtime_types;
            pub mod weight_v2 {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Weight {
                    #[codec(compact)]
                    pub ref_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub proof_size: ::core::primitive::u64,
                }
            }
            #[derive(
                ::subxt::ext::codec::Decode,
                ::subxt::ext::codec::Encode,
                ::subxt::ext::scale_decode::DecodeAsType,
                ::subxt::ext::scale_encode::EncodeAsType,
                Debug
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeDbWeight {
                pub read: ::core::primitive::u64,
                pub write: ::core::primitive::u64,
            }
        }
        pub mod substrate_validator_set {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// Contains a variant per dispatchable extrinsic that this
                /// pallet has.
                pub enum Call {
                    #[codec(index = 0)]
                    /// See [`Pallet::add_validator`].
                    add_validator {
                        validator_id: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    /// See [`Pallet::remove_validator`].
                    remove_validator {
                        validator_id: ::subxt::utils::AccountId32,
                    },
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Error` enum of this pallet.
                pub enum Error {
                    #[codec(index = 0)]
                    /// Target (post-removal) validator count is below the
                    /// minimum.
                    TooLowValidatorCount,
                    #[codec(index = 1)]
                    /// Validator is already in the validator set.
                    Duplicate,
                }
                #[derive(
                    ::subxt::ext::codec::Decode,
                    ::subxt::ext::codec::Encode,
                    ::subxt::ext::scale_decode::DecodeAsType,
                    ::subxt::ext::scale_encode::EncodeAsType,
                    Debug
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                /// The `Event` enum of this pallet
                pub enum Event {
                    #[codec(index = 0)]
                    /// New validator addition initiated. Effective in ~2
                    /// sessions.
                    ValidatorAdditionInitiated(::subxt::utils::AccountId32),
                    #[codec(index = 1)]
                    /// Validator removal initiated. Effective in ~2 sessions.
                    ValidatorRemovalInitiated(::subxt::utils::AccountId32),
                }
            }
        }
    }
}
