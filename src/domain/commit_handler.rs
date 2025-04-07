use crate::domain::commit::{Commit, CommitToCreate};
use crate::domain::commits_repository::CommitsRepository;
use crate::domain::current_branch_repository::CurrentBranchRepository;
use crate::domain::tree_handler::TreeHandler;

pub struct CommitHandler {
    commit_repository: Box<dyn CommitsRepository>,
    current_branch_repository: Box<dyn CurrentBranchRepository>,
    tree_handler: TreeHandler
}

impl CommitHandler {
    pub fn new(
        commit_repository: Box<dyn CommitsRepository>,
        current_branch_repository: Box<dyn CurrentBranchRepository>,
        tree_handler: TreeHandler,
    ) -> Self {
        Self {
            commit_repository,
            current_branch_repository,
            tree_handler
        }
    }

    pub fn create_commit(&self, to_create: CommitToCreate) -> Commit {
        let branch = self
            .current_branch_repository
            .get()
            .expect("Current branch not found");

        let parent_commit_id = self
            .commit_repository
            .get_last_commit(branch.name())
            .map(|commit| commit.id().to_owned())
            .unwrap_or_else(CommitToCreate::default_parent_id);

        let tree_hash = self.tree_handler.save_hash_tree();
        let commit = to_create.create(parent_commit_id, branch.name(), tree_hash.complete_hash());
        self.commit_repository.save(&commit);

        commit
    }

    pub fn restore_commit(&self, commit_to_restore: Commit) {
        self.tree_handler.restore_commit(commit_to_restore)
    }

    pub fn get_commits(&self, branch_name: String) -> Vec<Commit> {
        self.commit_repository.get_commits(branch_name)
    }
}
