mod tree_fixtures;

#[cfg(test)]
mod cli_branch_test {
    use git_rust::domain::tree::{TreeNodeTree, TreeNodeType};
    use crate::tree_fixtures::{file1_hash, file2_hash, file3_hash, root_hash, subdir_hash};

    #[test]
    fn test_should_transform_to_hash() {

        let file3 = TreeNodeTree::new("100644".to_string(), "subdir/file3.txt".to_string(), TreeNodeType::BLOB, Some("I'm the sub file 3".to_string()), vec![]);
        let file2 = TreeNodeTree::new("100644".to_string(), "subdir/file2.txt".to_string(), TreeNodeType::BLOB, Some("I'm the sub file 2".to_string()), vec![]);
        let file1 = TreeNodeTree::new("100755".to_string(), "file1.txt".to_string(), TreeNodeType::BLOB, Some("I'm the file 1".to_string()), vec![]);

        let sub_dir = TreeNodeTree::new("100755".to_string(), "subdir".to_string(), TreeNodeType::TREE, None, vec![file2, file3]);

        let root = TreeNodeTree::new("100644".to_string(), "git-rust-data".to_string(), TreeNodeType::TREE, None, vec![sub_dir, file1]);

        assert_eq!(TreeNodeTree::hash_tree(root), root_hash());
    }

    #[test]
    fn test_should_transform_hash_tree_to_hashes() {
        let hashes = vec![
            root_hash().to_flat(),
            subdir_hash().to_flat(),
            file2_hash().to_flat(),
            file3_hash().to_flat(),
            file1_hash().to_flat(),
        ];
        assert_eq!(root_hash().flatten_nodes(), hashes);
    }
}
