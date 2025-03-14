use crate::domain::branch::Branch;

pub trait BranchesRepository {
    fn save(&self, commit: &Branch);
    fn get_current_branch(&self) -> Branch;
    fn get_branches(&self) -> Vec<Branch>;
}