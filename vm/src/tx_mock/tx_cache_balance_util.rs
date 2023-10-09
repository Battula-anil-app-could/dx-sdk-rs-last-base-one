use num_bigint::BigUint;

use crate::{
    tx_execution::is_system_sc_address, tx_mock::TxPanic, types::VMAddress,
    world_mock::DctInstanceMetadata,
};

use super::TxCache;

impl TxCache {
    pub fn subtract_egld_balance(
        &self,
        address: &VMAddress,
        call_value: &BigUint,
    ) -> Result<(), TxPanic> {
        self.with_account_mut(address, |account| {
            if call_value > &account.egld_balance {
                return Err(TxPanic::vm_error("failed transfer (insufficient funds)"));
            }
            account.egld_balance -= call_value;
            Ok(())
        })
    }

    pub fn subtract_tx_gas(&self, address: &VMAddress, gas_limit: u64, gas_price: u64) {
        self.with_account_mut(address, |account| {
            let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
            assert!(
                account.egld_balance >= gas_cost,
                "Not enough balance to pay gas upfront"
            );
            account.egld_balance -= &gas_cost;
        });
    }

    pub fn increase_egld_balance(&self, address: &VMAddress, amount: &BigUint) {
        self.with_account_mut(address, |account| {
            account.egld_balance += amount;
        });
    }

    pub fn subtract_dct_balance(
        &self,
        address: &VMAddress,
        dct_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) -> Result<DctInstanceMetadata, TxPanic> {
        self.with_account_mut(address, |account| {
            let dct_data_map = &mut account.dct;
            let dct_data = dct_data_map
                .get_mut_by_identifier(dct_token_identifier)
                .ok_or_else(err_insufficient_funds)?;

            let dct_instances = &mut dct_data.instances;
            let dct_instance = dct_instances
                .get_mut_by_nonce(nonce)
                .ok_or_else(err_insufficient_funds)?;

            let dct_balance = &mut dct_instance.balance;
            if &*dct_balance < value {
                return Err(err_insufficient_funds());
            }

            *dct_balance -= value;

            Ok(dct_instance.metadata.clone())
        })
    }

    pub fn increase_dct_balance(
        &self,
        address: &VMAddress,
        dct_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
        dct_metadata: DctInstanceMetadata,
    ) {
        self.with_account_mut(address, |account| {
            account.dct.increase_balance(
                dct_token_identifier.to_vec(),
                nonce,
                value,
                dct_metadata,
            );
        });
    }

    pub fn transfer_egld_balance(
        &self,
        from: &VMAddress,
        to: &VMAddress,
        value: &BigUint,
    ) -> Result<(), TxPanic> {
        if !is_system_sc_address(from) {
            self.subtract_egld_balance(from, value)?;
        }
        if !is_system_sc_address(to) {
            self.increase_egld_balance(to, value);
        }
        Ok(())
    }

    pub fn transfer_dct_balance(
        &self,
        from: &VMAddress,
        to: &VMAddress,
        dct_token_identifier: &[u8],
        nonce: u64,
        value: &BigUint,
    ) -> Result<(), TxPanic> {
        if !is_system_sc_address(from) && !is_system_sc_address(to) {
            let metadata = self.subtract_dct_balance(from, dct_token_identifier, nonce, value)?;
            self.increase_dct_balance(to, dct_token_identifier, nonce, value, metadata);
        }
        Ok(())
    }
}

fn err_insufficient_funds() -> TxPanic {
    TxPanic::vm_error("insufficient funds")
}
