use crate::domain::branch::{Branch, BranchToCreate};
use crate::domain::branch_handler::BranchHandler;
use crate::domain::branches_repository::BranchesRepository;
use crate::domain::current_branch_repository::CurrentBranchRepository;

pub struct BranchesApplicationService {
    branch_handler: BranchHandler,
}

impl BranchesApplicationService {
    pub fn new(
        branches_repository: Box<dyn BranchesRepository>,
        current_branch_repository: Box<dyn CurrentBranchRepository>,
    ) -> Self {
        Self {
            branch_handler: BranchHandler::new(branches_repository, current_branch_repository),
        }
    }

    pub fn checkout(&self, branch_name: String) -> Branch {
        self.branch_handler.checkout(branch_name)
    }

    pub fn save(&self, to_create: BranchToCreate) -> Branch {
        self.branch_handler.create_branch(to_create)
    }
}
