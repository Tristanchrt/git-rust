use std::fs::{File, OpenOptions};
use std::io::{BufRead, Read};

pub fn read_file_line(path: String, index: u16) -> String {
    File::open(path)
        .and_then(|file| {
            let lines = std::io::BufReader::new(file).lines();
            lines.skip(index as usize).next().unwrap()
        })
        .unwrap()
}

pub fn clean_file(path: String) {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
}

const TEST_DB_PATH: &str = "tests/db/commits_test.txt";

#[cfg(test)]
mod commit_test {
    use git_rust::domain::commit::Commit;
    use git_rust::domain::commits_repository::CommitsRepository;
    use git_rust::infrastructure::secondary::db_commits_repository::DBCommitsRepository;
    use super::*;

    #[test]
    #[should_panic(expected = "Couldn't open file")]
    fn test_panic_when_file_not_found() {
        let db_repository = DBCommitsRepository::new("toto".to_string());
        let commit = Commit::new(
            "1".to_string(),
            "0".to_string(),
            "Init commit".to_string(),
            chrono::Local::now().naive_local()
        );
        db_repository.save(&commit);
    }

    #[test]
    fn test_save_commit() {
        let db_repository = DBCommitsRepository::new(TEST_DB_PATH.to_string());
        let commit = Commit::new(
            "1".to_string(),
            "0".to_string(),
            "Init commit".to_string(),
            chrono::Local::now().naive_local()
        );
        db_repository.save(&commit);
        assert_eq!(read_file_line(TEST_DB_PATH.to_string(), 0), commit.to_string());
        clean_file(TEST_DB_PATH.to_string());
    }
}
