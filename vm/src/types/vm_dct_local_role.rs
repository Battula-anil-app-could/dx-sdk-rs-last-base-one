use super::DctLocalRoleFlags;

const DCT_ROLE_NONE: &str = "";
const DCT_ROLE_LOCAL_MINT: &str = "DCTRoleLocalMint";
const DCT_ROLE_LOCAL_BURN: &str = "DCTRoleLocalBurn";
const DCT_ROLE_NFT_CREATE: &str = "DCTRoleNFTCreate";
const DCT_ROLE_NFT_ADD_QUANTITY: &str = "DCTRoleNFTAddQuantity";
const DCT_ROLE_NFT_BURN: &str = "DCTRoleNFTBurn";
const DCT_ROLE_NFT_ADD_URI: &str = "DCTRoleNFTAddURI";
const DCT_ROLE_NFT_UPDATE_ATTRIBUTES: &str = "DCTRoleNFTUpdateAttributes";
const DCT_ROLE_TRANSFER: &str = "DCTTransferRole";

/// The VM implementation for DctLocalRole, used internally in builtin functions.
///
/// There is another near-identical implementation in the framework, used for communicating with the VM.
///
/// It might be a good idea to move it to some "common ground" crate, between the framework and the VM.
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub enum DctLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
    NftAddUri,
    NftUpdateAttributes,
    Transfer,
}

impl DctLocalRole {
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::None => 0,
            Self::Mint => 1,
            Self::Burn => 2,
            Self::NftCreate => 3,
            Self::NftAddQuantity => 4,
            Self::NftBurn => 5,
            Self::NftAddUri => 6,
            Self::NftUpdateAttributes => 7,
            Self::Transfer => 8,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => DCT_ROLE_NONE,
            Self::Mint => DCT_ROLE_LOCAL_MINT,
            Self::Burn => DCT_ROLE_LOCAL_BURN,
            Self::NftCreate => DCT_ROLE_NFT_CREATE,
            Self::NftAddQuantity => DCT_ROLE_NFT_ADD_QUANTITY,
            Self::NftBurn => DCT_ROLE_NFT_BURN,
            Self::NftAddUri => DCT_ROLE_NFT_ADD_URI,
            Self::NftUpdateAttributes => DCT_ROLE_NFT_UPDATE_ATTRIBUTES,
            Self::Transfer => DCT_ROLE_TRANSFER,
        }
    }

    pub fn to_flag(&self) -> DctLocalRoleFlags {
        match self {
            Self::None => DctLocalRoleFlags::NONE,
            Self::Mint => DctLocalRoleFlags::MINT,
            Self::Burn => DctLocalRoleFlags::BURN,
            Self::NftCreate => DctLocalRoleFlags::NFT_CREATE,
            Self::NftAddQuantity => DctLocalRoleFlags::NFT_ADD_QUANTITY,
            Self::NftBurn => DctLocalRoleFlags::NFT_BURN,
            Self::NftAddUri => DctLocalRoleFlags::NFT_ADD_URI,
            Self::NftUpdateAttributes => DctLocalRoleFlags::NFT_UPDATE_ATTRIBUTES,
            Self::Transfer => DctLocalRoleFlags::TRANSFER,
        }
    }
}

// TODO: can be done with macros, but I didn't find a public library that does it and is no_std
// we can implement it, it's easy
const ALL_ROLES: [DctLocalRole; 8] = [
    DctLocalRole::Mint,
    DctLocalRole::Burn,
    DctLocalRole::NftCreate,
    DctLocalRole::NftAddQuantity,
    DctLocalRole::NftBurn,
    DctLocalRole::NftAddUri,
    DctLocalRole::NftUpdateAttributes,
    DctLocalRole::Transfer,
];

impl DctLocalRole {
    pub fn iter_all() -> core::slice::Iter<'static, DctLocalRole> {
        ALL_ROLES.iter()
    }
}

impl From<u16> for DctLocalRole {
    #[inline]
    fn from(value: u16) -> Self {
        match value {
            1 => Self::Mint,
            2 => Self::Burn,
            3 => Self::NftCreate,
            4 => Self::NftAddQuantity,
            5 => Self::NftBurn,
            6 => Self::NftAddUri,
            7 => Self::NftUpdateAttributes,
            8 => Self::Transfer,
            _ => Self::None,
        }
    }
}

impl<'a> From<&'a [u8]> for DctLocalRole {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == DCT_ROLE_LOCAL_MINT.as_bytes() {
            Self::Mint
        } else if byte_slice == DCT_ROLE_LOCAL_BURN.as_bytes() {
            Self::Burn
        } else if byte_slice == DCT_ROLE_NFT_CREATE.as_bytes() {
            Self::NftCreate
        } else if byte_slice == DCT_ROLE_NFT_ADD_QUANTITY.as_bytes() {
            Self::NftAddQuantity
        } else if byte_slice == DCT_ROLE_NFT_BURN.as_bytes() {
            Self::NftBurn
        } else if byte_slice == DCT_ROLE_NFT_ADD_URI.as_bytes() {
            Self::NftAddUri
        } else if byte_slice == DCT_ROLE_NFT_UPDATE_ATTRIBUTES.as_bytes() {
            Self::NftUpdateAttributes
        } else if byte_slice == DCT_ROLE_TRANSFER.as_bytes() {
            Self::Transfer
        } else {
            Self::None
        }
    }
}
