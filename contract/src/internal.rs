use model::TokenId;
use near_sdk::{env, store::UnorderedSet, AccountId, Balance, CryptoHash, Promise};

use crate::contract::{Contract, StorageKey};

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

//refund the initial deposit based on the amount of storage that was used up
pub(crate) fn refund_deposit(storage_used: u64) {
    //get how much it would cost to store the information
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    //get the attached deposit
    let attached_deposit = env::attached_deposit();

    //make sure that the attached deposit is greater than or equal to the required cost
    assert!(
        required_cost <= attached_deposit,
        "Must attach {required_cost} yoctoNEAR to cover storage",
    );

    //get the refund amount from the attached deposit - required cost
    let refund = attached_deposit - required_cost;

    //if the refund is greater than 1 yocto NEAR, we refund the predecessor that amount
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

impl Contract {
    pub(crate) fn internal_add_token_to_owner(&mut self, account_id: &AccountId, token_id: &TokenId) {
        let set = self.tokens_per_owner.entry(account_id.clone()).or_insert_with(|| {
            UnorderedSet::new(StorageKey::TokenPerOwnerInner {
                //we get a new unique prefix for the collection
                account_id_hash: hash_account_id(account_id),
            })
        });

        set.insert(token_id.clone());
    }
}
