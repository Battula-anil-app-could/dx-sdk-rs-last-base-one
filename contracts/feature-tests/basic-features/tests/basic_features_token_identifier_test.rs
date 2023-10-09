use dharitri_sc::types::{EgldOrDctTokenIdentifier, ManagedBuffer, TokenIdentifier};
use dharitri_sc_scenario::{api::StaticApi, *};

use basic_features::token_identifier_features::TokenIdentifierFeatures;

#[test]
fn test_token_identifier_egld() {
    let bf = basic_features::contract_obj::<StaticApi>();
    let result = bf.token_identifier_egld();
    assert_eq!(EgldOrDctTokenIdentifier::egld(), result);
}

/// This just tests the contract syntax.
/// For a complete suite of test cases, see `dharitri-sc-scenario/tests/managed_token_identifier_test.rs`.
#[test]
fn test_token_identifier_is_valid() {
    let bf = basic_features::contract_obj::<StaticApi>();
    let result = bf.token_identifier_is_valid_1(EgldOrDctTokenIdentifier::dct(
        TokenIdentifier::from(&b"ALC-6258d2"[..]),
    ));
    assert!(result);
    let result = bf.token_identifier_is_valid_1(EgldOrDctTokenIdentifier::dct(
        TokenIdentifier::from(&b"AL-C6258d2"[..]),
    ));
    assert!(!result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::from(&b"12345-6258d2"[..]));
    assert!(result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::from(&b"ALCCCCCCCCC-6258d2"[..]));
    assert!(!result);
}

#[test]
fn test_token_identifier_compare() {
    let token_id = TokenIdentifier::<StaticApi>::from(&b"ALC-6258d2"[..]);
    let dct_token_id = EgldOrDctTokenIdentifier::dct(token_id.clone());
    let wrong_dct_token_id =
        EgldOrDctTokenIdentifier::dct(TokenIdentifier::from(&b"AAA-111111"[..]));
    let egld_token_id = EgldOrDctTokenIdentifier::egld();

    assert!(token_id == dct_token_id);
    assert!(dct_token_id == token_id);

    assert!(token_id != egld_token_id);
    assert!(egld_token_id != token_id);

    assert!(token_id != wrong_dct_token_id);
    assert!(wrong_dct_token_id != token_id);
}
