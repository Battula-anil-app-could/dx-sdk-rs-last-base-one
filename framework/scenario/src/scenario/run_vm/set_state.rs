use crate::scenario::model::SetStateStep;

use dharitri_chain_vm::{
    types::VMAddress,
    world_mock::{
        AccountData, AccountDct, BlockInfo as CrateBlockInfo, BlockchainState, DctData,
        DctInstance, DctInstanceMetadata, DctInstances, DctRoles,
    },
};

use super::ScenarioVMRunner;

impl ScenarioVMRunner {
    pub fn perform_set_state(&mut self, set_state_step: &SetStateStep) {
        execute(&mut self.blockchain_mock.state, set_state_step);
    }
}

fn execute(state: &mut BlockchainState, set_state_step: &SetStateStep) {
    for (address, account) in set_state_step.accounts.iter() {
        let storage = account
            .storage
            .iter()
            .map(|(k, v)| (k.value.clone(), v.value.clone()))
            .collect();
        let dct = AccountDct::new_from_raw_map(
            account
                .dct
                .iter()
                .map(|(k, v)| (k.value.clone(), convert_mandos_dct_to_world_mock(v)))
                .collect(),
        );

        state.validate_and_add_account(AccountData {
            address: address.to_vm_address(),
            nonce: account
                .nonce
                .as_ref()
                .map(|nonce| nonce.value)
                .unwrap_or_default(),
            moax_balance: account
                .balance
                .as_ref()
                .map(|balance| balance.value.clone())
                .unwrap_or_default(),
            dct,
            username: account
                .username
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone())
                .unwrap_or_default(),
            storage,
            contract_path: account
                .code
                .as_ref()
                .map(|bytes_value| bytes_value.value.clone()),
            contract_owner: account
                .owner
                .as_ref()
                .map(|address_value| address_value.to_vm_address()),
            developer_rewards: account
                .developer_rewards
                .as_ref()
                .map(|rewards| rewards.value.clone())
                .unwrap_or_default(),
        });
    }
    for new_address in set_state_step.new_addresses.iter() {
        assert!(
            new_address.new_address.value.is_smart_contract_address(),
            "field should have SC format"
        );
        state.put_new_address(
            new_address.creator_address.to_vm_address(),
            new_address.creator_nonce.value,
            new_address.new_address.to_vm_address(),
        )
    }
    for new_token_identifier in set_state_step.new_token_identifiers.iter().cloned() {
        state.put_new_token_identifier(new_token_identifier)
    }
    if let Some(block_info_obj) = &*set_state_step.previous_block_info {
        update_block_info(&mut state.previous_block_info, block_info_obj);
    }
    if let Some(block_info_obj) = &*set_state_step.current_block_info {
        update_block_info(&mut state.current_block_info, block_info_obj);
    }
}

fn convert_mandos_dct_to_world_mock(mandos_dct: &crate::scenario::model::Dct) -> DctData {
    match mandos_dct {
        crate::scenario::model::Dct::Short(short_dct) => {
            let balance = short_dct.value.clone();
            let mut dct_data = DctData::default();
            dct_data.instances.add(0, balance);
            dct_data
        },
        crate::scenario::model::Dct::Full(full_dct) => DctData {
            instances: DctInstances::new_from_hash(
                full_dct
                    .instances
                    .iter()
                    .map(|mandos_instance| {
                        let mock_instance =
                            convert_scenario_dct_instance_to_world_mock(mandos_instance);
                        (mock_instance.nonce, mock_instance)
                    })
                    .collect(),
            ),
            last_nonce: full_dct
                .last_nonce
                .as_ref()
                .map(|last_nonce| last_nonce.value)
                .unwrap_or_default(),
            roles: DctRoles::new(
                full_dct
                    .roles
                    .iter()
                    .map(|role| role.as_bytes().to_vec())
                    .collect(),
            ),
            frozen: if let Some(u64_value) = &full_dct.frozen {
                u64_value.value > 0
            } else {
                false
            },
        },
    }
}

fn convert_scenario_dct_instance_to_world_mock(
    scenario_dct: &crate::scenario::model::DctInstance,
) -> DctInstance {
    DctInstance {
        nonce: scenario_dct
            .nonce
            .as_ref()
            .map(|nonce| nonce.value)
            .unwrap_or_default(),
        balance: scenario_dct
            .balance
            .as_ref()
            .map(|value| value.value.clone())
            .unwrap_or_default(),
        metadata: DctInstanceMetadata {
            name: Vec::new(),
            creator: scenario_dct
                .creator
                .as_ref()
                .map(|creator| VMAddress::from_slice(creator.value.as_slice())),
            royalties: scenario_dct
                .royalties
                .as_ref()
                .map(|royalties| royalties.value)
                .unwrap_or_default(),
            hash: scenario_dct.hash.as_ref().map(|hash| hash.value.clone()),
            uri: scenario_dct
                .uri
                .iter()
                .map(|uri| uri.value.clone())
                .collect(),
            attributes: scenario_dct
                .attributes
                .as_ref()
                .map(|attributes| attributes.value.clone())
                .unwrap_or_default(),
        },
    }
}

fn update_block_info(
    block_info: &mut CrateBlockInfo,
    scenario_block_info: &crate::scenario::model::BlockInfo,
) {
    if let Some(u64_value) = &scenario_block_info.block_timestamp {
        block_info.block_timestamp = u64_value.value;
    }
    if let Some(u64_value) = &scenario_block_info.block_nonce {
        block_info.block_nonce = u64_value.value;
    }
    if let Some(u64_value) = &scenario_block_info.block_epoch {
        block_info.block_epoch = u64_value.value;
    }
    if let Some(u64_value) = &scenario_block_info.block_round {
        block_info.block_round = u64_value.value;
    }
    if let Some(bytes_value) = &scenario_block_info.block_random_seed {
        const SEED_LEN: usize = 48;
        let val = &bytes_value.value;

        assert!(
            val.len() == SEED_LEN,
            "block random seed input value must be exactly 48 bytes long"
        );

        let mut seed = [0u8; SEED_LEN];
        seed[..].copy_from_slice(val.as_slice());
        block_info.block_random_seed = Box::from(seed);
    }
}
