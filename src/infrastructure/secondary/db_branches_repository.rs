use std::fs::OpenOptions;
use crate::domain::branch::Branch;
use crate::domain::branches_repository::BranchesRepository;
use crate::infrastructure::secondary::branch_entity::BranchEntity;
use std::io::Write;
use crate::infrastructure::secondary::branches_repository_file::BranchesRepositoryFile;

pub struct DBBranchesRepository;

impl DBBranchesRepository {

    pub fn new() -> Self {
        Self
    }

    pub fn save_to_file(&self, branch: &Branch) {
        let mut file = BranchesRepositoryFile::write_file();

        if let Err(e) = writeln!(file, "{}", BranchEntity::from(branch).to_string()) {
            panic!("Couldn't save commit: {}", e);
        }
    }

    pub fn get_branches(&self) -> Vec<Branch> {
        let file = BranchesRepositoryFile::read_string();

        file.lines().map(|line| BranchEntity::from_string(line).to_domain()).collect()
    }

    pub fn get_by_name(&self, branch_name: String) -> Option<Branch> {
        let file = BranchesRepositoryFile::read_string();

        file.lines()
            .map(|line| BranchEntity::from_string(line).to_domain())
            .filter(|branch| branch.name().eq(&branch_name)).last()
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