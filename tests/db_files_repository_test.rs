const TEST_DB_PATH: &str = "tests/git-rust-data";
mod commit_fixtures;
mod file_shared;
mod tree_fixtures;

#[cfg(test)]
mod db_files_repository_test {
    use git_rust::infrastructure::secondary::db_files_repository::DBFilesRepository;
    use crate::TEST_DB_PATH;
    use crate::tree_fixtures::tree_root;

    #[test]
    fn test_save_commit() {
        let db_repository = DBFilesRepository::new(TEST_DB_PATH.to_string());

        let tree = db_repository.get_state();

        assert_eq!(tree, tree_root());
    }
}