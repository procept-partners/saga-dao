use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, AccountId, require, near_bindgen};
use near_sdk::json_types::U128;
use async_trait::async_trait;
use near_sdk::serde::{Deserialize, Serialize};
use std::error::Error;
use crate::mana_structs::ManaBalancesProof; // Import ManaBalancesProof here
//use aurora_engine_sdk::proof::verify_proof;

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

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectContribution {
    pub account_id: AccountId,
    pub contribution_amount: u64,
    pub timestamp: u64,
}

#[async_trait]
pub trait AuroraIntegration {
    async fn get_aurora_mana_balances(&self, shld_holder_id: &AccountId) -> Result<(u128, u128), Box<dyn Error>>;
    async fn get_circulating_supply(&self) -> Result<(u128, u128), Box<dyn Error>>;
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

// VotingModule definition
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VotingModule {
    pub shld_holders: UnorderedMap<AccountId, u64>,
    pub project_plan_votes: UnorderedMap<u64, ProjectPlanVote>,
    pub project_execution_votes: UnorderedMap<u64, ProjectExecutionVote>,
    pub project_contributions: UnorderedMap<AccountId, Vec<ProjectContribution>>,
    pub proposals: UnorderedMap<String, Vec<u8>>,
    // GovernanceDataContract fields
    pub governance_data: UnorderedMap<AccountId, GovernanceData>,
}

impl Default for VotingModule {
    fn default() -> Self {
        Self {
            shld_holders: UnorderedMap::new(b"s"),           // 's' for shld_holders
            project_plan_votes: UnorderedMap::new(b"v"),     // 'p' for project_plan_votes
            project_execution_votes: UnorderedMap::new(b"e"), // 'e' for execution_votes
            project_contributions: UnorderedMap::new(b"p"),   // 'c' for contributions
            proposals: UnorderedMap::new(b"r"),              // 'r' for proposals
            governance_data: UnorderedMap::new(b"g"),
        }
    }
}

#[near_bindgen]
impl VotingModule {
    #[init]
    pub fn init_voting() -> Self {
        Self {
            shld_holders: UnorderedMap::new(b"s"),
            project_plan_votes: UnorderedMap::new(b"v"),
            project_execution_votes: UnorderedMap::new(b"e"),
            project_contributions: UnorderedMap::new(b"p"),
            proposals: UnorderedMap::new(b"r"),
            governance_data: UnorderedMap::new(b"g"),
        }
    }

    // Decodes proof data and returns necessary fields for verification
    fn decode_proof(proof: ManaBalancesProof) -> Result<(U128, U128, u64, AccountId), String> {
        let mana_balance = proof.mana_balance;
        let collateral_balance = proof.collateral_mana_balance;
        let voting_power = proof.voting_power;
        let signer_address = proof.signer_address.clone();

        // AccountId validation - check if it's the zero address or invalid format
        if signer_address.to_string() == "" {
            return Err("Invalid proof: signer address is missing.".to_string());
        }

        Ok((mana_balance, collateral_balance, voting_power, signer_address))
    }

    // Verifies the Aurora proof of mana and collateralized mana balances for governance voting
    pub fn verify_aurora_proof(
        &self,
        proof: ManaBalancesProof,
        account_id: AccountId,
    ) -> bool {

        // Check signature first before decoding
        if proof.signature.is_empty() {
            env::log_str("Invalid proof: empty signature");
            return false;
        }

        // Step 1: Decode and map the proof data
        let (mana_balance, collateral_balance, voting_power, signer_address) =
            match Self::decode_proof(proof) {
                Ok(decoded_data) => decoded_data,
                Err(e) => {
                    env::log_str(&format!("Failed to decode proof: {}", e));
                    return false;
                }
            };
    
        // Step 2: Construct message for verification
        let message = format!("{}{}{}", account_id, mana_balance.0, collateral_balance.0);
        let message_hash = env::sha256(message.as_bytes());

        // Step 3: Basic validation checks
        if mana_balance.0 == 0 && collateral_balance.0 == 0 {
            env::log_str("Invalid proof: zero balances");
            return false;
        }
    
        if voting_power == 0 {
            env::log_str("Invalid proof: zero voting power");
            return false;
        }

        // TODO: Implement proper signature verification
        // For now, return true if basic validations pass
        env::log_str("Basic proof validation passed");
        true
    
        // Step 3: Verify proof using the signer's address and signature
        /*match verify_proof(&signer_address, &message_hash, &proof.signature) {
            true => {
                env::log_str("Signature verified, proof is trusted");
                true
            }
            false => {
                env::log_str("Signature verification failed, proof is untrusted");
                false
            }
        }*/
    }

    // Add methods for managing contributions
    pub fn add_project_contribution(&mut self, account_id: AccountId, amount: u64) {
        let contribution = ProjectContribution {
            account_id: account_id.clone(),
            contribution_amount: amount,
            timestamp: env::block_timestamp(),
        };

        let mut contributions = self.project_contributions
            .get(&account_id)
            .unwrap_or_else(|| Vec::new());
        
        contributions.push(contribution);
        self.project_contributions.insert(&account_id, &contributions);
    }

    pub fn get_project_contributions(&self, account_id: AccountId) -> Option<Vec<ProjectContribution>> {
        self.project_contributions.get(&account_id)
    }

    // Additional functions for governance voting power, project voting power, and other related logic...


    // GovernanceDataContract methods
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
            if data.mana_balance != mana_balance {
                env::log_str("Verification failed: MANA balance mismatch.");
                return false;
            }
            if data.mana_collateral_balance != mana_collateral_balance {
                env::log_str("Verification failed: Collateral MANA balance mismatch.");
                return false;
            }
            if data.voting_power != voting_power {
                env::log_str("Verification failed: Voting power mismatch.");
                return false;
            }
            if data.transaction_id != Some(transaction_id) {
                env::log_str("Verification failed: Transaction ID mismatch.");
                return false;
            }
            true
        } else {
            env::log_str("Verification failed: No governance data for account.");
            false
        }
    }
}

/*
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

impl Default for GovernanceDataContract {
    fn default() -> Self {
        Self {
            governance_data: UnorderedMap::new(b"g"),
        }
    }
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
            if data.mana_balance != mana_balance {
                env::log_str("Verification failed: MANA balance mismatch.");
                return false;
            }
            if data.mana_collateral_balance != mana_collateral_balance {
                env::log_str("Verification failed: Collateral MANA balance mismatch.");
                return false;
            }
            if data.voting_power != voting_power {
                env::log_str("Verification failed: Voting power mismatch.");
                return false;
            }
            if data.transaction_id != Some(transaction_id) {
                env::log_str("Verification failed: Transaction ID mismatch.");
                return false;
            }
            true
        } else {
            env::log_str("Verification failed: No governance data for account.");
            false
        }
    }
}
*/