pub mod goro;

pub(crate) fn to_goro_ss58(source: subxt::utils::AccountId32) -> sp_core::crypto::AccountId32 {
    sp_core::crypto::AccountId32::from(source.0)
}

pub(crate) fn to_goro_ss58_string(source: subxt::utils::AccountId32) -> String {
    to_goro_ss58(source).to_string()
}
