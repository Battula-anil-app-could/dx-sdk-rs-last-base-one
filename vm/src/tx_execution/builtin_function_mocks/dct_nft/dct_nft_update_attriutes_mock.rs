use crate::{
    tx_execution::{builtin_function_names::DCT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME, BlockchainVMRef},
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
    types::{top_decode_u64, top_encode_u64},
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct DCTNftUpdateAttributes;

impl BuiltinFunction for DCTNftUpdateAttributes {
    fn name(&self) -> &str {
        DCT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        _vm: &BlockchainVMRef,
        _f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: FnOnce(),
    {
        if tx_input.args.len() != 3 {
            let err_result = TxResult::from_vm_error("DCTNFTUpdateAttributes expects 3 arguments");
            return (err_result, BlockchainUpdate::empty());
        }

        let token_identifier = tx_input.args[0].as_slice();
        let nonce = top_decode_u64(tx_input.args[1].as_slice());
        let attributes_bytes = tx_input.args[2].as_slice();

        tx_cache.with_account_mut(&tx_input.from, |account| {
            account
                .dct
                .update_attributes(token_identifier, nonce, attributes_bytes.to_vec());
        });

        let dct_nft_create_log = TxLog {
            address: tx_input.from,
            endpoint: DCT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME.into(),
            topics: vec![
                token_identifier.to_vec(),
                top_encode_u64(nonce),
                Vec::new(), // value = 0
                attributes_bytes.to_vec(),
            ],
            data: vec![],
        };

        let tx_result = TxResult {
            result_status: 0,
            result_logs: vec![dct_nft_create_log],
            ..Default::default()
        };

        (tx_result, tx_cache.into_blockchain_updates())
    }
}
