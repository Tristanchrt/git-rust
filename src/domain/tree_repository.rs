use crate::domain::tree::{TreeNodeTree, TreeNodeTreeHash};

pub trait TreeRepository {
    fn save(&self, tree: TreeNodeTreeHash);
}