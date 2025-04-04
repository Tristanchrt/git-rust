use crate::domain::commit::{Commit, CommitToCreate};
use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct CliCommitToCreate {
    pub message: String,
}

pub struct CliCommit {
    id: Uuid,
    parent_id: Uuid,
    message: String,
    created_at: NaiveDateTime,
    branch_id: String,
    tree_hash: String
}

pub struct CliCommits {
    value: Vec<CliCommit>,
}

impl CliCommit {
    pub fn from(commit: &Commit) -> Self {
        Self {
            id: commit.id(),
            parent_id: commit.parent_id(),
            message: commit.message().clone(),
            created_at: commit.created_at(),
            branch_id: commit.branch_id().clone(),
            tree_hash: commit.tree_hash().clone()
        }
    }

    pub fn to_display(&self) -> String {
        format!(
            "{} | {} | {} | {} | {} | {}",
            self.id, self.parent_id, self.created_at, self.message, self.branch_id, self.tree_hash
        )
    }
}

impl CliCommitToCreate {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn to_domain(&self) -> CommitToCreate {
        CommitToCreate::new(self.message.clone())
    }
}

impl CliCommits {
    pub fn from(commits: Vec<Commit>) -> Self {
        Self {
            value: commits
                .iter()
                .map(CliCommit::from)
                .collect(),
        }
    }

    pub fn to_display(&self) -> String {
        self.value
            .iter()
            .map(|commit| commit.to_display())
            .collect::<Vec<String>>()
            .join("\n ")
    }
}
