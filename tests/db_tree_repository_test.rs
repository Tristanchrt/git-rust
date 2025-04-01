const TEST_DB_PATH: &str = "tests/objects";
mod commit_fixtures;
mod file_shared;
mod tree_fixtures;

#[cfg(test)]
mod db_tree_repository_test {
    use git_rust::infrastructure::secondary::db_tree_repository::DBTreeRepository;
    use crate::file_shared::{clean_dir, read_file};
    use crate::TEST_DB_PATH;
    use crate::tree_fixtures::{file1_hash, file2_hash, file3_hash, root_hash, subdir_hash};

    #[test]
    fn test_save_commit() {
        let db_repository = DBTreeRepository::new(TEST_DB_PATH.to_string());

        db_repository.save(root_hash().clone());

        let file3_content = read_file("tests/objects/".to_string() + &file3_hash().prefix() + "/" + &file3_hash().hash());
        let file2_content = read_file("tests/objects/".to_string() + &file2_hash().prefix() + "/" + &file2_hash().hash());
        let file1_content = read_file("tests/objects/".to_string() + &file1_hash().prefix() + "/" + &file1_hash().hash());
        let root_content = read_file("tests/objects/".to_string() + &root_hash().prefix() + "/" + &root_hash().hash());
        let subdir_content = read_file("tests/objects/".to_string() + &subdir_hash().prefix() + "/" + &subdir_hash().hash());

        assert_eq!(root_content, root_hash().content());
        assert_eq!(subdir_content, subdir_hash().content());
        assert_eq!(file3_content, file3_hash().content());
        assert_eq!(file2_content, file2_hash().content());
        assert_eq!(file1_content, file1_hash().content());
        assert_eq!(file1_content, file1_hash().content());

        clean_dir(TEST_DB_PATH.to_string())
    }
}