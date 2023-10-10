use dharitri_sdk::{
    blockchain::{CommunicationProxy, DEVNET_GATEWAY},
    data::{address::Address, vm::VmValueRequest},
    wallet::Wallet,
};

#[tokio::main]
async fn main() {
    let wl = Wallet::from_private_key(
        "db6140e6f0854ad8a0b10b3e50e94fb1259b60bb0d7c2665d597eb1970f7a3a9",
    )
    .unwrap();
    let addr = wl.address();
    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let req = VmValueRequest {
        sc_address: Address::from_bech32_string(
            "erd1qqqqqqqqqqqqqpgqhn3ae8dpc957t7jadn7kywtg503dy7pnj9ts3umqxx",
        )
        .unwrap(),
        func_name: "get".to_string(),
        args: vec![],
        caller: addr.clone(),
        value: "0".to_string(),
    };
    let result = blockchain.execute_vmquery(&req).await;
    println!("{result:#?}");
}
