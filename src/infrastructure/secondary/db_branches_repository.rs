use crate::domain::branch::Branch;
use crate::domain::branches_repository::BranchesRepository;
use crate::infrastructure::secondary::branch_entity::BranchEntity;
use std::fs::OpenOptions;
use std::io::Write;

pub struct DBBranchesRepository {
    path: String,
}
impl DBBranchesRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn save_to_file(&self, branch: &Branch) {
        let mut file = OpenOptions::new()
            
            .append(true)
            .open(&self.path)
            .expect("Couldn't open file");

        if let Err(e) = writeln!(file, "{}", BranchEntity::from(branch).to_string()) {
            panic!("Couldn't save commit: {}", e);
        }
    }

    pub fn get_branches(&self) -> Vec<Branch> {
        let file = std::fs::read_to_string(&self.path).unwrap();

        file.lines()
            .map(|line| BranchEntity::from_string(line).to_domain())
            .collect()
    }

    pub fn get_by_name(&self, branch_name: String) -> Option<Branch> {
        let file = std::fs::read_to_string(&self.path).unwrap();

        file.lines()
            .map(|line| BranchEntity::from_string(line).to_domain())
            .filter(|branch| branch.name().eq(&branch_name))
            .last()
    }
}

impl BranchesRepository for DBBranchesRepository {
    fn save(&self, branch: &Branch) {
        self.save_to_file(branch)
    }

    fn get_branches(&self) -> Vec<Branch> {
        self.get_branches()
    }

    fn get_by_name(&self, branch_name: String) -> Option<Branch> {
        self.get_by_name(branch_name)
    }
}
