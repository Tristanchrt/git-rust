use crate::domain::commit::Commit;
use crate::domain::commits_repository::CommitsRepository;
use crate::infrastructure::secondary::commit_entity::CommitEntity;
use std::fs::OpenOptions;
use std::io::Write;

pub struct DBCommitsRepository {
    path: String,
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

    fn get_last_commit(&self, branch_name: String) -> Option<Commit> {
        let file = std::fs::read_to_string(&self.path).unwrap();

        file.lines()
            .map(|line| CommitEntity::from_string(line).to_domain())
            .filter(|commit| commit.branch_id().eq(&branch_name))
            .last()
    }

    fn get_commits(&self, branch_name: String) -> Vec<Commit> {
        let file = std::fs::read_to_string(&self.path).unwrap();

        file.lines()
            .map(|line| CommitEntity::from_string(line).to_domain())
            .filter(|commit| commit.branch_id().eq(&branch_name))
            .collect()
    }
}

impl CommitsRepository for DBCommitsRepository {
    fn save(&self, commit: &Commit) {
        self.save_to_file(commit);
    }

    fn get_last_commit(&self, branch_name: String) -> Option<Commit> {
        self.get_last_commit(branch_name)
    }
    fn get_commits(&self, branch_name: String) -> Vec<Commit> {
        self.get_commits(branch_name)
    }
}
