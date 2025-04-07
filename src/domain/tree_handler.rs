use crate::domain::commit::Commit;
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

    pub fn restore_commit(&self, commit: Commit) {
        let tree_node_hash = self.tree_repository.get_tree_node_hash(commit.tree_hash());
        self.files_repository.restore_tree(tree_node_hash.to_tree_node())
    }
}