use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/digital-cash");

    blockchain.register_contract(
        "file:output/digital-cash.wasm",
        digital_cash::ContractBuilder,
    );
    blockchain
}

#[test]
fn claim_egld_rs() {
    world().run("scenarios/claim-egld.scen.json");
}

#[test]
fn claim_dct_rs() {
    world().run("scenarios/claim-dct.scen.json");
}

#[test]
fn claim_fees_rs() {
    world().run("scenarios/claim-fees.scen.json");
}

#[test]
fn claim_multi_dct_rs() {
    world().run("scenarios/claim-multi-dct.scen.json");
}

#[test]
fn forward_rs() {
    world().run("scenarios/forward.scen.json");
}

#[test]
fn fund_egld_and_dct_rs() {
    world().run("scenarios/fund-egld-and-dct.scen.json");
}

#[test]
fn set_accounts_rs() {
    world().run("scenarios/set-accounts.scen.json");
}

#[test]
fn whitelist_blacklist_fee_token_rs() {
    world().run("scenarios/whitelist-blacklist-fee-tokens.scen.json");
}

#[test]
fn pay_fee_and_fund_dct_rs() {
    world().run("scenarios/pay-fee-and-fund-dct.scen.json");
}

#[test]
fn pay_fee_and_fund_egld_rs() {
    world().run("scenarios/pay-fee-and-fund-egld.scen.json");
}

#[test]
fn withdraw_egld_rs() {
    world().run("scenarios/withdraw-egld.scen.json");
}

#[test]
fn withdraw_dct_rs() {
    world().run("scenarios/withdraw-dct.scen.json");
}

#[test]
fn withdraw_multi_dct_rs() {
    world().run("scenarios/withdraw-multi-dct.scen.json");
}
