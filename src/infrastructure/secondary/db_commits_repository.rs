use crate::domain::commit::Commit;
use crate::domain::commits_repository::CommitsRepository;

pub struct DBCommitsRepository;

impl CommitsRepository for DBCommitsRepository {
    fn save(&self, commit: &Commit) {
        println!("Saving commit {:?}", commit);
    }
}