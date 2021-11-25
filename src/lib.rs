use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId};
near_sdk::setup_alloc!();
use near_sdk::collections::UnorderedMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NftOwners {
    owners: UnorderedMap<String, AccountId>,
}

impl Default for NftOwners {
    fn default() -> Self {
        Self {
            owners: UnorderedMap::new(b"o"),
        }
    }
}

#[near_bindgen]
impl NftOwners {
    pub fn set_owner(&mut self, token_id: String, account_id: AccountId) {
        self.owners.insert(&token_id, &account_id);
    }

    pub fn get_owner(&self, token_id: String) -> AccountId {
        match self.owners.get(&token_id) {
            Some(owner) => owner,
            None => "No owner found".to_string(),
        }
    }
}
