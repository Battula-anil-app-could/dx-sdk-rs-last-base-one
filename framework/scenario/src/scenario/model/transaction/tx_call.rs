use crate::{
    api::StaticApi,
    dharitri_sc::types::{ContractCall, ContractCallWithMoax, DctTokenPayment},
    scenario::model::{AddressValue, BigUintValue, BytesValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::TxCallRaw,
    },
};

use super::{tx_interpret_util::interpret_moax_value, TxDCT};

pub const DEFAULT_GAS_EXPR: &str = "5,000,000";

#[derive(Debug, Clone)]
pub struct TxCall {
    pub from: AddressValue,
    pub to: AddressValue,
    pub moax_value: BigUintValue,
    pub dct_value: Vec<TxDCT>,
    pub function: String,
    pub arguments: Vec<BytesValue>,
    pub gas_limit: U64Value,
    pub gas_price: U64Value,
}

impl Default for TxCall {
    fn default() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            moax_value: Default::default(),
            dct_value: Default::default(),
            function: Default::default(),
            arguments: Default::default(),
            gas_limit: U64Value::from(DEFAULT_GAS_EXPR),
            gas_price: Default::default(),
        }
    }
}

impl InterpretableFrom<TxCallRaw> for TxCall {
    fn interpret_from(from: TxCallRaw, context: &InterpreterContext) -> Self {
        TxCall {
            from: AddressValue::interpret_from(from.from, context),
            to: AddressValue::interpret_from(from.to, context),
            moax_value: interpret_moax_value(from.value, from.moax_value, context),
            dct_value: from
                .dct_value
                .into_iter()
                .map(|dct_value| TxDCT::interpret_from(dct_value, context))
                .collect(),
            function: from.function,
            arguments: from
                .arguments
                .into_iter()
                .map(|t| BytesValue::interpret_from(t, context))
                .collect(),
            gas_limit: U64Value::interpret_from(from.gas_limit, context),
            gas_price: U64Value::interpret_from(from.gas_price, context),
        }
    }
}

impl IntoRaw<TxCallRaw> for TxCall {
    fn into_raw(self) -> TxCallRaw {
        TxCallRaw {
            from: self.from.into_raw(),
            to: self.to.into_raw(),
            value: None,
            moax_value: self.moax_value.into_raw_opt(),
            dct_value: self
                .dct_value
                .into_iter()
                .map(|dct_value| dct_value.into_raw())
                .collect(),
            function: self.function,
            arguments: self
                .arguments
                .into_iter()
                .map(|arg| arg.into_raw())
                .collect(),
            gas_limit: self.gas_limit.into_raw(),
            gas_price: self.gas_price.into_raw(),
        }
    }
}

impl TxCall {
    pub fn to_contract_call(&self) -> ContractCallWithMoax<StaticApi, ()> {
        let mut contract_call = ContractCallWithMoax::new(
            (&self.to.value).into(),
            self.function.as_bytes(),
            (&self.moax_value.value).into(),
        );

        contract_call.basic.explicit_gas_limit = self.gas_limit.value;

        contract_call = contract_call.convert_to_dct_transfer_call(
            self.dct_value
                .iter()
                .map(|dct| {
                    DctTokenPayment::new(
                        dct.dct_token_identifier.value.as_slice().into(),
                        dct.nonce.value,
                        (&dct.dct_value.value).into(),
                    )
                })
                .collect(),
        );

        // For some contract calls from == to.
        // The contract call objects have no "from" field, since that is always part of the execution context.
        // On the static API there is no execution context, but a placeholder value is provided.
        // Here we already know the sender, so we can replace the placeholder with the actual value.
        if StaticApi::is_current_address_placeholder(&contract_call.basic.to.to_address()) {
            contract_call.basic.to = self.from.value.clone().into();
        }

        for argument in &self.arguments {
            contract_call.push_raw_argument(argument.value.as_slice());
        }
        contract_call
    }
}
