const TEST_DB_PATH: &str = "tests/git-rust-data";
mod commit_fixtures;
mod file_shared;

#[cfg(test)]
mod db_files_repository_test {
    use git_rust::infrastructure::secondary::db_files_repository::DBFilesRepository;
    use crate::file_shared::read_file;
    use crate::TEST_DB_PATH;

    #[test]
    fn test_save_commit() {
        let db_repository = DBFilesRepository::new(TEST_DB_PATH.to_string());
        // db_repository.save("toto".to_string(), sample_commit().id());

        const PREFIX_ROOT: &'static str = "56";
        const ROOT_FILE: &'static str = "5089e241b55e7d43931615e3f2fd402797e9a2";
        let data = read_file("tests/objects/".to_string() + PREFIX_ROOT + "/" + ROOT_FILE);

        let expected_file = "100644 blob abc1234567890abcdef1234567890abcdef12  file1.txt \n040000 tree subdirHash1234567890abcdef1234567890abc123  subdir";
        assert_eq!(data, expected_file)

    }
}