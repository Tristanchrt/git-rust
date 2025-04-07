use crate::domain::tree::TreeNodeTreeHash;

pub trait TreeRepository {
    fn save(&self, tree: &TreeNodeTreeHash);
    fn get_tree_node_hash(&self, tree_hash: &String) -> TreeNodeTreeHash;
}