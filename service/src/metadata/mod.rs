pub mod nagara;

pub(crate) fn to_nagara_ss58(
    source: subxt::utils::AccountId32,
) -> sp_core::crypto::AccountId32 {
    sp_core::crypto::AccountId32::from(source.0)
}

pub(crate) fn to_nagara_ss58_string(
    source: subxt::utils::AccountId32,
) -> String {
    to_nagara_ss58(source).to_string()
}
