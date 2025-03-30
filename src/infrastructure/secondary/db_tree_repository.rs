use std::fs::{File, OpenOptions};
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
            let mut file = File::create(&file_to_save).expect("Cannot create file");
            file.write_all(node.content().as_bytes()).expect("Cannot write file");
        }
    }
}

impl TreeRepository for DBTreeRepository {
    fn save(&self, tree: TreeNodeTreeHash) {
        self.save(tree)
    }
}