use crate::domain::branch::Branch;

pub trait BranchesRepository {
    fn save(&self, branch: &Branch);
    fn get_branches(&self) -> Vec<Branch>;
}