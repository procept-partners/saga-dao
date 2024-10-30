// proposals.rs

use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::{AccountId, BorshStorageKey, env};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::U128;
use serde_json::json;

#[derive(BorshStorageKey, BorshSerialize)]
enum ProposalStorageKey {
    Proposals,
    ProposalVoters { proposal_id: u64 },
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: AccountId,
    pub votes_for: U128,
    pub votes_against: U128,
    pub voters: UnorderedSet<AccountId>,
    pub status: ProposalStatus,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
}

impl Proposal {
    pub fn new(id: u64, title: String, description: String, proposer: AccountId) -> Self {
        Proposal {
            id,
            title,
            description,
            proposer,
            votes_for: U128::from(0),
            votes_against: U128::from(0),
            voters: UnorderedSet::new(ProposalStorageKey::ProposalVoters { proposal_id: id }),
            status: ProposalStatus::Active,
        }
    }

    pub fn to_json_value(&self) -> serde_json::Value {
        json!({
            "id": self.id,
            "title": self.title,
            "description": self.description,
            "proposer": self.proposer,
            "votes_for": self.votes_for.0,
            "votes_against": self.votes_against.0,
            "status": self.status,
        })
    }

    pub fn vote(&mut self, voter: &AccountId, vote: bool, is_token_owner: bool) {
        require!(is_token_owner, "Only token owners can vote.");
        require!(self.status == ProposalStatus::Active, "Proposal is not active");
        require!(!self.voters.contains(voter), "Voter has already voted");

        if vote {
            self.votes_for = U128(self.votes_for.0 + 1);
        } else {
            self.votes_against = U128(self.votes_against.0 + 1);
        }
        
        self.voters.insert(voter);

        // Simple majority threshold calculation
        let total_votes = self.votes_for.0 + self.votes_against.0;
        if total_votes >= 1 {
            if self.votes_for.0 > self.votes_against.0 {
                self.status = ProposalStatus::Passed;
            } else {
                self.status = ProposalStatus::Rejected;
            }
        }
    }
}

pub struct Proposals {
    pub proposals: UnorderedMap<u64, Proposal>,
    pub next_proposal_id: u64,
}

impl Proposals {
    pub fn new() -> Self {
        Self {
            proposals: UnorderedMap::new(ProposalStorageKey::Proposals),
            next_proposal_id: 0,
        }
    }

    pub fn create_proposal(
        &mut self,
        title: String,
        description: String,
        proposer: AccountId,
    ) -> u64 {
        let proposal_id = self.next_proposal_id;
        self.next_proposal_id += 1;

        let proposal = Proposal::new(proposal_id, title, description, proposer);
        self.proposals.insert(&proposal_id, &proposal);

        proposal_id
    }

    pub fn get_proposal(&self, proposal_id: u64) -> Option<serde_json::Value> {
        self.proposals.get(&proposal_id).map(|p| p.to_json_value())
    }

    pub fn get_all_proposals(&self) -> Vec<serde_json::Value> {
        self.proposals.values().map(|p| p.to_json_value()).collect()
    }
}