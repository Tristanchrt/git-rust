use uuid::Uuid;
use crate::domain::commit::{Commit, CommitToCreate};

pub struct CommitHandler;

impl CommitHandler {
    pub fn create_commit(commit: CommitToCreate, parent_id: Uuid) -> Commit {
        commit.create(parent_id)
    }
}