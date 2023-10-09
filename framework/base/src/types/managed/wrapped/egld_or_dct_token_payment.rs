use crate::{
    api::ManagedTypeApi,
    types::{BigUint, EgldOrDctTokenIdentifier},
};

use crate::codec::{
    self,
    derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    CodecFrom, CodecFromSelf,
};

use crate as dharitri_sc; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

use super::DctTokenPayment;

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct EgldOrDctTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: EgldOrDctTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> EgldOrDctTokenPayment<M> {
    pub fn no_payment() -> Self {
        EgldOrDctTokenPayment {
            token_identifier: EgldOrDctTokenIdentifier::egld(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(
        token_identifier: EgldOrDctTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        EgldOrDctTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    /// Will convert to just DCT or terminate execution if the token is EGLD.
    pub fn unwrap_dct(self) -> DctTokenPayment<M> {
        DctTokenPayment::new(
            self.token_identifier.unwrap_dct(),
            self.token_nonce,
            self.amount,
        )
    }

    pub fn into_tuple(self) -> (EgldOrDctTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<(EgldOrDctTokenIdentifier<M>, u64, BigUint<M>)>
    for EgldOrDctTokenPayment<M>
{
    #[inline]
    fn from(value: (EgldOrDctTokenIdentifier<M>, u64, BigUint<M>)) -> Self {
        let (token_identifier, token_nonce, amount) = value;
        Self::new(token_identifier, token_nonce, amount)
    }
}

impl<M: ManagedTypeApi> From<DctTokenPayment<M>> for EgldOrDctTokenPayment<M> {
    fn from(dct_payment: DctTokenPayment<M>) -> Self {
        EgldOrDctTokenPayment {
            token_identifier: EgldOrDctTokenIdentifier::dct(dct_payment.token_identifier),
            token_nonce: dct_payment.token_nonce,
            amount: dct_payment.amount,
        }
    }
}

impl<M> CodecFromSelf for EgldOrDctTokenPayment<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for EgldOrDctTokenPayment<M> where M: ManagedTypeApi {}
