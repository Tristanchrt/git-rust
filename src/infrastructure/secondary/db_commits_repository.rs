use std::fs::OpenOptions;
use crate::domain::commit::Commit;
use crate::domain::commits_repository::CommitsRepository;
use std::io::Write;
use crate::infrastructure::secondary::commit_entity::CommitEntity;

pub struct DBCommitsRepository {
    path: String
}

impl DBCommitsRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    fn save_to_file(&self, commit: &Commit) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)
            .expect("Couldn't open file");

        if let Err(e) = writeln!(file, "{}", CommitEntity::from(commit).to_string()) {
            panic!("Couldn't save commit: {}", e);
        }
    }
}

impl CommitsRepository for DBCommitsRepository {

    fn save(&self, commit: &Commit) {
        self.save_to_file(commit);
    }
}