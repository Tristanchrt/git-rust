use crate::domain::branch::{Branch, BranchToCreate};
use crate::domain::branches_repository::BranchesRepository;
use crate::domain::commits_repository::CommitsRepository;
use crate::domain::current_branch_repository::CurrentBranchRepository;
use crate::domain::tree_handler::TreeHandler;

pub struct BranchHandler {
    branches_repository: Box<dyn BranchesRepository>,
    current_branch_repository: Box<dyn CurrentBranchRepository>,
    commits_repository: Box<dyn CommitsRepository>,
    tree_handler: TreeHandler
}

impl BranchHandler {
    pub fn new(
        branches_repository: Box<dyn BranchesRepository>,
        current_branch_repository: Box<dyn CurrentBranchRepository>,
        commits_repository: Box<dyn CommitsRepository>,
        tree_handler: TreeHandler
    ) -> Self {
        Self {
            branches_repository,
            current_branch_repository,
            commits_repository,
            tree_handler
        }
    }

    pub fn checkout(&self, branch_name: String) -> Branch {
        // TODO restore file from commit in current state

        let branch = self.branches_repository.get_by_name(branch_name);
        match branch {
            Some(value) => {
                let commit = self.commits_repository.get_last_commit(value.name()).expect("Commit to restore not found");
                self.tree_handler.restore_commit(commit);
                self.current_branch_repository.save(&value);
                value
            }
            None => {
                panic!("Branch not found");
            }
        }
    }

    pub fn create_branch(&self, branch_to_create: BranchToCreate) -> Branch {
        if self
            .branches_repository
            .get_by_name(branch_to_create.name())
            .is_some()
        {
            panic!("Branch name already taken");
        }

        let branch = branch_to_create.create();
        self.branches_repository.save(&branch);
        branch
    }
}
