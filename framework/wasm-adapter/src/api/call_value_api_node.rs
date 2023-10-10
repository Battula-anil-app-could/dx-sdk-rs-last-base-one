use super::VmApiImpl;
use dharitri_sc::api::{CallValueApi, CallValueApiImpl};

extern "C" {
    fn checkNoPayment();

    fn bigIntGetCallValue(dest: i32);

    fn managedGetMultiDCTCallValue(resultHandle: i32);

    fn getNumDCTTransfers() -> i32;
}

impl CallValueApi for VmApiImpl {
    type CallValueApiImpl = VmApiImpl;

    #[inline]
    fn call_value_api_impl() -> Self::CallValueApiImpl {
        VmApiImpl {}
    }
}

impl CallValueApiImpl for VmApiImpl {
    #[inline]
    fn check_not_payable(&self) {
        unsafe {
            checkNoPayment();
        }
    }

    fn load_moax_value(&self, dest: Self::BigIntHandle) {
        unsafe {
            bigIntGetCallValue(dest);
        }
    }

    fn load_all_dct_transfers(&self, dest_handle: Self::ManagedBufferHandle) {
        unsafe {
            managedGetMultiDCTCallValue(dest_handle);
        }
    }

    fn dct_num_transfers(&self) -> usize {
        unsafe { getNumDCTTransfers() as usize }
    }
}
