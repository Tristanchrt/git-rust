use crate::domain::commit::{Commit, CommitToCreate};

pub struct CommitHandler {
    create_commit: String
}

impl CommitHandler {
    pub fn create_commit(commit: CommitToCreate) -> Commit {
        let parent_id = "0".to_string();
        CommitToCreate::create(commit, parent_id)
    }
}