use crate::{
    abi::{TypeAbi, TypeName},
    api::{HandleConstraints, ManagedTypeApi},
    codec::*,
    derive::ManagedVecItem,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    types::{ManagedBuffer, ManagedOption, ManagedRef, ManagedType, TokenIdentifier},
};

use crate as dharitri_sc; // required by the ManagedVecItem derive

/// Specialized type for handling either EGLD or DCT token identifiers.
///
/// Equivalent to a structure of the form
/// ```
/// # use dharitri_sc::{api::ManagedTypeApi, types::TokenIdentifier};
/// enum EgldOrDctTokenIdentifier<M: ManagedTypeApi> {
///     Egld,
///     Dct(TokenIdentifier<M>),
/// }
/// ```
///
/// It is, however more optimized than that. Its implementation is based on `ManagedOption`.
///
/// EGLD a special, invalid token identifier handle. This way we can fit it inside a single i32 in memory.
#[repr(transparent)]
#[derive(ManagedVecItem, Clone)]
pub struct EgldOrDctTokenIdentifier<M: ManagedTypeApi> {
    data: ManagedOption<M, TokenIdentifier<M>>,
}

impl<M: ManagedTypeApi> EgldOrDctTokenIdentifier<M> {
    /// This special representation is interpreted as the EGLD token.
    #[allow(clippy::needless_borrow)] // clippy is wrog here, there is no other way
    pub const EGLD_REPRESENTATION: &'static [u8; 4] = &b"EGLD";

    /// New instance of the special EGLD token representation.
    #[inline]
    pub fn egld() -> Self {
        Self {
            data: ManagedOption::none(),
        }
    }

    /// DCT instance, containing an DCT token identifier.
    #[inline]
    pub fn dct<TI>(token_identifier: TI) -> Self
    where
        TokenIdentifier<M>: From<TI>,
    {
        Self {
            data: ManagedOption::some(TokenIdentifier::from(token_identifier)),
        }
    }

    pub fn from_opt_raw_handle(opt_handle: Option<M::ManagedBufferHandle>) -> Self {
        match opt_handle {
            Some(handle) => Self::dct(TokenIdentifier::from_handle(handle)),
            None => Self::egld(),
        }
    }

    pub fn parse(data: ManagedBuffer<M>) -> Self {
        if data == Self::EGLD_REPRESENTATION {
            Self::egld()
        } else {
            Self::dct(TokenIdentifier::from(data))
        }
    }

    #[inline]
    pub fn is_egld(&self) -> bool {
        self.data.is_none()
    }

    #[inline]
    pub fn is_dct(&self) -> bool {
        self.data.is_some()
    }

    #[inline]
    pub fn into_name(self) -> ManagedBuffer<M> {
        self.map_or_else(
            || ManagedBuffer::from(&Self::EGLD_REPRESENTATION[..]),
            |token_identifier| token_identifier.into_managed_buffer(),
        )
    }

    /// Checks the DCT token identifier for validity. EGLD is considered valid, no checks needed.
    ///
    /// Will fail if it encodes an invalid DCT token identifier.
    pub fn is_valid(&self) -> bool {
        self.map_ref_or_else(
            || true,
            |token_identifier| token_identifier.is_valid_dct_identifier(),
        )
    }

    pub fn map_or_else<U, D, F>(self, for_egld: D, for_dct: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(TokenIdentifier<M>) -> U,
    {
        self.data.map_or_else(for_egld, for_dct)
    }

    pub fn map_ref_or_else<U, D, F>(&self, for_egld: D, for_dct: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(&TokenIdentifier<M>) -> U,
    {
        self.data.map_ref_or_else(for_egld, for_dct)
    }

    pub fn unwrap_dct(self) -> TokenIdentifier<M> {
        self.data.unwrap_or_sc_panic("DCT expected")
    }

    /// Representation of the object as an `Option`.
    ///
    /// Because it does not consume `self` only a reference to the DCT token identifier can be returned.
    pub fn as_dct_option(&self) -> Option<ManagedRef<'_, M, TokenIdentifier<M>>> {
        self.data.as_option()
    }

    /// Converts `self` into an `Option`. Consumes `self` in the process.
    pub fn into_dct_option(self) -> Option<TokenIdentifier<M>> {
        self.data.into_option()
    }
}

impl<M: ManagedTypeApi> PartialEq for EgldOrDctTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<M: ManagedTypeApi> Eq for EgldOrDctTokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<TokenIdentifier<M>> for EgldOrDctTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &TokenIdentifier<M>) -> bool {
        self.map_ref_or_else(
            || false,
            |self_dct_token_identifier| self_dct_token_identifier == other,
        )
    }
}

impl<M: ManagedTypeApi> NestedEncode for EgldOrDctTokenIdentifier<M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if let Some(token_identifier) = self.data.as_option() {
            token_identifier.dep_encode_or_handle_err(dest, h)
        } else {
            (&Self::EGLD_REPRESENTATION[..]).dep_encode_or_handle_err(dest, h)
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for EgldOrDctTokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if let Some(token_identifier) = self.data.as_option() {
            token_identifier.top_encode_or_handle_err(output, h)
        } else {
            (&Self::EGLD_REPRESENTATION[..]).top_encode_or_handle_err(output, h)
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for EgldOrDctTokenIdentifier<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::parse(ManagedBuffer::dep_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M: ManagedTypeApi> TopDecode for EgldOrDctTokenIdentifier<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::parse(ManagedBuffer::top_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M> CodecFromSelf for EgldOrDctTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<TokenIdentifier<M>> for EgldOrDctTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> CodecFrom<&TokenIdentifier<M>> for EgldOrDctTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for EgldOrDctTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> CodecFrom<&str> for EgldOrDctTokenIdentifier<M> where M: ManagedTypeApi {}

impl<M: ManagedTypeApi> TypeAbi for EgldOrDctTokenIdentifier<M> {
    fn type_name() -> TypeName {
        "EgldOrDctTokenIdentifier".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for EgldOrDctTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer(&ManagedBuffer::from_handle(
                token_identifier.get_handle().cast_or_signal_error::<M, _>(),
            ));
        } else {
            f.append_bytes(Self::EGLD_REPRESENTATION);
        }
    }
}

const EGLD_REPRESENTATION_HEX: &[u8] = b"45474C44";

impl<M: ManagedTypeApi> SCLowerHex for EgldOrDctTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer_lower_hex(&ManagedBuffer::from_handle(
                token_identifier.get_handle().cast_or_signal_error::<M, _>(),
            ));
        } else {
            f.append_bytes(EGLD_REPRESENTATION_HEX);
        }
    }
}

impl<M> core::fmt::Debug for EgldOrDctTokenIdentifier<M>
where
    M: ManagedTypeApi,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::alloc::string::ToString;
        if let Some(token_identifier) = self.data.as_option() {
            let token_id_str = token_identifier.to_string();
            f.debug_tuple("EgldOrDctTokenIdentifier::Dct")
                .field(&token_id_str)
                .finish()
        } else {
            f.write_str("EgldOrDctTokenIdentifier::Egld")
        }
    }
}
