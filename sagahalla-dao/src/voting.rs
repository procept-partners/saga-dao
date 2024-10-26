// voting.rs

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use near_sdk::{AccountId, env, require};
use near_sdk::json_types::U128;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;
use crate::mana_structs::{ManaBalancesProof}; // Import ManaBalancesProof here
use aurora_engine_sdk::proof::verify_proof;

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

#[async_trait]
pub trait AuroraIntegration {
    async fn get_aurora_mana_balances(&self, shld_holder_id: &AccountId) -> Result<(u128, u128), Box<dyn Error>>;
    async fn get_circulating_supply(&self) -> Result<(u128, u128), Box<dyn Error>>;
}

// VotingModule definition
#[near_bindgen]
pub struct VotingModule {
    pub shld_holders: UnorderedMap<AccountId, u64>,
    pub project_plan_votes: UnorderedMap<u64, ProjectPlanVote>,
    pub project_execution_votes: UnorderedMap<u64, ProjectExecutionVote>,
}

#[near_bindgen]
impl VotingModule {
    // Verifies the Aurora proof of mana and collateralized mana balances for governance voting
    pub fn verify_aurora_proof(
        &self,
        proof: ManaBalancesProof,
        account_id: AccountId,
    ) -> bool {
        let message = format!("{}{}{}", account_id, proof.mana_balance.0, proof.collateral_mana_balance.0);
        let message_hash = env::sha256(message.as_bytes());

        match verify_proof(&proof.signer_address, &message_hash, &proof.signature) {
            true => {
                env::log_str("Signature verified, proof is trusted");
                true
            }
            false => {
                env::log_str("Signature verification failed, proof is untrusted");
                false
            }
        }
    }

    // Additional functions for governance voting power, project voting power, and other related logic...
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GovernanceData {
    mana_balance: U128,
    mana_collateral_balance: U128,
    voting_power: u64,
    transaction_id: Option<u64>, // New field for unique transaction identification
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct GovernanceDataContract {
    governance_data: UnorderedMap<AccountId, GovernanceData>,
}

#[near_bindgen]
impl GovernanceDataContract {
    #[init]
    pub fn new() -> Self {
        Self {
            governance_data: UnorderedMap::new(b"g"),
        }
    }

    pub fn update_governance_data(
        &mut self,
        account_id: AccountId,
        mana_balance: U128,
        mana_collateral_balance: U128,
        voting_power: u64,
        transaction_id: u64,
    ) {
        let data = GovernanceData {
            mana_balance,
            mana_collateral_balance,
            voting_power,
            transaction_id: Some(transaction_id),
        };
        self.governance_data.insert(&account_id, &data);

        env::log_str(&format!(
            "Updated governance data for account {}: mana_balance = {}, mana_collateral_balance = {}, voting_power = {}, transaction_id = {}",
            account_id,
            mana_balance.0,
            mana_collateral_balance.0,
            voting_power,
            transaction_id,
        ));
    }

    pub fn get_governance_data(&self, account_id: AccountId) -> Option<GovernanceData> {
        self.governance_data.get(&account_id)
    }

    pub fn emit_governance_update_event(&self, account_id: AccountId, block_timestamp: u64) {
        if let Some(data) = self.governance_data.get(&account_id) {
            env::log_str(&format!(
                "GovernanceDataUpdate: {{ account_id: {}, mana_balance: {}, mana_collateral_balance: {}, voting_power: {}, timestamp: {}, transaction_id: {:?} }}",
                account_id, data.mana_balance.0, data.mana_collateral_balance.0, data.voting_power, block_timestamp, data.transaction_id
            ));
        }
    }

    pub fn verify_cross_chain_data(
        &self,
        account_id: AccountId,
        mana_balance: U128,
        mana_collateral_balance: U128,
        voting_power: u64,
        transaction_id: u64,
    ) -> bool {
        if let Some(data) = self.governance_data.get(&account_id) {
            data.mana_balance == mana_balance &&
            data.mana_collateral_balance == mana_collateral_balance &&
            data.voting_power == voting_power &&
            data.transaction_id == Some(transaction_id)
        } else {
            env::log_str("Verification failed: data mismatch or missing transaction ID.");
            false
        }
    }
}
