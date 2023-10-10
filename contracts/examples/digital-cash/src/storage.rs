dharitri_sc::imports!();
dharitri_sc::derive_imports!();

use crate::deposit_info::*;

#[dharitri_sc::module]
pub trait StorageModule {
    #[view]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<DepositInfo<Self::Api>>;

    #[storage_mapper("fee")]
    fn fee(&self, token: &MoaxOrDctTokenIdentifier) -> SingleValueMapper<BigUint>;

    #[storage_mapper("collectedFees")]
    fn collected_fees(&self, token: &MoaxOrDctTokenIdentifier) -> SingleValueMapper<BigUint>;

    #[storage_mapper("whitelistedFeeTokens")]
    fn whitelisted_fee_tokens(&self) -> UnorderedSetMapper<MoaxOrDctTokenIdentifier>;

    #[storage_mapper("allTimeFeeTokens")]
    fn all_time_fee_tokens(&self) -> UnorderedSetMapper<MoaxOrDctTokenIdentifier>;
}
