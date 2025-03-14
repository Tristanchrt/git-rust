use std::fs::OpenOptions;
use crate::domain::branch::Branch;
use crate::domain::branches_repository::BranchesRepository;
use crate::infrastructure::secondary::branch_entity::BranchEntity;
use std::io::Write;
use crate::infrastructure::secondary::commit_entity::CommitEntity;

pub struct DBBranchesRepository {
    path: String
}


impl DBBranchesRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn save_to_file(&self, branch: &Branch) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.path)
            .expect("Couldn't open file");

        if let Err(e) = writeln!(file, "{}", BranchEntity::from(branch).to_string()) {
            panic!("Couldn't save commit: {}", e);
        }
    }

    pub fn get_branches(&self) -> Vec<Branch> {
        let file = std::fs::read_to_string(&self.path).unwrap();

        file.lines().map(|line| BranchEntity::from_string(line).to_domain()).collect()
    }
}

impl BranchesRepository for DBBranchesRepository {

    fn save(&self, branch: &Branch) {
       self.save_to_file(branch)
    }

    fn get_current_branch(&self) -> Branch {
        todo!()
    }

    fn get_branches(&self) -> Vec<Branch> {
        self.get_branches()
    }
}