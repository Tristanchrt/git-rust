use sha2::{Sha256};

enum TreeNodeType {
    BLOB,
    TREE
}

struct TreeNode {
    id: Sha256,
    left: TreeNode,
    right: TreeNode,
    type_node: TreeNodeType,
    mode: String,
    filename: String
}

pub struct Tree {
    node: TreeNode
}