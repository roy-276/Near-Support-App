
# Near Support App

This smart contract is build with the aim of supporting content creators around the world. We help people show their support by providing them with a easy and secure way to send funds in NEAR to their favorite musicians or youtubers.\
NEAR protocol provides all the neccesary features for this purpose.

## How it works

### Imports

Here we import all the neccesary dependencies needed.\
Since the transfer method takes a number in yoctoNEAR, it's likely to need numbers much larger than 2^53, therefore json_types is used.\
To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)

    use std::collections::HashMap;
    use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
    use near_sdk::{env, near_bindgen, setup_alloc,AccountId, Promise, json_types::U128};

Support State is declared here.

    #[near_bindgen]
    #[derive(BorshDeserialize, BorshSerialize)]

    pub struct Support {
        deposits: HashMap<String, u128>,
        gift: HashMap<String, u128>,
    }

We initialize the default support state using the Default keyword.

    impl Default for Support {
        fn default() -> Self {
            Self { deposits: HashMap::new(), gift: HashMap::new(),}
        }
    }

Here we define all the implementations and methods that will be available on the Support object. 

    #[near_bindgen]
    impl Support {
        ...// Implementation here...
    }

The deposit method allows the fans to deposit a certain amount that they would wish to use in their support quest.\
The data is stored in a hashmap with the fan's account_id as the key and the deposit as the value. 

    #[payable]
    pub fn deposit(&mut self) {
        let account_id = env::predecessor_account_id();
        let deposit: u128 = env::attached_deposit();
        let previous_deposit: u128 = self.get_deposit(account_id.clone());
        self.deposits.insert(account_id, previous_deposit + deposit);
    }

`get_deposit()` method allows the client to view the deposited amount if any remaining on his account.

    pub fn get_deposit(&self, account_id: String) -> u128 {
        match self.deposits.get(&account_id) {
            Some(deposit) => *deposit,
            None => 0,
        }
    }

`send_gift()` implementation can be called with or without a deposit thus payable function.\
json_types U128 is used incase the amount transfered is larger than 2^53.\
The type of account passed to the function is also checked to confirm it's a valid NEAR AccountId.\
If the client has sufficient `self.deposit` it will be used for the transaction and the `self.deposit` value be subtracted by the amount sent.
If the client does not have sufficient `self.deposit`or no deposit the amount will be charged from the attached_deposit if any and the remaining amount be added to the client's `self.deposit` value on the HashMap.\
Near_sdk's Promise method is used to make the transactions.

    #[payable]
    pub fn send_gift(&mut self, youtube_user_id: AccountId, token: U128) -> Promise {
        let account_id = env::predecessor_account_id();
        let deposit: u128 = env::attached_deposit();
        let amount = u128::from(token);
        let deposited_amount = self.get_deposit(account_id.clone());

        if deposited_amount < amount {
            assert!(amount <= deposit, "Amount not enough for the transaction");
            self.deposits
                .insert(account_id.clone(), deposit - amount.clone());
        }
        self.deposits
            .insert(account_id.clone(), deposited_amount - amount.clone());

        let balance: u128 = self.get_balance(youtube_user_id.clone());
        self.gift.insert(youtube_user_id.clone(), balance + amount);
        Promise::new(youtube_user_id).transfer(token.0)
    }

The `get_balance()` method allows the user to get the amount gifted to a specific on our smart contract.

    pub fn get_balance(&self, youtube_user_id: String) -> u128 {
        match self.gift.get(&youtube_user_id) {
            Some(gift) => *gift,
            None => 0,
        }
    }

## Unit tests

The imports for unit tests

    #[cfg(test)]
    mod tests {
        use super::*;
        use near_sdk::MockedBlockchain;
        use near_sdk::{testing_env, VMContext};

Part of writing unit tests is setting up a mock context for testing.\
Near_sdk's `VMContext` is used to simulate a user interaction with the smart contract.
more info on VMContext [here](https://www.near-sdk.io/testing/unit-tests)\

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            ...// Implementation here...
        }
    }

We are also using the function `ntoy()` to convert yocto to NEAR.

    fn ntoy(near_amount: u128) -> u128 {
            near_amount * 10u128.pow(24)
        }

Individual unit tests with #[test] registered and fired.\
Tests for the `deposit()` method using the mock context as the testing environment.\
Here we try to make a deposit with an attached_deposit of 10 near and asserting that the account was added to the deposits HashMap with the right value.

    #[test]
    fn deposit_test() {
        let mut context = get_context(vec![], false);
        context.attached_deposit = ntoy(10);
        context.is_view = false;
        testing_env!(context);
        let mut contract = Support::default();

        contract.deposit();
        assert_eq!(
            contract.get_deposit(env::predecessor_account_id().to_string()),
            ntoy(10)
        );
    }

Test for the `send_gift()` method using the mock context as the testing environment.\
Here we are sending a gift of 10 NEAR to an account by id '12345' and checkinf if the account recieved it using the `get_balance()` method.


    #[test]
    fn send_gift_test() {
        let mut context = get_context(vec![], false);
        context.attached_deposit = ntoy(10);
        context.is_view = false;

        testing_env!(context);
        let mut contract = Support::default();

        contract.deposit();
        contract.send_gift("12345".to_string(), U128::from(ntoy(4)));
        assert_eq!(contract.get_balance("12345".to_string()), ntoy(4));
    }

A [smart contract] written in [Rust] for an app initialized with [create-near-app]

[smart contract]: https://docs.near.org/docs/develop/contracts/overview
[rust]: https://www.rust-lang.org/
[create-near-app]: https://github.com/near/create-near-app
[correct target]: https://github.com/near/near-sdk-rs#pre-requisites
[cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
