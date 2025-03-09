use std::fs::OpenOptions;
use crate::domain::commit::Commit;
use crate::domain::commits_repository::CommitsRepository;
use std::io::{BufRead, BufReader, Write};
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
        // TODO
        // let file = OpenOptions::new().read(true).open(&self.path).ok()?;
        //
        // let lines: Vec<String> = BufReader::new(file)
        //     .lines().last().iter().last().map(|line| line.to_string())?;
        //
        // let last_line = lines.last();
        let last_line = "936da01f-9abd-4d9d-80c7-02af85c822a8,936da01f-9abd-4d9d-80c7-02af85c822a7,totoot,2025-03-09 11:46:12.170356";
        Some(CommitEntity::from_string(last_line).to_domain())
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