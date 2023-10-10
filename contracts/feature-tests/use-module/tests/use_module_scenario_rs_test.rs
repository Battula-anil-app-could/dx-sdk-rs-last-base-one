mod user_builtin {
    dharitri_sc::imports!();

    #[dharitri_sc::proxy]
    pub trait UserBuiltin {
        #[endpoint(SetUserName)]
        fn set_user_name(&self, name: &BoxedBytes) -> BigUint;
    }
}

mod dns_mock {
    dharitri_sc::imports!();

    #[dharitri_sc::contract]
    pub trait DnsMock {
        #[proxy]
        fn user_builtin_proxy(&self, to: ManagedAddress) -> super::user_builtin::Proxy<Self::Api>;

        #[payable("MOAX")]
        #[endpoint]
        fn register(&self, name: BoxedBytes) {
            let _payment = self.call_value().moax_value();
            let address = self.blockchain().get_caller();
            self.user_builtin_proxy(address)
                .set_user_name(&name)
                .async_call()
                .call_and_exit()
        }
    }
}

use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract("file:output/use-module.wasm", use_module::ContractBuilder);

    blockchain.register_contract(
        "file:test-wasm/dharitri-wasm-sc-dns.wasm",
        dns_mock::ContractBuilder,
    );

    blockchain
}

#[test]
fn use_module_claim_developer_rewards_rs() {
    world().run("scenarios/use_module_claim_developer_rewards.scen.json");
}

#[test]
fn use_module_dns_register_rs() {
    world().run("scenarios/use_module_dns_register.scen.json");
}

#[test]
fn use_module_features_rs() {
    world().run("scenarios/use_module_features.scen.json");
}

#[test]
fn use_module_internal_rs() {
    world().run("scenarios/use_module_internal.scen.json");
}

#[test]
fn use_module_no_endpoint_rs() {
    world().run("scenarios/use_module_no_endpoint.scen.json");
}

/// Will not work in scenarios-rs, since there is no gas usage
#[test]
#[ignore]
fn use_module_ongoing_operation_example_rs() {
    world().run("scenarios/use_module_ongoing_operation_example.scen.json");
}

#[test]
fn use_module_only_admin_rs() {
    world().run("scenarios/use_module_only_admin.scen.json");
}

#[test]
fn use_module_only_owner_rs() {
    world().run("scenarios/use_module_only_owner.scen.json");
}

#[test]
fn use_module_pause_rs() {
    world().run("scenarios/use_module_pause.scen.json");
}
