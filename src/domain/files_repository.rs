use crate::domain::tree::TreeNodeTree;

pub trait FilesRepository {
    fn get_current_state(&self) -> TreeNodeTree;
    fn restore_tree(&self, tree: TreeNodeTree);
}