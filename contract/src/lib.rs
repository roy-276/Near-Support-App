use std::collections::HashMap;

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, json_types::U128, near_bindgen, setup_alloc, AccountId, Promise};

setup_alloc!();

// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]

// Support State
pub struct Support {
    deposits: HashMap<String, u128>,
    gift: HashMap<String, u128>,
}

impl Default for Support {
    fn default() -> Self {
        Self {
            deposits: HashMap::new(),
            gift: HashMap::new(),
        }
    }
}

// Support Methods
#[near_bindgen]
impl Support {
    #[payable]
    pub fn deposit(&mut self) {
        let account_id = env::predecessor_account_id();
        let deposit: u128 = env::attached_deposit();

        let previous_deposit: u128 = self.get_deposit(account_id.clone());

        self.deposits.insert(account_id, previous_deposit + deposit);
    }

    pub fn get_deposit(&self, account_id: String) -> u128 {
        match self.deposits.get(&account_id) {
            Some(deposit) => *deposit,
            None => 0,
        }
    }

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

            return Promise::new(youtube_user_id).transfer(token.0);
        } else {
            self.deposits
                .insert(account_id.clone(), deposited_amount - amount.clone());

            let balance: u128 = self.get_balance(youtube_user_id.clone());
            self.gift.insert(youtube_user_id.clone(), balance + amount);

            Promise::new(youtube_user_id).transfer(token.0)
        }
    }

    pub fn get_balance(&self, youtube_user_id: String) -> u128 {
        match self.gift.get(&youtube_user_id) {
            Some(gift) => *gift,
            None => 0,
        }
    }
}

// Use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // Individual unit tests with #[test] registered and fired
    #[test]

    fn deposit_test() {
        // set up the mock context into the testing environment
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

    #[test]
    fn set_gift_test() {
        // set up the mock context into the testing environment
        let mut context = get_context(vec![], false);
        context.attached_deposit = ntoy(10);
        context.is_view = false;

        testing_env!(context);
        let mut contract = Support::default();

        contract.deposit();

        contract.send_gift("12345".to_string(), U128::from(ntoy(4)));

        assert_eq!(contract.get_balance("12345".to_string()), ntoy(4));
    }
    // ntoy(4)
    // convert near to yocto
    fn ntoy(near_amount: u128) -> u128 {
        near_amount * 10u128.pow(24)
    }
}
