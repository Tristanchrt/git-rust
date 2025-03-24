mod branch_fixtures;
mod file_shared;

// TODO configuration file
const TEST_DB_PATH: &str = "tests/db/branches_test.txt";

#[cfg(test)]
mod branches_repository_test {
    use super::*;
    use crate::branch_fixtures::sample_branch;
    use crate::file_shared::{clean_file, read_file_line};
    use git_rust::domain::branches_repository::BranchesRepository;
    use git_rust::infrastructure::secondary::db_branches_repository::DBBranchesRepository;

    #[test]
    #[should_panic(expected = "Couldn't open file")]
    fn test_panic_when_file_not_found() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBBranchesRepository::new("toto".to_string());
        let branch = sample_branch();
        db_repository.save(&branch);

        clean_file(TEST_DB_PATH.to_string());
    }

    #[test]
    fn test_should_save_branch() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBBranchesRepository::new(TEST_DB_PATH.to_string());
        let branch = sample_branch();
        db_repository.save(&branch);
        assert_eq!(
            read_file_line(TEST_DB_PATH.to_string(), 0),
            "toto,2023-01-01 12:00:00"
        );

        clean_file(TEST_DB_PATH.to_string());
    }

    #[test]
    fn test_should_get_branches() {
        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBBranchesRepository::new(TEST_DB_PATH.to_string());
        let branch = sample_branch();
        db_repository.save(&branch);
        db_repository.save(&branch);

        let branches = db_repository.get_branches();

        assert!(sample_branch().eq(branches.first().unwrap()));

        clean_file(TEST_DB_PATH.to_string());
    }

    #[test]
    fn test_should_get_branch_by_name() {
        // TODO test are run in parallel either disabled it or find another way
        // [dev-dependencies]
        // test-serial = "0.1"
        // #[serial]

        clean_file(TEST_DB_PATH.to_string());

        let db_repository = DBBranchesRepository::new(TEST_DB_PATH.to_string());
        let branch = sample_branch();
        db_repository.save(&branch);

        assert!(sample_branch().eq(&db_repository.get_by_name(branch.name()).unwrap()));

        clean_file(TEST_DB_PATH.to_string());
    }
}
