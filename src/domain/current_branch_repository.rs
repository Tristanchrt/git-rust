use crate::domain::branch::Branch;

pub trait CurrentBranchRepository {
    fn save(&self, branch: &Branch);
    fn get(&self) -> Option<Branch>;
}