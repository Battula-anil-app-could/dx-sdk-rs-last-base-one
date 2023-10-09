use dharitri_sc::types::{
    BoxedBytes, EgldOrDctTokenIdentifier, EgldOrDctTokenPayment, DctTokenPayment, ManagedBuffer,
    TokenIdentifier,
};
use dharitri_sc_scenario::{
    api::StaticApi, managed_egld_token_id, managed_test_util::check_managed_top_encode_decode,
    managed_token_id, managed_token_id_wrapped, dharitri_sc,
};

#[test]
fn test_egld() {
    assert!(EgldOrDctTokenIdentifier::<StaticApi>::egld().is_egld());
}

#[test]
fn test_codec() {
    check_managed_top_encode_decode(
        EgldOrDctTokenIdentifier::<StaticApi>::egld(),
        EgldOrDctTokenIdentifier::<StaticApi>::EGLD_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &EgldOrDctTokenIdentifier::<StaticApi>::EGLD_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        vec![EgldOrDctTokenIdentifier::<StaticApi>::egld()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dct_identifier() {
    // valid identifier
    assert!(TokenIdentifier::<StaticApi>::from("ALC-6258d2").is_valid_dct_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<StaticApi>::from("ALC123-6258d2").is_valid_dct_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<StaticApi>::from("12345-6258d2").is_valid_dct_identifier());

    // missing dash
    assert!(!TokenIdentifier::<StaticApi>::from("ALC6258d2").is_valid_dct_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<StaticApi>::from("AL-C6258d2").is_valid_dct_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<StaticApi>::from("alc-6258d2").is_valid_dct_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<StaticApi>::from("ALC-6258D2").is_valid_dct_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<StaticApi>::from("ALC-6258d2ff").is_valid_dct_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<StaticApi>::from("AL-6258d2").is_valid_dct_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258d2").is_valid_dct_identifier());
}

#[test]
#[rustfmt::skip]
fn test_ticker() {
    // valid identifier
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // valid identifier with numbers in ticker
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC123-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC123"),
    );

    // valid ticker only numbers
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("12345-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("12345"),
    );

    // missing dash
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // wrong dash position
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("AL-C6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL-"),
    );

    // lowercase ticker
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("alc-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("alc"),
    );

    // uppercase random chars
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258D2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC"),
    );

    // too many random chars
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALC-6258d2ff").ticker(),
        ManagedBuffer::<StaticApi>::from("ALC-6"),
    );

    // ticker too short
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("AL-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("AL"),
    );

    // ticker too long
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("ALCCCCCCCCC-6258d2").ticker(),
        ManagedBuffer::<StaticApi>::from("ALCCCCCCCCC"),
    );
}

#[test]
fn test_is_valid_egld_or_dct() {
    // egld is always valid
    assert!(EgldOrDctTokenIdentifier::<StaticApi>::egld().is_valid());

    // valid dct
    assert!(
        EgldOrDctTokenIdentifier::<StaticApi>::dct(TokenIdentifier::from("ALC-6258d2"))
            .is_valid()
    );

    // invalid dct, see above
    assert!(
        !EgldOrDctTokenIdentifier::<StaticApi>::dct(TokenIdentifier::from("ALCCCCCCCCC-6258d2"))
            .is_valid()
    );
}

#[test]
fn test_token_identifier_eq() {
    assert_eq!(
        TokenIdentifier::<StaticApi>::from("DCT-00000"),
        TokenIdentifier::<StaticApi>::from("DCT-00000")
    );
    assert_ne!(
        TokenIdentifier::<StaticApi>::from("DCT-00001"),
        TokenIdentifier::<StaticApi>::from("DCT-00002")
    );

    assert_eq!(
        EgldOrDctTokenIdentifier::<StaticApi>::dct(TokenIdentifier::from("DCT-00003")),
        TokenIdentifier::<StaticApi>::from("DCT-00003")
    );
    assert_ne!(
        EgldOrDctTokenIdentifier::<StaticApi>::egld(),
        TokenIdentifier::<StaticApi>::from("ANYTHING-1234")
    );
    assert_ne!(
        EgldOrDctTokenIdentifier::<StaticApi>::egld(),
        TokenIdentifier::<StaticApi>::from("EGLD")
    );
}

#[test]
fn test_payment_eq() {
    assert_eq!(
        DctTokenPayment::<StaticApi>::new("PAY-00000".into(), 0, 1000u32.into()),
        DctTokenPayment::<StaticApi>::new("PAY-00000".into(), 0, 1000u32.into()),
    );
    assert_ne!(
        DctTokenPayment::<StaticApi>::new("PAY-00001".into(), 0, 1000u32.into()),
        DctTokenPayment::<StaticApi>::new("PAY-00002".into(), 0, 1000u32.into()),
    );
    assert_eq!(
        EgldOrDctTokenPayment::<StaticApi>::no_payment(),
        EgldOrDctTokenPayment::<StaticApi>::no_payment(),
    );
    assert_eq!(
        EgldOrDctTokenPayment::<StaticApi>::new(
            EgldOrDctTokenIdentifier::dct("DCTPAY-00000"),
            0,
            1000u32.into()
        ),
        EgldOrDctTokenPayment::<StaticApi>::new(
            EgldOrDctTokenIdentifier::dct("DCTPAY-00000"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        EgldOrDctTokenPayment::<StaticApi>::new(
            EgldOrDctTokenIdentifier::dct("DCTPAY-00001"),
            0,
            1000u32.into()
        ),
        EgldOrDctTokenPayment::<StaticApi>::new(
            EgldOrDctTokenIdentifier::dct("DCTPAY-00002"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        EgldOrDctTokenPayment::<StaticApi>::new(
            EgldOrDctTokenIdentifier::dct("DCTPAY-00001"),
            0,
            1000u32.into()
        ),
        EgldOrDctTokenPayment::<StaticApi>::no_payment(),
    );
}

#[test]
fn test_managed_token_id_macro() {
    assert_eq!(
        managed_egld_token_id!(),
        EgldOrDctTokenIdentifier::<StaticApi>::egld()
    );
    assert_eq!(
        managed_token_id!(b"ALC-6258d2"),
        TokenIdentifier::<StaticApi>::from("ALC-6258d2")
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258d2").unwrap_dct(),
        TokenIdentifier::<StaticApi>::from("ALC-6258d2")
    )
}
