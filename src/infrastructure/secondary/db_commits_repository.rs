use std::fs::OpenOptions;
use crate::domain::commit::Commit;
use crate::domain::commits_repository::CommitsRepository;
use std::io::Write;

pub struct DBCommitsRepository {
    path: String
}

impl DBCommitsRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl CommitsRepository for DBCommitsRepository {

    fn save(&self, commit: &Commit) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)
            .expect("Couldn't open file");

        // TODO to test
        if let Err(e) = writeln!(file, "{}", commit.to_string()) {
            panic!("Couldn't save commit: {}", e);
        }
    }
}