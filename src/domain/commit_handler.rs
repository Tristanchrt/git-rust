use crate::domain::commit::{Commit, CommitToCreate};
use crate::domain::commits_repository::CommitsRepository;
use crate::domain::current_branch_repository::CurrentBranchRepository;
use crate::domain::files_repository::FilesRepository;
use crate::domain::tree::TreeNodeTree;
use crate::domain::tree_repository::TreeRepository;

pub struct CommitHandler {
    commit_repository: Box<dyn CommitsRepository>,
    current_branch_repository: Box<dyn CurrentBranchRepository>,
    files_repository: Box<dyn FilesRepository>,
    tree_repository: Box<dyn TreeRepository>
}

impl CommitHandler {
    pub fn new(
        commit_repository: Box<dyn CommitsRepository>,
        current_branch_repository: Box<dyn CurrentBranchRepository>,
        files_repository: Box<dyn FilesRepository>,
        tree_repository: Box<dyn TreeRepository>
    ) -> Self {
        Self {
            commit_repository,
            current_branch_repository,
            files_repository,
            tree_repository
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

        let tree_node = self.files_repository.get_current_state();
        let tree_hash = TreeNodeTree::hash_tree(tree_node);
        let commit = to_create.create(parent_commit_id, branch.name(), tree_hash.complete_hash());
        self.commit_repository.save(&commit);
        self.tree_repository.save(tree_hash);

        commit
    }

    pub fn get_commits(&self, branch_name: String) -> Vec<Commit> {
        self.commit_repository.get_commits(branch_name)
    }
}
