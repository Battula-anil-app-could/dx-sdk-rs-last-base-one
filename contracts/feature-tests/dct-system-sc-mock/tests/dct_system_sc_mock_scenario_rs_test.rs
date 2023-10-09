use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    todo!()
}

#[test]
#[ignore = "builtin SC not implemented"]
fn dct_system_sc_rs() {
    world().run("scenarios/dct_system_sc.scen.json");
}
