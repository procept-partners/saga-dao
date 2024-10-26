use std::collections::HashMap;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::json_types::U128;
use near_sdk::{AccountId, BorshDeserialize, BorshSerialize};

// Enum for tracking the lifecycle of a task from planning through execution
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum TaskStatus {
    Planned,       // Task is planned but not yet started
    NotStarted,    // Task is ready but hasn't begun
    InProgress,    // Task is currently being worked on
    Completed,     // Task has been completed
    Rejected,      // Task was rejected or canceled
}

// Main struct for Proposal, with optional parent ID for hierarchical governance-project relationships
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub mana_tokens_allocated: U128,
    pub is_ended: bool,
    pub submitted_by: AccountId,
    pub mana_hours_budgeted: u64,
    pub target_date: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub parent_id: Option<u64>, // Link to a parent governance proposal if this is a project proposal
    pub sub_projects: Vec<SubProject>,
    pub budget_items: Vec<ProposalBudget>,
}

// Proposal Budget details within a proposal
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalBudget {
    pub id: u64,
    pub proposal_id: u64,
    pub role_name: String,
    pub budget_usd: U128,
    pub budget_mana: U128,
}

// SubProject struct within a proposal
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SubProject {
    pub id: u64,
    pub proposal_id: Option<u64>, // Optional to support nested subprojects linked to a parent proposal
    pub sub_project_name: String,
    pub epics: Vec<Epic>,
}

// Epic struct within a SubProject
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Epic {
    pub id: u64,
    pub sub_project_id: Option<u64>,
    pub epic_name: String,
    pub tasks: Vec<Task>,
}

// Task struct within an Epic with a unified status field
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Task {
    pub id: u64,
    pub epic_id: Option<u64>,
    pub task_name: String,
    pub roles_mana_hours: Vec<TaskRoleManaHours>,
    pub status: TaskStatus, // Unified status field for both planning and execution phases
}

// Role Mana Hours within a Task
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskRoleManaHours {
    pub id: u64,
    pub task_id: u64,
    pub role_name: String,
    pub mana_hours: u64,
}

// Main struct for Project Plan
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectPlan {
    pub id: u64,
    pub proposal_id: Option<u64>, // Optional to support standalone project plans
    pub project_name: String,
    pub total_mana_hours: u64,
    pub voting_power: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub developers: HashMap<String, DeveloperProjectPlan>,
    pub proposal: Option<Proposal>,
}

// Developer-specific project plan details
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct DeveloperProjectPlan {
    pub developer_name: String,
    pub mana_hours_budgeted: u64,
    pub mana_token_allocated: U128,
    pub sub_projects: Vec<SubProjectPlan>,
}

// Struct for SubProjectPlan within a project plan
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct SubProjectPlan {
    pub id: u64,
    pub project_plan_id: u64,
    pub sub_project_name: String,
    pub epics: Vec<EpicPlan>,
}

// Struct for EpicPlan within a subproject plan
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EpicPlan {
    pub id: u64,
    pub sub_project_plan_id: u64,
    pub epic_name: String,
    pub tasks: Vec<TaskPlan>,
}

// Struct for TaskPlan within an epic plan, now with unified status tracking
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskPlan {
    pub id: u64,
    pub epic_plan_id: u64,
    pub task_name: String,
    pub estimated_mana_hours: u64,
    pub roles_mana_hours: Vec<TaskRoleManaHours>,
    pub status: TaskStatus, // Unified status field for the task's lifecycle
}

// Main struct for Project Execution, linked to ProjectPlan by project_plan_id
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectExecution {
    pub id: u64,
    pub project_plan_id: u64, // Link to parent ProjectPlan
    pub actual_mana_hours: u64,
    pub tasks: Vec<TaskExecution>, // Tracks task execution status based on ProjectPlan tasks
    pub peer_votes: Vec<PeerVote>,
}

// Task Execution struct within a project execution, linked to TaskPlan
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskExecution {
    pub id: u64,
    pub project_execution_id: u64,
    pub task_plan_id: u64, // Link to specific TaskPlan in ProjectPlan
    pub actual_mana_hours: u64,
    pub status: TaskStatus, // Unified status for tracking task execution progress
}

// Peer vote struct for project execution feedback
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PeerVote {
    pub id: u64,
    pub project_execution_id: u64,
    pub user_id: u64,
    pub vote: bool,
    pub created_at: String,
}

// Task Feedback struct within task execution
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskFeedback {
    pub id: u64,
    pub task_execution_id: u64,
    pub user_id: u64,
    pub feedback: String,
    pub rating: u8,
    pub created_at: String,
}
