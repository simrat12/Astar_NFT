#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod Nft {
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::{
        contracts::{
            ownable::*,
            psp34::extensions::{
                enumerable::*,
                metadata::*,
            },
            reentrancy_guard::*,
        },
        traits::{
            Storage,
            String,
        },
    };

    use payable_mint_pkg::{
        impls::payable_mint::*,
        traits::payable_mint::*,
    };

    // NftContract contract storage
    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct NftContract {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        guard: reentrancy_guard::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        payable_mint: types::Data,
    }

    impl PSP34 for NftContract {}
    impl PSP34Enumerable for NftContract {}
    impl PSP34Metadata for NftContract {}
    impl Ownable for NftContract {}

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: Id,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: Option<Id>,
        approved: bool,
    }

    impl NftContract {
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
            price_per_mint: Balance,
        ) -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(instance.env().caller());
            let collection_id = instance.collection_id();
            instance._set_attribute(collection_id.clone(), String::from("name"), name);
            instance._set_attribute(collection_id.clone(), String::from("symbol"), symbol);
            instance._set_attribute(collection_id, String::from("baseUri"), base_uri);
            instance.payable_mint.max_supply = max_supply;
            instance.payable_mint.price_per_mint = price_per_mint;
            instance.payable_mint.last_token_id = 0;
            instance.payable_mint.max_amount = 1;
            instance
        }
    }

    // Override event emission methods
    impl psp34::Internal for NftContract {
        fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
            self.env().emit_event(Transfer { from, to, id });
        }

        fn _emit_approval_event(
            &self,
            from: AccountId,
            to: AccountId,
            id: Option<Id>,
            approved: bool,
        ) {
            self.env().emit_event(Approval {
                from,
                to,
                id,
                approved,
            });
        }
    }

    impl PayableMint for NftContract {}

    // ------------------- T E S T -----------------------------------------------------
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::nft::PSP34Error::*;
        use ink::{
            env::{
                pay_with_call,
                test,
            },
            prelude::string::String as PreludeString,
        };
        use payable_mint_pkg::impls::payable_mint::{
            payable_mint::Internal,
            types::NftError,
        };
        const PRICE: Balance = 100_000_000_000_000_000;
        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const MAX_SUPPLY: u64 = 10;

        #[ink::test]
        fn init_works() {
            let sh34 = init();
            let collection_id = sh34.collection_id();
            assert_eq!(
                sh34.get_attribute(collection_id.clone(), String::from("name")),
                Some(String::from("nft"))
            );
            assert_eq!(
                sh34.get_attribute(collection_id.clone(), String::from("symbol")),
                Some(String::from("SH34"))
            );
            assert_eq!(
                sh34.get_attribute(collection_id, String::from("baseUri")),
                Some(String::from(BASE_URI))
            );
            assert_eq!(sh34.max_supply(), MAX_SUPPLY);
            assert_eq!(sh34.price(), PRICE);
        }

        fn init() -> NftContract {
            NftContract::new(
                String::from("nft"),
                String::from("nft"),
                String::from(BASE_URI),
                MAX_SUPPLY,
                PRICE,
            )
        }

        #[ink::test]
        fn mint_single_works() {
            let mut sh34 = init();
            let accounts = default_accounts();
            assert_eq!(sh34.owner(), accounts.alice);
            set_sender(accounts.bob);

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink::env::DefaultEnvironment>(PRICE);
            assert!(sh34.mint_next().is_ok());
            assert_eq!(sh34.total_supply(), 1);
            assert_eq!(sh34.owner_of(Id::U64(1)), Some(accounts.bob));
            assert_eq!(sh34.balance_of(accounts.bob), 1);

            assert_eq!(sh34.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(sh34.payable_mint.last_token_id, 1);
            assert_eq!(1, ink::env::test::recorded_events().count());
        }

        #[ink::test]
        fn mint_multiple_works() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = 5;
            // Set max limit to 'num_of_mints', fails to mint 'num_of_mints + 1'. Caller is contract owner
            assert!(sh34.set_max_mint_amount(num_of_mints).is_ok());
            assert_eq!(
                sh34.mint(accounts.bob, num_of_mints + 1),
                Err(PSP34Error::Custom(
                    NftError::TooManyTokensToMint.as_str()
                ))
            );

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink::env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(sh34.mint(accounts.bob, num_of_mints).is_ok());
            assert_eq!(sh34.total_supply(), num_of_mints as u128);
            assert_eq!(sh34.balance_of(accounts.bob), 5);
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 0), Ok(Id::U64(1)));
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 1), Ok(Id::U64(2)));
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 2), Ok(Id::U64(3)));
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 3), Ok(Id::U64(4)));
            assert_eq!(sh34.owners_token_by_index(accounts.bob, 4), Ok(Id::U64(5)));
            assert_eq!(5, ink::env::test::recorded_events().count());
            assert_eq!(
                sh34.owners_token_by_index(accounts.bob, 5),
                Err(TokenNotExists)
            );
        }

        #[ink::test]
        fn mint_above_limit_fails() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);
            let num_of_mints: u64 = MAX_SUPPLY + 1;

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink::env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(sh34.set_max_mint_amount(num_of_mints).is_ok());
            assert_eq!(
                sh34.mint(accounts.bob, num_of_mints),
                Err(PSP34Error::Custom(NftError::CollectionIsFull.as_str()))
            );
        }

        #[ink::test]
        fn mint_low_value_fails() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_sender(accounts.bob);
            let num_of_mints = 1;

            assert_eq!(sh34.total_supply(), 0);
            test::set_value_transferred::<ink::env::DefaultEnvironment>(
                PRICE * num_of_mints as u128 - 1,
            );
            assert_eq!(
                sh34.mint(accounts.bob, num_of_mints),
                Err(PSP34Error::Custom(NftError::BadMintValue.as_str()))
            );
            test::set_value_transferred::<ink::env::DefaultEnvironment>(
                PRICE * num_of_mints as u128 - 1,
            );
            assert_eq!(
                sh34.mint_next(),
                Err(PSP34Error::Custom(NftError::BadMintValue.as_str()))
            );
            assert_eq!(sh34.total_supply(), 0);
        }

        #[ink::test]
        fn withdrawal_works() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_balance(accounts.bob, PRICE);
            set_sender(accounts.bob);

            assert!(pay_with_call!(sh34.mint_next(), PRICE).is_ok());
            let expected_contract_balance = PRICE + sh34.env().minimum_balance();
            assert_eq!(sh34.env().balance(), expected_contract_balance);

            // Bob fails to withdraw
            set_sender(accounts.bob);
            assert!(sh34.withdraw().is_err());
            assert_eq!(sh34.env().balance(), expected_contract_balance);

            // Alice (contract owner) withdraws. Existential minimum is still set
            set_sender(accounts.alice);
            assert!(sh34.withdraw().is_ok());
            // assert_eq!(sh34.env().balance(), sh34.env().minimum_balance());
        }

        #[ink::test]
        fn token_uri_works() {
            let mut sh34 = init();
            let accounts = default_accounts();
            set_sender(accounts.alice);

            test::set_value_transferred::<ink::env::DefaultEnvironment>(PRICE);
            assert!(sh34.mint_next().is_ok());
            // return error if request is for not yet minted token
            assert_eq!(sh34.token_uri(42), Err(TokenNotExists));
            assert_eq!(
                sh34.token_uri(1),
                Ok(PreludeString::from(BASE_URI.to_owned() + "1.json"))
            );

            // return error if request is for not yet minted token
            assert_eq!(sh34.token_uri(42), Err(TokenNotExists));

            // verify token_uri when baseUri is empty
            set_sender(accounts.alice);
            assert!(sh34.set_base_uri(PreludeString::from("")).is_ok());
            assert_eq!(
                sh34.token_uri(1),
                Ok("".to_owned() + &PreludeString::from("1.json"))
            );
        }

        #[ink::test]
        fn owner_is_set() {
            let accounts = default_accounts();
            let sh34 = init();
            assert_eq!(sh34.owner(), accounts.alice);
        }

        #[ink::test]
        fn set_base_uri_works() {
            let accounts = default_accounts();
            const NEW_BASE_URI: &str = "new_uri/";
            let mut sh34 = init();

            set_sender(accounts.alice);
            let collection_id = sh34.collection_id();
            assert!(sh34.set_base_uri(NEW_BASE_URI.into()).is_ok());
            assert_eq!(
                sh34.get_attribute(collection_id, String::from("baseUri")),
                Some(String::from(NEW_BASE_URI))
            );
            set_sender(accounts.bob);
            assert_eq!(
                sh34.set_base_uri(NEW_BASE_URI.into()),
                Err(PSP34Error::Custom(String::from("O::CallerIsNotOwner")))
            );
        }

        #[ink::test]
        fn check_supply_overflow_ok() {
            let max_supply = u64::MAX - 1;
            let mut sh34 = NftContract::new(
                String::from("nft"),
                String::from("nft"),
                String::from(BASE_URI),
                max_supply,
                PRICE,
            );
            sh34.payable_mint.last_token_id = max_supply - 1;

            // check case when last_token_id.add(mint_amount) if more than u64::MAX
            assert!(sh34.set_max_mint_amount(u64::MAX).is_ok());
            assert_eq!(
                sh34.check_amount(3),
                Err(PSP34Error::Custom(NftError::CollectionIsFull.as_str()))
            );

            // check case when mint_amount is 0
            assert_eq!(
                sh34.check_amount(0),
                Err(PSP34Error::Custom(
                    NftError::CannotMintZeroTokens.as_str()
                ))
            );
        }

        #[ink::test]
        fn check_value_overflow_ok() {
            let max_supply = u64::MAX;
            let price = u128::MAX as u128;
            let sh34 = NftContract::new(
                String::from("nft"),
                String::from("nft"),
                String::from(BASE_URI),
                max_supply,
                price,
            );
            let transferred_value = u128::MAX;
            let mint_amount = u64::MAX;
            assert_eq!(
                sh34.check_value(transferred_value, mint_amount),
                Err(PSP34Error::Custom(NftError::BadMintValue.as_str()))
            );
        }

        fn default_accounts() -> test::DefaultAccounts<ink::env::DefaultEnvironment> {
            test::default_accounts::<Environment>()
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink::env::test::set_account_balance::<ink::env::DefaultEnvironment>(account_id, balance)
        }
    }
}
