use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalType {
    Governance,
    Project { project_id: u64 },
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub proposer: AccountId,
    pub description: String,
    pub proposal_type: ProposalType,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: ProposalStatus,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Active,
    Approved,
    Rejected,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VotingModule {
    pub owner_id: AccountId,
    pub shld_holders: UnorderedMap<AccountId, u64>,
    pub project_contributions: UnorderedMap<(AccountId, u64), u64>,
    pub proposals: UnorderedMap<u64, Proposal>,
}

#[near_bindgen]
impl VotingModule {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            shld_holders: UnorderedMap::new(b"s"),
            project_contributions: UnorderedMap::new(b"p"),
            proposals: UnorderedMap::new(b"r"),
        }
    }

    pub fn create_proposal(&mut self, description: String, proposal_type: ProposalType) -> u64 {
        let proposal_id = (self.proposals.len() + 1) as u64;
        let proposal = Proposal {
            proposer: env::predecessor_account_id(),
            description,
            proposal_type,
            votes_for: 0,
            votes_against: 0,
            status: ProposalStatus::Active,
        };
        
        self.proposals.insert(&proposal_id, &proposal);
        proposal_id
    }

    pub fn get_voting_power(&self, account_id: AccountId, proposal_type: &ProposalType) -> u64 {
        match proposal_type {
            ProposalType::Governance => {
                if self.shld_holders.get(&account_id).is_some() {
                    1 // Each SHLD holder gets one vote for governance
                } else {
                    0
                }
            },
            ProposalType::Project { project_id } => {
                // Get contribution power for project
                self.project_contributions
                    .get(&(account_id, *project_id))
                    .unwrap_or(0)
            }
        }
    }

    pub fn vote(&mut self, proposal_id: u64, vote: bool) {
        let account_id = env::predecessor_account_id();
        let mut proposal = self.proposals.get(&proposal_id).expect("Proposal not found");
        
        let voting_power = self.get_voting_power(account_id, &proposal.proposal_type);
        assert!(voting_power > 0, "No voting power for this proposal");
        
        if vote {
            proposal.votes_for += voting_power;
        } else {
            proposal.votes_against += voting_power;
        }
        
        // Update proposal status
        if proposal.votes_for > proposal.votes_against * 2 { // 66% majority
            proposal.status = ProposalStatus::Approved;
        } else if proposal.votes_against >= proposal.votes_for {
            proposal.status = ProposalStatus::Rejected;
        }
        
        self.proposals.insert(&proposal_id, &proposal);
    }

    // SHLD token management functions
    pub fn register_shld_holder(&mut self, account_id: AccountId) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can register SHLD holders"
        );
        self.shld_holders.insert(&account_id, &1);
    }

    pub fn register_project_contribution(
        &mut self,
        account_id: AccountId,
        project_id: u64,
        contribution: u64,
    ) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only owner can register contributions"
        );
        self.project_contributions.insert(&(account_id, project_id), &contribution);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn setup_context() -> VMContextBuilder {
        let mut context = VMContextBuilder::new();
        context.predecessor_account_id(accounts(0));
        context.current_account_id(accounts(0));
        context
    }

    #[test]
    fn test_create_proposal() {
        let mut context = setup_context();
        testing_env!(context.build());
        
        let mut module = VotingModule::new(accounts(0));
        
        let proposal_id = module.create_proposal(
            "Test proposal".to_string(),
            ProposalType::Governance,
        );
        
        assert_eq!(proposal_id, 1);
        let proposal = module.proposals.get(&proposal_id).unwrap();
        assert_eq!(proposal.status, ProposalStatus::Active);
    }

    #[test]
    fn test_governance_voting_power() {
        let mut context = setup_context();
        testing_env!(context.build());
        
        let mut module = VotingModule::new(accounts(0));
        
        // Register SHLD holder
        module.register_shld_holder(accounts(1));
        
        assert_eq!(
            module.get_voting_power(accounts(1), &ProposalType::Governance),
            1
        );
        assert_eq!(
            module.get_voting_power(accounts(2), &ProposalType::Governance),
            0
        );
    }
}