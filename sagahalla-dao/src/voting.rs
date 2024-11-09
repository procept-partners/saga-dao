use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::{env, AccountId, require, near_bindgen, log};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use serde_json::json;

// Enum for Project Plan and Project Execution statuses
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ProjectPlanStatus {
    Active,
    Approved,
    Rejected,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ProjectExecutionStatus {
    Active,
    Approved,
    Rejected,
}

// Structures to track votes for Project Plans and Executions
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectPlanVote {
    pub project_plan_id: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: ProjectPlanStatus,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectExecutionVote {
    pub project_execution_id: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: ProjectExecutionStatus,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectContribution {
    pub account_id: AccountId,
    pub contribution_amount: u64,
    pub timestamp: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GovernanceData {
    pub mana_balance: U128,
    pub mana_collateral_balance: U128,
    pub governance_power: u64,  // Governance power for general proposals and project plans
    pub project_governance_power: u64, // Project-specific governance power
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VotingModule {
    pub shld_holders: UnorderedMap<AccountId, u64>,
    pub project_plan_votes: UnorderedMap<u64, ProjectPlanVote>,
    pub project_execution_votes: UnorderedMap<u64, ProjectExecutionVote>,
    pub project_contributions: UnorderedMap<AccountId, Vec<ProjectContribution>>,
    pub governance_data: UnorderedMap<AccountId, GovernanceData>,
}

impl Default for VotingModule {
    fn default() -> Self {
        Self {
            shld_holders: UnorderedMap::new(b"s"),
            project_plan_votes: UnorderedMap::new(b"v"),
            project_execution_votes: UnorderedMap::new(b"e"),
            project_contributions: UnorderedMap::new(b"p"),
            governance_data: UnorderedMap::new(b"g"),
        }
    }
}

#[near_bindgen]
impl VotingModule {
    #[init]
    pub fn init_voting() -> Self {
        Self::default()
    }

    pub fn update_governance_data(
        &mut self,
        account_id: AccountId,
        mana_balance: U128,
        mana_collateral_balance: U128,
        total_circulating_supply: U128,
        project_allocated_mana: U128,
        project_total_allocated_mana: U128,
    ) {
        // Calculate governance power for proposals and project plans
        let governance_power = if total_circulating_supply.0 > 0 {
            ((mana_balance.0 + mana_collateral_balance.0) * 1_000_000 / total_circulating_supply.0) as u64
        } else {
            0
        };

        // Calculate project-specific governance power
        let project_governance_power = if project_total_allocated_mana.0 > 0 {
            (project_allocated_mana.0 * 1_000_000 / project_total_allocated_mana.0) as u64
        } else {
            0
        };

        let data = GovernanceData {
            mana_balance,
            mana_collateral_balance,
            governance_power,
            project_governance_power,
        };
        self.governance_data.insert(&account_id, &data);

        env::log_str(&format!(
            "Updated governance data for account {}: governance_power = {}, project_governance_power = {}",
            account_id, governance_power, project_governance_power,
        ));
    }

    pub fn vote_on_proposal(&mut self, proposal_id: u64, account_id: AccountId, support: bool) {
        let governance_data = self
            .governance_data
            .get(&account_id)
            .expect("Governance data not found for account");

        let vote_weight = governance_data.governance_power;

        let mut proposal = self
            .project_plan_votes
            .get(&proposal_id)
            .expect("Proposal not found");

        if support {
            proposal.votes_for += vote_weight;
        } else {
            proposal.votes_against += vote_weight;
        }

        // Update proposal status based on vote results
        if proposal.votes_for > proposal.votes_against {
            proposal.status = ProjectPlanStatus::Approved;
        } else {
            proposal.status = ProjectPlanStatus::Rejected;
        }

        self.project_plan_votes.insert(&proposal_id, &proposal);

        env::log_str(&format!(
            "Proposal {} updated: votes_for = {}, votes_against = {}, status = {:?}",
            proposal_id, proposal.votes_for, proposal.votes_against, proposal.status,
        ));
    }

    pub fn vote_on_project_execution(
        &mut self,
        project_execution_id: u64,
        account_id: AccountId,
        support: bool,
    ) {
        let governance_data = self
            .governance_data
            .get(&account_id)
            .expect("Governance data not found for account");

        let vote_weight = governance_data.project_governance_power;

        let mut project_execution = self
            .project_execution_votes
            .get(&project_execution_id)
            .expect("Project execution not found");

        if support {
            project_execution.votes_for += vote_weight;
        } else {
            project_execution.votes_against += vote_weight;
        }

        // Update project execution status based on vote results
        if project_execution.votes_for > project_execution.votes_against {
            project_execution.status = ProjectExecutionStatus::Approved;
        } else {
            project_execution.status = ProjectExecutionStatus::Rejected;
        }

        self.project_execution_votes.insert(&project_execution_id, &project_execution);

        env::log_str(&format!(
            "Project execution {} updated: votes_for = {}, votes_against = {}, status = {:?}",
            project_execution_id, project_execution.votes_for, project_execution.votes_against, project_execution.status,
        ));
    }
}
