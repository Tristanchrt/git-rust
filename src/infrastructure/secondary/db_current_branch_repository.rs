use crate::domain::branch::Branch;
use crate::domain::current_branch_repository::CurrentBranchRepository;
use crate::infrastructure::secondary::branch_entity::BranchEntity;
use std::fs::OpenOptions;
use std::io::Write;

pub struct DBCurrentBranchRepository {
    path: String,
}

impl DBCurrentBranchRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn save_to_file(&self, branch: &Branch) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(false)
            .open(&self.path)
            .expect("Couldn't open file");

        if let Err(e) = writeln!(file, "{}", BranchEntity::from(branch).to_string()) {
            panic!("Couldn't save commit: {}", e);
        }
    }

    pub fn get(&self) -> Option<Branch> {
        let file = std::fs::read_to_string(&self.path).unwrap();

        file.lines()
            .last()
            .map(|branch| BranchEntity::from_string(branch).to_domain())
    }
}

impl CurrentBranchRepository for DBCurrentBranchRepository {
    fn save(&self, branch: &Branch) {
        self.save_to_file(branch)
    }

    fn get(&self) -> Option<Branch> {
        self.get()
    }
}
