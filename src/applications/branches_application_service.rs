use crate::domain::branch::{Branch, BranchToCreate};
use crate::domain::branch_handler::BranchHandler;
use crate::domain::branches_repository::BranchesRepository;
use crate::domain::commits_repository::CommitsRepository;
use crate::domain::current_branch_repository::CurrentBranchRepository;
use crate::domain::files_repository::FilesRepository;
use crate::domain::tree_handler::TreeHandler;
use crate::domain::tree_repository::TreeRepository;

pub struct BranchesApplicationService {
    branch_handler: BranchHandler,
}

impl BranchesApplicationService {
    pub fn new(
        branches_repository: Box<dyn BranchesRepository>,
        current_branch_repository: Box<dyn CurrentBranchRepository>,
        commits_repository: Box<dyn CommitsRepository>,
        tree_repository: Box<dyn TreeRepository>,
        file_repository: Box<dyn FilesRepository>
    ) -> Self {
        Self {
            branch_handler: BranchHandler::new(branches_repository, current_branch_repository, commits_repository, TreeHandler::new(tree_repository, file_repository)),
        }
    }

    pub fn checkout(&self, branch_name: String) -> Branch {
        self.branch_handler.checkout(branch_name)
    }

    pub fn save(&self, to_create: BranchToCreate) -> Branch {
        self.branch_handler.create_branch(to_create)
    }
}
