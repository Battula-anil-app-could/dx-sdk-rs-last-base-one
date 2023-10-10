#![no_std]
#![allow(clippy::type_complexity)]

use dharitri_sc::codec::Empty;

dharitri_sc::imports!();

/// General test contract.
/// Used especially for investigating async calls and contract interaction in general.
#[dharitri_sc::contract]
pub trait Vault {
    #[init]
    fn init(&self, opt_arg_to_echo: OptionalValue<ManagedBuffer>) -> OptionalValue<ManagedBuffer> {
        opt_arg_to_echo
    }

    #[endpoint]
    fn echo_arguments(
        &self,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> MultiValueEncoded<ManagedBuffer> {
        self.call_counts(ManagedBuffer::from(b"echo_arguments"))
            .update(|c| *c += 1);
        args
    }

    #[endpoint]
    fn echo_arguments_without_storage(
        &self,
        args: MultiValueEncoded<ManagedBuffer>,
    ) -> MultiValueEncoded<ManagedBuffer> {
        args
    }

    #[endpoint]
    fn echo_caller(&self) -> ManagedAddress {
        self.blockchain().get_caller()
    }

    fn dct_transfers_multi(&self) -> MultiValueEncoded<DctTokenPaymentMultiValue> {
        self.call_value()
            .all_dct_transfers()
            .clone_value()
            .into_multi_value()
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds(&self) {
        let dct_transfers_multi = self.dct_transfers_multi();
        self.accept_funds_event(&self.call_value().moax_value(), &dct_transfers_multi);

        self.call_counts(ManagedBuffer::from(b"accept_funds"))
            .update(|c| *c += 1);
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_echo_payment(
        &self,
    ) -> MultiValue2<BigUint, MultiValueEncoded<DctTokenPaymentMultiValue>> {
        let moax_value = self.call_value().moax_value();
        let dct_transfers_multi = self.dct_transfers_multi();
        self.accept_funds_event(&moax_value, &dct_transfers_multi);

        self.call_counts(ManagedBuffer::from(b"accept_funds_echo_payment"))
            .update(|c| *c += 1);

        (moax_value.clone_value(), dct_transfers_multi).into()
    }

    #[payable("*")]
    #[endpoint]
    fn accept_funds_single_dct_transfer(&self) {
        let _ = self.call_value().single_dct();
    }

    #[payable("*")]
    #[endpoint]
    fn reject_funds(&self) {
        let dct_transfers_multi = self.dct_transfers_multi();
        self.reject_funds_event(&self.call_value().moax_value(), &dct_transfers_multi);
        sc_panic!("reject_funds");
    }

    #[payable("*")]
    #[endpoint]
    fn retrieve_funds_with_transfer_exec(
        &self,
        token: TokenIdentifier,
        amount: BigUint,
        opt_receive_func: OptionalValue<ManagedBuffer>,
    ) {
        let caller = self.blockchain().get_caller();
        let func_name = opt_receive_func.into_option().unwrap_or_default();

        self.send_raw()
            .transfer_dct_execute(
                &caller,
                &token,
                &amount,
                50_000_000,
                &func_name,
                &ManagedArgBuffer::new(),
            )
            .unwrap_or_else(|_| sc_panic!("DCT transfer failed"));
    }

    #[label("promises-endpoint")]
    #[payable("*")]
    #[endpoint]
    fn retrieve_funds_promises(
        &self,
        back_transfers: OptionalValue<u64>,
        back_transfer_value: OptionalValue<BigUint>,
    ) {
        let payment = self.call_value().moax_or_single_dct();
        let caller = self.blockchain().get_caller();
        let endpoint_name = ManagedBuffer::from(b"");
        let nr_callbacks = match back_transfers.into_option() {
            Some(nr) => nr,
            None => sc_panic!("Nr of calls is None"),
        };

        let value = match back_transfer_value.into_option() {
            Some(val) => val,
            None => sc_panic!("Value for parent callback is None"),
        };

        let return_payment =
            MoaxOrDctTokenPayment::new(payment.token_identifier, payment.token_nonce, value);

        self.num_called_retrieve_funds_promises()
            .update(|c| *c += 1);

        for _ in 0..nr_callbacks {
            self.num_async_calls_sent_from_child().update(|c| *c += 1);

            self.send()
                .contract_call::<()>(caller.clone(), endpoint_name.clone())
                .with_moax_or_single_dct_transfer(return_payment.clone())
                .with_gas_limit(self.blockchain().get_gas_left() / 2)
                .transfer_execute()
        }
    }

    #[endpoint]
    fn retrieve_funds(&self, token: MoaxOrDctTokenIdentifier, nonce: u64, amount: BigUint) {
        self.retrieve_funds_event(&token, nonce, &amount);
        let caller = self.blockchain().get_caller();

        if let Some(dct_token_id) = token.into_dct_option() {
            self.send()
                .direct_dct(&caller, &dct_token_id, nonce, &amount);
        } else {
            self.send().direct_moax(&caller, &amount);
        }
    }

    #[endpoint]
    fn retrieve_multi_funds_async(
        &self,
        token_payments: MultiValueEncoded<MultiValue3<TokenIdentifier, u64, BigUint>>,
    ) {
        let caller = self.blockchain().get_caller();
        let mut all_payments = ManagedVec::new();

        for multi_arg in token_payments.into_iter() {
            let (token_id, nonce, amount) = multi_arg.into_tuple();

            all_payments.push(DctTokenPayment::new(token_id, nonce, amount));
        }

        self.send().direct_multi(&caller, &all_payments);
    }

    #[payable("*")]
    #[endpoint]
    fn burn_and_create_retrive_async(&self) {
        let payments = self.call_value().all_dct_transfers();
        let mut uris = ManagedVec::new();
        uris.push(ManagedBuffer::new());

        let mut new_tokens = ManagedVec::new();

        for payment in payments.into_iter() {
            // burn old tokens
            self.send().dct_local_burn(
                &payment.token_identifier,
                payment.token_nonce,
                &payment.amount,
            );

            // create new ones
            let new_token_nonce = self.send().dct_nft_create(
                &payment.token_identifier,
                &payment.amount,
                &ManagedBuffer::new(),
                &BigUint::zero(),
                &ManagedBuffer::new(),
                &Empty,
                &uris,
            );

            new_tokens.push(DctTokenPayment::new(
                payment.token_identifier,
                new_token_nonce,
                payment.amount,
            ));
        }

        self.send()
            .direct_multi(&self.blockchain().get_caller(), &new_tokens);
    }

    /// TODO: invert token_payment and token_nonce, for consistency.
    #[event("accept_funds")]
    fn accept_funds_event(
        &self,
        #[indexed] moax_value: &BigUint,
        #[indexed] multi_dct: &MultiValueEncoded<DctTokenPaymentMultiValue>,
    );

    #[event("reject_funds")]
    fn reject_funds_event(
        &self,
        #[indexed] moax_value: &BigUint,
        #[indexed] multi_dct: &MultiValueEncoded<DctTokenPaymentMultiValue>,
    );

    #[event("retrieve_funds")]
    fn retrieve_funds_event(
        &self,
        #[indexed] token: &MoaxOrDctTokenIdentifier,
        #[indexed] nonce: u64,
        #[indexed] amount: &BigUint,
    );

    #[endpoint]
    fn get_owner_address(&self) -> ManagedAddress {
        self.blockchain().get_owner_address()
    }

    /// We already leave a trace of the calls using the event logs;
    /// this additional counter has the role of showing that storage also gets saved correctly.
    #[view]
    #[storage_mapper("call_counts")]
    fn call_counts(&self, endpoint: ManagedBuffer) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("num_called_retrieve_funds_promises")]
    fn num_called_retrieve_funds_promises(&self) -> SingleValueMapper<usize>;

    #[view]
    #[storage_mapper("num_async_calls_sent_from_child")]
    fn num_async_calls_sent_from_child(&self) -> SingleValueMapper<usize>;
}
