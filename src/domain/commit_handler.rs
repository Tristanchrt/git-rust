use crate::domain::commit::{Commit, CommitToCreate};
use crate::domain::commits_repository::CommitsRepository;

pub struct CommitHandler {
    db_repository: Box<dyn CommitsRepository>,
}

impl CommitHandler {

    pub fn new(db_repository: Box<dyn CommitsRepository>) -> Self {
        Self {
            db_repository
        }
    }

    pub fn create_commit(&self, to_create: CommitToCreate) -> Commit {
        let parent_commit_id = self.db_repository
            .get_last_commit()
            .map(|commit| commit.id().to_owned())
            .unwrap_or_else(|| CommitToCreate::default_parent_id());

        let commit = to_create.create(parent_commit_id);
        self.db_repository.save(&commit);

        commit
    }
}