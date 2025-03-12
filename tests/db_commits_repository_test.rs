use std::fs::{File, OpenOptions};
use std::io::BufRead;
mod commit_fixtures;

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
    use git_rust::domain::commits_repository::CommitsRepository;
    use git_rust::infrastructure::secondary::db_commits_repository::DBCommitsRepository;
    use crate::commit_fixtures::sample_commit;
    use super::*;

    #[test]
    #[should_panic(expected = "Couldn't open file")]
    fn test_panic_when_file_not_found() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCommitsRepository::new("toto".to_string());
        let commit = sample_commit();
        db_repository.save(&commit);
    }

    #[test]
    fn test_save_commit() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCommitsRepository::new(TEST_DB_PATH.to_string());
        let commit = sample_commit();
        db_repository.save(&commit);
        assert_eq!(read_file_line(TEST_DB_PATH.to_string(), 0), "936da01f-9abd-4d9d-80c7-02af85c822a8,936da01f-9abd-4d9d-80c7-02af85c822a7,Init commit,2023-01-01 12:00:00");

        clean_file(TEST_DB_PATH.to_string());
    }

    #[test]
    fn test_get_last_commit() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCommitsRepository::new(TEST_DB_PATH.to_string());
        let commit = sample_commit();
        db_repository.save(&commit);
        let last_commit = db_repository.get_last_commit().unwrap();

        // TODO maybe use #[derive(PartialEq)] or impl PartialEq for Person
        assert_eq!(last_commit.id(), commit.id());
        assert_eq!(last_commit.message(), commit.message());
        assert_eq!(last_commit.parent_id(), commit.parent_id());
        assert_eq!(last_commit.created_at(), commit.created_at());

        clean_file(TEST_DB_PATH.to_string());
    }

    #[test]
    fn test_get_last_commit_find_none() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCommitsRepository::new(TEST_DB_PATH.to_string());
        let last_commit = db_repository.get_last_commit();

        assert!(last_commit.is_none());
    }
}
