use std::fs::{File};
use crate::domain::tree::TreeNodeTreeHash;
use crate::domain::tree_repository::TreeRepository;
use std::io::Write;

pub struct DBTreeRepository {
    path: String,
}

impl DBTreeRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn save(&self, tree: TreeNodeTreeHash) {
        for node in tree.flatten_nodes() {

            let root = format!("{}/{}/", self.path, node.prefix());
            let path = std::path::Path::new(&root);
            std::fs::create_dir_all(path).unwrap();

            let file_to_save = format!("{}/{}", root, node.hash());

            let mut file = File::create(&file_to_save)
                .unwrap_or_else(|error| panic!("Cannot create file {}, {}", file_to_save, error));
            file.write_all(node.content().as_bytes())
                .unwrap_or_else(|error| panic!("Cannot write file {}, {}", file_to_save, error))
        }
    }
}

impl TreeRepository for DBTreeRepository {
    fn save(&self, tree: TreeNodeTreeHash) {
        self.save(tree)
    }
}