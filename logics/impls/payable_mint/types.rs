use openbrush::traits::{
    Balance,
    String,
};
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);
use ink::prelude::vec::Vec;

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub last_token_id: u64,
    pub collection_id: u32,
    pub max_supply: u64,
    pub price_per_mint: Balance,
    pub max_amount: u64,
    pub hp: Vec<(u64, u64)>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NftError {
    BadMintValue,
    CannotMintZeroTokens,
    CollectionIsFull,
    TooManyTokensToMint,
    WithdrawalFailed,
}

impl NftError {
    pub fn as_str(&self) -> String {
        match self {
            NftError::BadMintValue => String::from("BadMintValue"),
            NftError::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            NftError::CollectionIsFull => String::from("CollectionIsFull"),
            NftError::TooManyTokensToMint => String::from("TooManyTokensToMint"),
            NftError::WithdrawalFailed => String::from("WithdrawalFailed"),
        }
    }
}
