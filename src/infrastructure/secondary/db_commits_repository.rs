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

    fn get_last_commit(&self) -> Option<Commit> {
        let file = std::fs::read_to_string(&self.path).unwrap();

        match file.lines().last() {
            Some(last_line) => Some(CommitEntity::from_string(last_line).to_domain()),
            None => None
        }
    }
}

impl CommitsRepository for DBCommitsRepository {

    fn save(&self, commit: &Commit) {
        self.save_to_file(commit);
    }

    fn get_last_commit(&self) -> Option<Commit> {
        self.get_last_commit()
    }
}