use bip39::Mnemonic;

use dharitri_sdk::{crypto::public_key::PublicKey, data::address::Address, wallet::Wallet};

#[test]
fn test_private_key_from_mnemonic() {
    let mnemonic: Mnemonic = Mnemonic::parse_normalized("acid twice post genre topic observe apple viable gesture fortune funny dawn around blood enemy page update reduce decline van bundle zebra rookie real").unwrap();

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic.clone(), 0, 0);
    let public_key = PublicKey::from(&private_key);
    let address = Address::from(&public_key);
    assert_eq!(
        "c6ecd5ee6ae3ed006d3033093d6c3ca1d28566fc87beaad387ab67bb2b259057",
        private_key.to_string()
    );
    assert_eq!(
        "1e152cda4a8babddb6750c6ba48b3849719d9453ab323b89d75ba7b85b1ef431",
        public_key.to_string()
    );
    assert_eq!(
        "moa1rc2jekj23w4amdn4p346fzecf9cem9zn4verhzwhtwnmskc77scsjt74t7",
        address.to_string()
    );

    let private_key = Wallet::get_private_key_from_mnemonic(mnemonic, 0, 1);
    let public_key = PublicKey::from(&private_key);
    let address = Address::from(&public_key);
    assert_eq!(
        "db6140e6f0854ad8a0b10b3e50e94fb1259b60bb0d7c2665d597eb1970f7a3a9",
        private_key.to_string()
    );
    assert_eq!(
        "413885ecb0b058f51db1362568a6d156c4fa6d310debf2cfaa94365ab93d9f74",
        public_key.to_string()
    );
    assert_eq!(
        "moa1gyugtm9skpv028d3xcjk3fk32mz05mf3ph4l9na2jsm94wfana6qt9gfd5",
        address.to_string()
    );
}

#[test]
fn test_load_from_pem() {
    let wallet = Wallet::from_pem_file("tests/alice.pem").unwrap();
    eprintln!("wallte is {:?}", wallet);
    let addr = wallet.address();
    assert_eq!(
        addr.to_bech32_string().unwrap(),
        "moa1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssfq94h8"
    );
}
