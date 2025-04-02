use git_rust::domain::tree::{TreeNodeTree, TreeNodeTreeHash, TreeNodeType};
use git_rust::domain::tree::TreeNodeType::TREE;

pub fn root_hash() -> TreeNodeTreeHash {
    TreeNodeTreeHash::new("b1".to_string(), "02c5760bdb348c75f0bf944d18c389b577f9c3".to_string(), content_hash_root(), vec![file1_hash(), subdir_hash()])
}

pub fn file1_hash()  -> TreeNodeTreeHash {
    TreeNodeTreeHash::new("da".to_string(), "4e04a1877a7b6a5363ed4fc407a3885010c4de".to_string(), "I'm the file 1".to_string(), vec![])
}

pub fn subdir_hash() -> TreeNodeTreeHash  {
    TreeNodeTreeHash::new("58".to_string(), "4aac37881e5c2c954e42f3470c256aaad7dde4".to_string(), content_hash_sub_dir(), vec![file2_hash(), file3_hash()])
}

pub fn file2_hash() -> TreeNodeTreeHash {
    TreeNodeTreeHash::new("e3".to_string(), "09fe7afbb26194fbade3c9266f2cef65b7a613".to_string(), "I'm the sub file 2".to_string(), vec![])
}

pub fn file3_hash() -> TreeNodeTreeHash {
    TreeNodeTreeHash::new("cb".to_string(), "b472bd5a547c092365cee7bec4022f09e4b8cc".to_string(),"I'm the sub file 3".to_string(), vec![])
}

pub fn content_hash_sub_dir() -> String {
    "33188 blob e309fe7afbb26194fbade3c9266f2cef65b7a613 tests/git-rust-data/subdir/file2.txt\n33188 blob cbb472bd5a547c092365cee7bec4022f09e4b8cc tests/git-rust-data/subdir/file3.txt".to_string()
}

pub fn content_hash_root() -> String {
    "33188 blob da4e04a1877a7b6a5363ed4fc407a3885010c4de tests/git-rust-data/file1.txt\n16877 tree 584aac37881e5c2c954e42f3470c256aaad7dde4 tests/git-rust-data/subdir".to_string()
}

pub fn file3() -> TreeNodeTree {
    TreeNodeTree::new("33188".to_string(), "tests/git-rust-data/subdir/file3.txt".to_string(), TreeNodeType::BLOB, Some("I'm the sub file 3".to_string()), vec![])
}

pub fn file2() -> TreeNodeTree {
    TreeNodeTree::new("33188".to_string(), "tests/git-rust-data/subdir/file2.txt".to_string(), TreeNodeType::BLOB, Some("I'm the sub file 2".to_string()), vec![])
}

pub fn file1() -> TreeNodeTree {
    TreeNodeTree::new("33188".to_string(), "tests/git-rust-data/file1.txt".to_string(), TreeNodeType::BLOB, Some("I'm the file 1".to_string()), vec![])
}

pub fn sub_dir() -> TreeNodeTree {
    TreeNodeTree::new("16877".to_string(), "tests/git-rust-data/subdir".to_string(), TREE, None, vec![file2(), file3()])
}

pub fn tree_root() -> TreeNodeTree  {
    TreeNodeTree::new("16877".to_string(), "tests/git-rust-data".to_string(), TREE, None, vec![file1(), sub_dir()])
}