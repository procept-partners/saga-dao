use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};
use async_trait::async_trait;
use crate::mana_structs::{Proposal, ProposalType, ProposalStatus};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;
use aurora_engine_sdk::proof::verify_proof;
use near_sdk::json_types::U128;

// Define constants for timeout duration
const RPC_TIMEOUT_SECONDS: u64 = 10;

// Enums for Project Plan and Project Execution statuses
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

// VotingModule definition
#[near_bindgen]
pub struct VotingModule {
    pub shld_holders: UnorderedMap<AccountId, u64>,
    pub project_contributions: UnorderedMap<(AccountId, u64), u64>,
    pub total_mana_hours: UnorderedMap<u64, u64>,
    pub proposals: UnorderedMap<u64, Proposal>,
    pub project_plan_votes: UnorderedMap<u64, ProjectPlanVote>,
    pub project_execution_votes: UnorderedMap<u64, ProjectExecutionVote>,
}

// Async trait for Aurora-based operations
#[async_trait]
pub trait AuroraIntegration {
    async fn get_aurora_mana_balances(&self, shld_holder_id: &AccountId) -> Result<(u128, u128), Box<dyn Error>>;
    async fn get_circulating_supply(&self) -> Result<(u128, u128), Box<dyn Error>>;
}

#[near_bindgen]
impl VotingModule {
    // Calculates governance voting power as a percentage
    pub async fn get_governance_voting_power(&self, account_id: &AccountId) -> u64 {
        let (mana_balance, collateral_mana_balance) = self.get_aurora_mana_balances(account_id).await.unwrap_or((0, 0));
        let (circulating_mana, circulating_collateral_mana) = self.get_circulating_supply().await.unwrap_or((0, 0));

        if circulating_mana == 0 || circulating_collateral_mana == 0 {
            env::log_str("Circulating supply is zero; unable to calculate voting power.");
            return 0;
        }

        let mana_ratio = mana_balance * 1_000_000 / circulating_mana;
        let collateral_mana_ratio = collateral_mana_balance * 1_000_000 / circulating_collateral_mana;

        (mana_ratio + collateral_mana_ratio) as u64
    }

    // Calculates project voting power based on mana hours
    pub fn get_project_voting_power(&self, account_id: &AccountId, project_id: u64) -> u64 {
        let account_contribution = *self.project_contributions.get(&(account_id.clone(), project_id)).unwrap_or(&0);
        let total_project_mana_hours = *self.total_mana_hours.get(&project_id).unwrap_or(&0);

        if total_project_mana_hours == 0 {
            env::log_str("Total project mana hours is zero; unable to calculate voting power.");
            return 0;
        }

        (account_contribution * 100 / total_project_mana_hours) as u64
    }

    // Async function to fetch Aurora-based mana and collateralized MANA balances with timeout handling
    async fn get_aurora_mana_balances(&self, shld_holder_id: &AccountId) -> Result<(u128, u128), Box<dyn Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(RPC_TIMEOUT_SECONDS))
            .build()?;

        let url = "https://aurora.rpc.endpoint.example";
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "get_mana_balances",
            "params": [shld_holder_id],
            "id": 1
        });

        match client.post(url).json(&request_body).send().await {
            Ok(response) => match response.json::<Value>().await {
                Ok(data) => {
                    let mana_balance = data["result"]["mana_balance"].as_u64().unwrap_or(0) as u128;
                    let collateral_mana_balance = data["result"]["collateral_mana_balance"].as_u64().unwrap_or(0) as u128;
                    Ok((mana_balance, collateral_mana_balance))
                }
                Err(e) => {
                    env::log_str("Failed to parse JSON response from Aurora.");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                env::log_str("Failed to fetch data from Aurora: Network timeout or error.");
                Err(Box::new(e))
            }
        }
    }

    // Async function to fetch circulating supply from Aurora with timeout handling
    async fn get_circulating_supply(&self) -> Result<(u128, u128), Box<dyn Error>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(RPC_TIMEOUT_SECONDS))
            .build()?;

        let url = "https://aurora.rpc.endpoint.example";
        let request_body = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "get_circulating_supply",
            "params": [],
            "id": 1
        });

        match client.post(url).json(&request_body).send().await {
            Ok(response) => match response.json::<Value>().await {
                Ok(data) => {
                    let circulating_mana = data["result"]["circulating_mana"].as_u64().unwrap_or(0) as u128;
                    let circulating_collateral_mana = data["result"]["circulating_collateral_mana"].as_u64().unwrap_or(0) as u128;
                    Ok((circulating_mana, circulating_collateral_mana))
                }
                Err(e) => {
                    env::log_str("Failed to parse JSON response from Aurora.");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                env::log_str("Failed to fetch data from Aurora: Network timeout or error.");
                Err(Box::new(e))
            }
        }
    }
    
    // Voting function that uses governance power for proposal voting
    pub async fn vote(&mut self, voting_type: ProposalType, vote_id: u64, vote: bool) {
        let account_id = env::predecessor_account_id();
        
        match voting_type {
            ProposalType::Governance => {
                let mut proposal = self.proposals.get(&vote_id).expect("Proposal not found");
                let voting_power = self.get_governance_voting_power(&account_id).await;
                assert!(voting_power > 0, "No voting power for this proposal");

                if vote {
                    proposal.votes_for += voting_power;
                } else {
                    proposal.votes_against += voting_power;
                }

                if proposal.votes_for > proposal.votes_against * 2 {
                    proposal.status = ProposalStatus::Approved;
                } else if proposal.votes_against >= proposal.votes_for {
                    proposal.status = ProposalStatus::Rejected;
                }

                self.proposals.insert(&vote_id, &proposal);
            },
            ProposalType::Project { project_id } => {
                let mut project_vote = self.project_plan_votes.get(&project_id).expect("Project vote not found");
                let voting_power = self.get_project_voting_power(&account_id, project_id);
                assert!(voting_power > 0, "No voting power for this project");

                if vote {
                    project_vote.votes_for += voting_power;
                } else {
                    project_vote.votes_against += voting_power;
                }

                if project_vote.votes_for > project_vote.votes_against * 2 {
                    project_vote.status = ProjectPlanStatus::Approved;
                } else if project_vote.votes_against >= project_vote.votes_for {
                    project_vote.status = ProjectPlanStatus::Rejected;
                }

                self.project_plan_votes.insert(&project_id, &project_vote);
            }
        }
    }
}
