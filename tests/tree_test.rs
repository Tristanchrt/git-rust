#[cfg(test)]
mod cli_branch_test {
    use git_rust::domain::tree::{TreeNodeTree, TreeNodeTreeHash, TreeNodeType};

    #[test]
    fn test_should_transform_to_hash() {

        let file3 = TreeNodeTree::new("100644".to_string(), "subdir/file3.txt".to_string(), TreeNodeType::BLOB, Some("I'm the sub file 3".to_string()), vec![]);
        let file2 = TreeNodeTree::new("100644".to_string(), "subdir/file2.txt".to_string(), TreeNodeType::BLOB, Some("I'm the sub file 2".to_string()), vec![]);
        let file1 = TreeNodeTree::new("100755".to_string(), "file1.txt".to_string(), TreeNodeType::BLOB, Some("I'm the file 1".to_string()), vec![]);

        let sub_dir = TreeNodeTree::new("100755".to_string(), "subdir".to_string(), TreeNodeType::TREE, None, vec![file2, file3]);

        let root = TreeNodeTree::new("100644".to_string(), "git-rust-data".to_string(), TreeNodeType::TREE, None, vec![sub_dir, file1]);

        let content_hash_sub_dir = "100644 blob e309fe7afbb26194fbade3c9266f2cef65b7a613 subdir/file2.txt\n100644 blob cbb472bd5a547c092365cee7bec4022f09e4b8cc subdir/file3.txt".to_string();

        let content_hash_root =  "100755 tree 96764a513c0973ec5f6737a5bcdb5f95c30006f9 subdir\n100755 blob da4e04a1877a7b6a5363ed4fc407a3885010c4de file1.txt".to_string();

        let file3_hash = TreeNodeTreeHash::new("cb".to_string(), "b472bd5a547c092365cee7bec4022f09e4b8cc".to_string(),"I'm the sub file 3".to_string(), vec![]);
        let file2_hash = TreeNodeTreeHash::new("e3".to_string(), "09fe7afbb26194fbade3c9266f2cef65b7a613".to_string(), "I'm the sub file 2".to_string(), vec![]);
        let file1_hash = TreeNodeTreeHash::new("da".to_string(), "4e04a1877a7b6a5363ed4fc407a3885010c4de".to_string(), "I'm the file 1".to_string(), vec![]);
        let subdir_hash = TreeNodeTreeHash::new("96".to_string(), "764a513c0973ec5f6737a5bcdb5f95c30006f9".to_string(), content_hash_sub_dir, vec![file2_hash, file3_hash]);
        let root_hash = TreeNodeTreeHash::new("2b".to_string(), "903ff50c055a7e06a7bb5de7210eb5d691b38d".to_string(), content_hash_root, vec![subdir_hash, file1_hash]);

        assert_eq!(TreeNodeTree::hash_tree(root), root_hash);
    }

    // #[test]
    // fn test_should_transform_hash_tree_to_hashs() {
    //
    //     let content_hash_sub_dir = "100644 blob e309fe7afbb26194fbade3c9266f2cef65b7a613 subdir/file2.txt\n100644 blob cbb472bd5a547c092365cee7bec4022f09e4b8cc subdir/file3.txt".to_string();
    //
    //     let content_hash_root =  "100755 tree 96764a513c0973ec5f6737a5bcdb5f95c30006f9 subdir\n100755 blob da4e04a1877a7b6a5363ed4fc407a3885010c4de file1.txt".to_string();
    //
    //     let file3_hash = TreeNodeTreeHash::new("cb".to_string(), "b472bd5a547c092365cee7bec4022f09e4b8cc".to_string(),"I'm the sub file 3".to_string(), vec![]);
    //     let file2_hash = TreeNodeTreeHash::new("e3".to_string(), "09fe7afbb26194fbade3c9266f2cef65b7a613".to_string(), "I'm the sub file 2".to_string(), vec![]);
    //     let file1_hash = TreeNodeTreeHash::new("da".to_string(), "4e04a1877a7b6a5363ed4fc407a3885010c4de".to_string(), "I'm the file 1".to_string(), vec![]);
    //     let subdir_hash = TreeNodeTreeHash::new("96".to_string(), "764a513c0973ec5f6737a5bcdb5f95c30006f9".to_string(), content_hash_sub_dir, vec![file2_hash, file3_hash]);
    //     let root_hash = TreeNodeTreeHash::new("2b".to_string(), "903ff50c055a7e06a7bb5de7210eb5d691b38d".to_string(), content_hash_root, vec![subdir_hash, file1_hash]);
    //
    //     // TODO
    //     assert_eq!(root_hash.to_hashes(), hashes);
    // }
}
