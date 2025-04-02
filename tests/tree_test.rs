mod tree_fixtures;

#[cfg(test)]
mod cli_branch_test {
    use git_rust::domain::tree::{TreeNodeTree};
    use crate::tree_fixtures::{file1_hash, file2_hash, file3_hash, root_hash, subdir_hash, tree_root};

    #[test]
    fn test_should_transform_to_hash() {
        assert_eq!(TreeNodeTree::hash_tree(tree_root()), root_hash());
    }

    #[test]
    fn test_should_transform_hash_tree_to_hashes() {
        let hashes = vec![
            root_hash().to_flat(),
            file1_hash().to_flat(),
            subdir_hash().to_flat(),
            file2_hash().to_flat(),
            file3_hash().to_flat(),
        ];
        assert_eq!(root_hash().flatten_nodes(), hashes);
    }
}
