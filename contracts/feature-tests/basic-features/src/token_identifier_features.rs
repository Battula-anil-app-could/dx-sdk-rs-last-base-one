dharitri_sc::imports!();

#[dharitri_sc::module]
pub trait TokenIdentifierFeatures {
    #[endpoint]
    fn token_identifier_egld(&self) -> EgldOrDctTokenIdentifier {
        EgldOrDctTokenIdentifier::egld()
    }

    #[endpoint]
    fn token_identifier_is_valid_1(&self, token_id: EgldOrDctTokenIdentifier) -> bool {
        token_id.is_valid()
    }

    #[endpoint]
    fn token_identifier_is_valid_2(&self, bytes: ManagedBuffer) -> bool {
        TokenIdentifier::from(bytes).is_valid_dct_identifier()
    }
}
