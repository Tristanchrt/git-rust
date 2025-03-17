use crate::domain::branch::{Branch, BranchToCreate};
use crate::domain::branch_handler::BranchHandler;
use crate::domain::branches_repository::BranchesRepository;

pub struct BranchesApplicationService {
    db_repository: Box<dyn BranchesRepository>,
}

impl BranchesApplicationService {

    pub fn new(db_repository: Box<dyn BranchesRepository>) -> Self {
        Self { db_repository }
    }

    pub fn save(&self, to_create: BranchToCreate) -> Branch {
        let branch = BranchHandler::create_branch(to_create);
        self.db_repository.save(&branch);

        branch
    }
}