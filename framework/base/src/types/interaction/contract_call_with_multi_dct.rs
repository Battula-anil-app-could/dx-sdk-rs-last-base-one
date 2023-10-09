use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, DctTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec, TokenIdentifier,
    },
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithEgld};

#[must_use]
pub struct ContractCallWithMultiDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub basic: ContractCallNoPayment<SA, OriginalResult>,
    pub dct_payments: ManagedVec<SA, DctTokenPayment<SA>>,
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithMultiDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithEgld<SA, Self::OriginalResult> {
        self.basic
            .into_normalized()
            .convert_to_dct_transfer_call(self.dct_payments)
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        self.basic.transfer_execute_dct(self.dct_payments);
    }
}

impl<SA, OriginalResult> ContractCallWithMultiDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_dct_transfer` or `with_multi_token_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        payments: ManagedVec<SA, DctTokenPayment<SA>>,
    ) -> Self {
        ContractCallWithMultiDct {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            dct_payments: payments,
        }
    }

    /// Adds a single DCT token transfer to a contract call.
    ///
    /// Can be called multiple times on the same call.
    pub fn with_dct_transfer<P: Into<DctTokenPayment<SA>>>(mut self, payment: P) -> Self {
        self.dct_payments.push(payment.into());
        self
    }

    #[deprecated(
        since = "0.39.0",
        note = "Replace by `contract_call.with_dct_transfer((payment_token, payment_nonce, payment_amount))`. 
        The tuple argument will get automatically converted to DctTokenPayment."
    )]
    pub fn add_dct_token_transfer(
        self,
        payment_token: TokenIdentifier<SA>,
        payment_nonce: u64,
        payment_amount: BigUint<SA>,
    ) -> Self {
        self.with_dct_transfer((payment_token, payment_nonce, payment_amount))
    }
}
