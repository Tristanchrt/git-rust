mod branch_fixtures;
mod file_shared;

// TODO configuration file
const TEST_DB_PATH: &str = "tests/db/current_branch_test.txt";

#[cfg(test)]
mod current_branch_test {
    use git_rust::domain::current_branch_repository::CurrentBranchRepository;
    use git_rust::infrastructure::secondary::db_current_branch_repository::DBCurrentBranchRepository;
    use crate::branch_fixtures::sample_branch;
    use crate::file_shared::{clean_file, read_file_line};
    use super::*;

    #[test]
    #[should_panic(expected = "Couldn't open file")]
    fn test_panic_when_file_not_found() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCurrentBranchRepository::new("toto".to_string());
        let branch = sample_branch();
        db_repository.save(&branch);
    }

    #[test]
    fn test_should_save_branch() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCurrentBranchRepository::new(TEST_DB_PATH.to_string());
        let branch = sample_branch();
        db_repository.save(&branch);

        assert_eq!(read_file_line(TEST_DB_PATH.to_string(), 0), "toto,2023-01-01 12:00:00");

        clean_file(TEST_DB_PATH.to_string());
    }

    #[test]
    fn test_should_get_branch() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCurrentBranchRepository::new(TEST_DB_PATH.to_string());
        let branch = sample_branch();
        db_repository.save(&branch);

        let current_branch = db_repository.get().unwrap();

        assert!(branch.eq(&current_branch));

        clean_file(TEST_DB_PATH.to_string());
    }

    #[test]
    fn test_should_get_non_when_no_current_branch() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBCurrentBranchRepository::new(TEST_DB_PATH.to_string());
        let branch = sample_branch();

        let current_branch = db_repository.get();

        assert!(current_branch.is_none());

    }
}
