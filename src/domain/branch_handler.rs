use crate::domain::branch::{Branch, BranchToCreate};

pub struct BranchHandler;

impl BranchHandler {
    pub fn create_branch(branch: BranchToCreate) -> Branch {
        branch.create()
    }
}