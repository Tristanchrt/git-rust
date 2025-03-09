use crate::domain::commit::Commit;

pub trait CommitsRepository {
    fn save(&self, commit: &Commit);
    fn get_last_commit(&self) -> Option<Commit>;
}