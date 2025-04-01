use crate::domain::tree_repository::TreeRepository;

pub struct TreeHandler {
    tree_repository: Box<dyn TreeRepository>
}

impl TreeHandler {
    pub fn new(
        tree_repository: Box<dyn TreeRepository>
    ) -> Self {
        Self {
            tree_repository
        }
    }
}