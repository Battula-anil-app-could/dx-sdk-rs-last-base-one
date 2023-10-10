use super::{ErrorApiImpl, HandleTypeInfo, ManagedTypeApiImpl};

pub trait CallValueApi: HandleTypeInfo {
    type CallValueApiImpl: CallValueApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn call_value_api_impl() -> Self::CallValueApiImpl;
}

pub trait CallValueApiImpl: ErrorApiImpl + ManagedTypeApiImpl + Sized {
    fn check_not_payable(&self);

    /// Retrieves the MOAX call value from the VM.
    /// Will return 0 in case of an DCT transfer (cannot have both MOAX and DCT transfer simultaneously).
    fn load_moax_value(&self, dest_handle: Self::BigIntHandle);

    /// Loads all DCT call values into a managed vec. Overwrites destination.
    fn load_all_dct_transfers(&self, dest_handle: Self::ManagedBufferHandle);

    /// Gets the total number of DCT transfers (Fungible/SFT/NFT).
    ///
    /// It is redundant, since the number can also be retrieved from `load_all_dct_transfers`,
    /// but it is easier and cheaper to call when the content of those transfers is of no interest.
    fn dct_num_transfers(&self) -> usize;
}
