const TEST_DB_PATH: &str = "tests/objects";
mod commit_fixtures;
mod file_shared;

#[cfg(test)]
mod db_tree_repository_test {
    use git_rust::domain::tree::TreeNodeTreeHash;
    use git_rust::infrastructure::secondary::db_tree_repository::DBTreeRepository;
    use crate::file_shared::{clean_dir, read_file};
    use crate::TEST_DB_PATH;

    #[test]
    fn test_save_commit() {
        let db_repository = DBTreeRepository::new(TEST_DB_PATH.to_string());

        let content_hash_sub_dir = "100644 blob e309fe7afbb26194fbade3c9266f2cef65b7a613 subdir/file2.txt\n100644 blob cbb472bd5a547c092365cee7bec4022f09e4b8cc subdir/file3.txt".to_string();

        let content_hash_root =  "100755 tree 96764a513c0973ec5f6737a5bcdb5f95c30006f9 subdir\n100755 blob da4e04a1877a7b6a5363ed4fc407a3885010c4de file1.txt".to_string();

        let file3_hash = TreeNodeTreeHash::new("cb".to_string(), "b472bd5a547c092365cee7bec4022f09e4b8cc".to_string(),"I'm the sub file 3".to_string(), vec![]);
        let file2_hash = TreeNodeTreeHash::new("e3".to_string(), "09fe7afbb26194fbade3c9266f2cef65b7a613".to_string(), "I'm the sub file 2".to_string(), vec![]);
        let file1_hash = TreeNodeTreeHash::new("da".to_string(), "4e04a1877a7b6a5363ed4fc407a3885010c4de".to_string(), "I'm the file 1".to_string(), vec![]);
        let subdir_hash = TreeNodeTreeHash::new("96".to_string(), "764a513c0973ec5f6737a5bcdb5f95c30006f9".to_string(), content_hash_sub_dir, vec![file2_hash.clone(), file3_hash.clone()]);
        let root_hash = TreeNodeTreeHash::new("2b".to_string(), "903ff50c055a7e06a7bb5de7210eb5d691b38d".to_string(), content_hash_root, vec![subdir_hash.clone(), file1_hash.clone()]);

        db_repository.save(root_hash.clone());

        let file3_content = read_file("tests/objects/".to_string() + &file3_hash.prefix() + "/" + &file3_hash.hash());
        let file2_content = read_file("tests/objects/".to_string() + &file2_hash.prefix() + "/" + &file2_hash.hash());
        let file1_content = read_file("tests/objects/".to_string() + &file1_hash.prefix() + "/" + &file1_hash.hash());
        let root_content = read_file("tests/objects/".to_string() + &root_hash.prefix() + "/" + &root_hash.hash());
        let subdir_content = read_file("tests/objects/".to_string() + &subdir_hash.prefix() + "/" + &subdir_hash.hash());

        assert_eq!(root_content, root_hash.content());
        assert_eq!(subdir_content, subdir_hash.content());
        assert_eq!(file3_content, file3_hash.content());
        assert_eq!(file2_content, file2_hash.content());
        assert_eq!(file1_content, file1_hash.content());
        assert_eq!(file1_content, file1_hash.content());

        clean_dir(TEST_DB_PATH.to_string())
    }

    // TODO add panic test
}