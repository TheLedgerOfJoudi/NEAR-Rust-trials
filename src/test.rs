mod lib;
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_owner() {
        let context = get_context("hdsaleh.testnet".to_string(), 0);
        testing_env!(context);
        let mut contract = NftOwners::default();
        let my_token = "my_token".to_string();
        let owner_account_id = "hdsaleh.testnet".to_string();
        contract.set_owner(my_token.clone(), owner_account_id.clone());
        let owner_of_nft = contract.get_owner(my_token);
        assert_eq!(owner_of_nft, owner_account_id);
    }
}
