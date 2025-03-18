use crate::domain::commit::{Commit, CommitToCreate};
use crate::domain::commit_handler::CommitHandler;
use crate::domain::commits_repository::CommitsRepository;

pub struct CommitsApplicationService {
    commit_handler: CommitHandler
}

impl CommitsApplicationService {

    pub fn new(db_repository: Box<dyn CommitsRepository>) -> Self {
        Self { commit_handler: CommitHandler::new(db_repository) }
    }

    pub fn save(&self, to_create: CommitToCreate) -> Commit {
        self.commit_handler.create_commit(to_create)
    }
}