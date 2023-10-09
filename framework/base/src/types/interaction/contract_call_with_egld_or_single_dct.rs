use crate::codec::TopEncodeMulti;

use crate::{
    api::CallTypeApi,
    types::{
        BigUint, EgldOrDctTokenIdentifier, EgldOrDctTokenPayment, ManagedAddress, ManagedBuffer,
    },
};

use super::{contract_call_no_payment::ContractCallNoPayment, ContractCall, ContractCallWithEgld};

/// Holds data for calling another contract, with a single payment, either EGLD or a single DCT token.
///
/// Gets created when chaining method `with_egld_or_single_dct_transfer`.
#[must_use]
pub struct ContractCallWithEgldOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
{
    pub(super) basic: ContractCallNoPayment<SA, OriginalResult>,
    pub payment: EgldOrDctTokenPayment<SA>,
}

impl<SA, OriginalResult> ContractCallWithEgldOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    fn into_normalized_egld(self) -> ContractCallWithEgld<SA, OriginalResult> {
        ContractCallWithEgld {
            basic: self.basic,
            egld_payment: self.payment.amount,
        }
    }

    fn into_normalized_dct(self) -> ContractCallWithEgld<SA, OriginalResult> {
        self.basic
            .into_normalized()
            .convert_to_single_transfer_dct_call(self.payment.unwrap_dct())
    }
}

impl<SA, OriginalResult> ContractCall<SA> for ContractCallWithEgldOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    type OriginalResult = OriginalResult;

    fn into_normalized(self) -> ContractCallWithEgld<SA, Self::OriginalResult> {
        if self.payment.token_identifier.is_egld() {
            self.into_normalized_egld()
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
        if self.payment.token_identifier.is_egld() {
            self.basic.transfer_execute_egld(self.payment.amount);
        } else {
            self.basic
                .transfer_execute_single_dct(self.payment.unwrap_dct());
        }
    }
}

impl<SA, OriginalResult> ContractCallWithEgldOrSingleDct<SA, OriginalResult>
where
    SA: CallTypeApi + 'static,
    OriginalResult: TopEncodeMulti,
{
    /// Creates a new instance directly.
    ///
    /// The constructor is mostly for hand-written proxies,
    /// the usual way of constructing this object is via the builder methods of other contract call types,
    /// especially `with_egld_or_single_dct_transfer`.
    pub fn new<N: Into<ManagedBuffer<SA>>>(
        to: ManagedAddress<SA>,
        endpoint_name: N,
        token_identifier: EgldOrDctTokenIdentifier<SA>,
        token_nonce: u64,
        amount: BigUint<SA>,
    ) -> Self {
        ContractCallWithEgldOrSingleDct {
            basic: ContractCallNoPayment::new(to, endpoint_name),
            payment: EgldOrDctTokenPayment::new(token_identifier, token_nonce, amount),
        }
    }
}
