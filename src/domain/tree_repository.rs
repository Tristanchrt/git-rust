use crate::domain::tree::TreeNodeTreeHash;

pub trait TreeRepository {
    fn save(&self, tree: &TreeNodeTreeHash);
}