#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod orderbook {

    use ink_storage::traits::SpreadAllocate;
    use ink_storage::Mapping;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct OrderbookContract {
        /// Storage of values for the orderbook
        my_account: AccountId,
        user_balance: Balance,
        asks: Mapping<u32, u128>,
        bids: Mapping<u32, u128>,
        trading_pair: String,
    }

    impl OrderbookContract {
        /// Initial contract state into storage.
        #[ink(constructor)]
        pub fn new(base: String, quote: String) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.my_account = AccountId;
            })
        }

        // Mapping from a Decimal Price to a Limit Order
        pub fn new(limit: Decimal) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.map.insert(&caller, &count);
            })
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let orderbook = Orderbook::default();
            assert_eq!(orderbook.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut orderbook = Orderbook::new(false);
            assert_eq!(orderbook.get(), false);
            orderbook.flip();
            assert_eq!(orderbook.get(), true);
        }
    }
}