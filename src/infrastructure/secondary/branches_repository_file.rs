use std::fs::{File, OpenOptions};

pub struct BranchesRepositoryFile;

const DB_PATH: &str = "db/branches.txt";

impl BranchesRepositoryFile {

    pub fn write_file() -> File {
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(DB_PATH.to_string())
            .expect("Couldn't open file")
    }

    pub fn read_string() -> String {
        std::fs::read_to_string(DB_PATH.to_string()).unwrap()
    }
}