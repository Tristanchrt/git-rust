use crate::domain::branch::{Branch, BranchToCreate};
use crate::domain::branches_repository::BranchesRepository;
use crate::domain::current_branch_repository::CurrentBranchRepository;

pub struct BranchHandler {
    branches_repository: Box<dyn BranchesRepository>,
    current_branch_repository: Box<dyn CurrentBranchRepository>,
}

impl BranchHandler {

    pub fn new(branches_repository: Box<dyn BranchesRepository>,
               current_branch_repository: Box<dyn CurrentBranchRepository>) -> Self {
        Self {
            branches_repository,
            current_branch_repository
        }
    }

    pub fn checkout(&self, branch_name: String) -> Branch {
        let branch = self.branches_repository.get_by_name(branch_name);
        match branch {
            Some(value) => {
                self.current_branch_repository.save(&value);
                value
            },
            None => {
                panic!("Branch not found");
            }
        }
    }

    pub fn create_branch(&self, branch_to_create: BranchToCreate) -> Branch {
        let branch = branch_to_create.create();
        self.branches_repository.save(&branch);
        branch
    }
}