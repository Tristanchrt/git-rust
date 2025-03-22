use std::io::BufRead;
mod commit_fixtures;
mod file_shared;

// TODO configuration file
const TEST_DB_PATH: &str = "tests/db/commits_test.txt";

#[cfg(test)]
mod commit_test {
    use git_rust::domain::commits_repository::CommitsRepository;
    use git_rust::infrastructure::secondary::db_commits_repository::DBCommitsRepository;
    use crate::commit_fixtures::{sample_commit, sample_commit_three, sample_commit_two};
    use crate::file_shared::{clean_file, read_file_line};
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

        assert_eq!(read_file_line(TEST_DB_PATH.to_string(), 0), "936da01f-9abd-4d9d-80c7-02af85c822a8,936da01f-9abd-4d9d-80c7-02af85c822a7,Init commit,2023-01-01 12:00:00,toto");
    }

    #[test]
    fn test_get_last_commit() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCommitsRepository::new(TEST_DB_PATH.to_string());
        let commit = sample_commit();
        db_repository.save(&commit);
        db_repository.save(&sample_commit_three());
        let last_commit = db_repository.get_last_commit("toto".to_string()).unwrap();

        assert!(last_commit.eq(&commit));
    }

    #[test]
    fn test_get_last_commit_find_none() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCommitsRepository::new(TEST_DB_PATH.to_string());
        let last_commit = db_repository.get_last_commit("toto".to_string());

        assert!(last_commit.is_none());
    }

    #[test]
    fn test_get_commits_from_branch() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCommitsRepository::new(TEST_DB_PATH.to_string());
        let commit = sample_commit();
        let commit_two = sample_commit_two();
        db_repository.save(&commit);
        db_repository.save(&commit_two);

        let commits = db_repository.get_commits("toto".to_string());

        assert!(commits.get(0).unwrap().eq(&commit));
        assert!(commits.get(1).unwrap().eq(&commit_two));
    }
}
