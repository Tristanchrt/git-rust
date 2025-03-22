use crate::domain::commit::Commit;

pub trait CommitsRepository {
    fn save(&self, commit: &Commit);
    fn get_last_commit(&self, branch_name: String) -> Option<Commit>;
    fn get_commits(&self, branch_name: String) -> Vec<Commit>;
}