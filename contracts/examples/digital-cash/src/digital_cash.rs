#![no_std]
#![allow(unused_attributes)]

dharitri_sc::imports!();
dharitri_sc::derive_imports!();

mod constants;
mod deposit_info;
mod helpers;
mod pay_fee_and_fund;
mod signature_operations;
mod storage;

use constants::*;

#[dharitri_sc::contract]
pub trait DigitalCash:
    pay_fee_and_fund::PayFeeAndFund
    + signature_operations::SignatureOperationsModule
    + helpers::HelpersModule
    + storage::StorageModule
{
    #[init]
    fn init(&self, fee: BigUint, token: EgldOrDctTokenIdentifier) {
        self.whitelist_fee_token(fee, token);
    }

    #[endpoint(whitelistFeeToken)]
    #[only_owner]
    fn whitelist_fee_token(&self, fee: BigUint, token: EgldOrDctTokenIdentifier) {
        require!(self.fee(&token).is_empty(), "Token already whitelisted");
        self.fee(&token).set(fee);
        self.whitelisted_fee_tokens().insert(token.clone());
        self.all_time_fee_tokens().insert(token);
    }

    #[endpoint(blacklistFeeToken)]
    #[only_owner]
    fn blacklist_fee_token(&self, token: EgldOrDctTokenIdentifier) {
        require!(!self.fee(&token).is_empty(), "Token is not whitelisted");
        self.fee(&token).clear();
        self.whitelisted_fee_tokens().swap_remove(&token);
    }

    #[endpoint(claimFees)]
    #[only_owner]
    fn claim_fees(&self) {
        let fee_tokens_mapper = self.all_time_fee_tokens();
        let fee_tokens = fee_tokens_mapper.iter();
        let caller_address = self.blockchain().get_caller();
        let mut collected_dct_fees = ManagedVec::new();
        for token in fee_tokens {
            let fee = self.collected_fees(&token).take();
            if fee == 0 {
                continue;
            }
            if token == EgldOrDctTokenIdentifier::egld() {
                self.send().direct_egld(&caller_address, &fee);
            } else {
                let collected_fee = DctTokenPayment::new(token.unwrap_dct(), 0, fee);
                collected_dct_fees.push(collected_fee);
            }
        }
        if !collected_dct_fees.is_empty() {
            self.send()
                .direct_multi(&caller_address, &collected_dct_fees);
        }
    }

    #[view(getAmount)]
    fn get_amount(
        &self,
        address: ManagedAddress,
        token: EgldOrDctTokenIdentifier,
        nonce: u64,
    ) -> BigUint {
        let deposit_mapper = self.deposit(&address);
        require!(!deposit_mapper.is_empty(), NON_EXISTENT_KEY_ERR_MSG);

        let deposit = deposit_mapper.get();
        if token.is_egld() {
            return deposit.egld_funds;
        }

        for dct in deposit.dct_funds.into_iter() {
            if dct.token_identifier == token && dct.token_nonce == nonce {
                return dct.amount;
            }
        }

        BigUint::zero()
    }
}
