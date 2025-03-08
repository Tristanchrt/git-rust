use crate::domain::commit::Commit;

pub trait CommitsRepository {
    fn save(&self, commit: &Commit);
}