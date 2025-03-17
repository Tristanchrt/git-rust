use crate::domain::branch::{Branch, BranchToCreate};
use crate::domain::branch_handler::BranchHandler;
use crate::domain::branches_repository::BranchesRepository;
use crate::domain::current_branch_repository::CurrentBranchRepository;

pub struct BranchesApplicationService {
    branches_repository: Box<dyn BranchesRepository>,
    current_branch_repository: Box<dyn CurrentBranchRepository>,
}

impl BranchesApplicationService {

    pub fn new(branches_repository: Box<dyn BranchesRepository>,
               current_branch_repository: Box<dyn CurrentBranchRepository>) -> Self {
        Self {
            branches_repository,
            current_branch_repository
        }
    }

    pub(crate) fn checkout(&self, branch_name: String) -> Branch {
        todo!()
    }

    pub fn save(&self, to_create: BranchToCreate) -> Branch {
        let branch = BranchHandler::create_branch(to_create);
        self.branches_repository.save(&branch);

        branch
    }
}