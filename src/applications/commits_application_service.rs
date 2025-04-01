use crate::domain::commit::{Commit, CommitToCreate};
use crate::domain::commit_handler::CommitHandler;
use crate::domain::commits_repository::CommitsRepository;
use crate::domain::current_branch_repository::CurrentBranchRepository;
use crate::domain::tree_repository::TreeRepository;

pub struct CommitsApplicationService {
    commit_handler: CommitHandler,
}

impl CommitsApplicationService {
    pub fn new(
        commit_repository: Box<dyn CommitsRepository>,
        current_branch_repository: Box<dyn CurrentBranchRepository>,
        tree_repository: Box<dyn TreeRepository>
    ) -> Self {
        Self {
            commit_handler: CommitHandler::new(commit_repository, current_branch_repository, tree_repository),
        }
    }

    pub fn save(&self, to_create: CommitToCreate) -> Commit {
        self.commit_handler.create_commit(to_create)
    }

    pub fn get_commits(&self, branch_name: String) -> Vec<Commit> {
        self.commit_handler.get_commits(branch_name)
    }
}
