use uuid::Uuid;
use crate::domain::commit::{Commit, CommitToCreate};

pub struct CommitHandler {
    create_commit: String
}

impl CommitHandler {
    pub fn create_commit(commit: CommitToCreate, parent_id: Uuid) -> Commit {
        CommitToCreate::create(commit, parent_id)
    }
}