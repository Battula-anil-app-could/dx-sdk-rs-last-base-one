use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, MoaxOrDctTokenIdentifier, MoaxOrDctTokenPayment, ManagedAddress, ManagedBuffer,
    },
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithMoax};

/// Holds data for calling another contract, with a single payment, either MOAX or a single DCT token.
///
/// Gets created when chaining method `with_moax_or_single_dct_transfer`.
#[must_use]
pub struct ContractCallWithMoaxOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payment: MoaxOrDctTokenPayment<SA>,
}

impl<SA, OriginalResult> ContractCallWithMoaxOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    fn into_normalized_moax(self) -> ContractCallWithMoax<SA, OriginalResult> {
        ContractCallWithMoax {
            basic: self.basic,
            moax_payment: self.payment.amount,
        }
    }

    fn into_normalized_dct(self) -> ContractCallWithMoax<SA, OriginalResult> {
        self.basic
            .into_normalized()
            .convert_to_single_transfer_dct_call(self.payment.unwrap_dct())
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithMoaxOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithMoax<SA, Self::OriginalResult> {
        if self.payment.token_identifier.is_moax() {
            self.into_normalized_moax()
        } else {
            // Because we know that there can be at most one DCT payment,
            // there is no need to call the full `convert_to_dct_transfer_call`.
            self.into_normalized_dct()
        }
    }

    #[inline]
    fn get_mut_basic(&mut self) -> &mut ContractCallNoPayment<SA, OriginalResult> {
        &mut self.basic
    }

    fn transfer_execute(self) {
        if self.payment.token_identifier.is_moax() {
            self.basic.transfer_execute_moax(self.payment.amount);
        } else {
            self.basic
                .transfer_execute_single_dct(self.payment.unwrap_dct());
        }
    }
}

impl<SA, OriginalResult> ContractCallWithMoaxOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_moax_or_single_dct_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        token_identifier: MoaxOrDctTokenIdentifier<SA>,
        token_nonce: u64,
        amount: BigUint<SA>,
    ) -> Self {
        ContractCallWithMoaxOrSingleDct {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment: MoaxOrDctTokenPayment::new(token_identifier, token_nonce, amount),
        }
    }
}
