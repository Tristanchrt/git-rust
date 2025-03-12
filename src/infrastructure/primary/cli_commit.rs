use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::domain::commit::{Commit, CommitToCreate};

pub struct CliCommitToCreate {
    pub message: String,
}

pub struct CliCommit {
    id: Uuid,
    parent_id: Uuid,
    message: String,
    created_at: NaiveDateTime
}

impl CliCommit {
    pub fn from(commit: Commit) -> Self {
        Self {
            id: commit.id().clone(),
            parent_id: commit.parent_id().clone(),
            message: commit.message().clone(),
            created_at: commit.created_at().clone()
        }
    }

    pub fn to_display(&self) -> String {
        format!("{} | {} | {} | {}", self.id, self.parent_id, self.created_at, self.message)
    }
}

impl CliCommitToCreate {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }

    pub fn to_domain(&self) -> CommitToCreate {
        CommitToCreate::new(self.message.clone())
    }
}