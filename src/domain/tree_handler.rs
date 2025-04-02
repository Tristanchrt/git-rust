use crate::domain::files_repository::FilesRepository;
use crate::domain::tree::{TreeNodeTree, TreeNodeTreeHash};
use crate::domain::tree_repository::TreeRepository;

pub struct TreeHandler {
    tree_repository: Box<dyn TreeRepository>,
    files_repository: Box<dyn FilesRepository>
}

impl TreeHandler {
    pub fn new(
        tree_repository: Box<dyn TreeRepository>,
        files_repository: Box<dyn FilesRepository>
    ) -> Self {
        Self {
            tree_repository,
            files_repository
        }
    }

    pub fn save_hash_tree(&self) -> TreeNodeTreeHash {
        let tree_node = self.files_repository.get_current_state();
        let tree_hash = TreeNodeTree::hash_tree(tree_node);
        self.tree_repository.save(&tree_hash);
        tree_hash
    }
}