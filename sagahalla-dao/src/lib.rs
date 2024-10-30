pub mod mana_structs;
mod voting;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId};
use voting::{VotingModule, ProjectPlanStatus, ProjectExecutionStatus};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub voting_module: VotingModule,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id: owner_id.clone(),
            voting_module: VotingModule {
                shld_holders: UnorderedMap::new(b"s"),
                project_contributions: UnorderedMap::new(b"p"),
                proposals: UnorderedMap::new(b"r"),
                project_plan_votes: UnorderedMap::new(b"v"),
                project_execution_votes: UnorderedMap::new(b"e"),
            },
        }
    }

    // Contract methods for governance and project voting can call methods in `voting_module`
}