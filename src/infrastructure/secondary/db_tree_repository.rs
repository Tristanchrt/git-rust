use fs::read_to_string;
use std::fs;
use std::fs::{create_dir_all, File};
use crate::domain::tree::{TreeNodeTreeHash};
use crate::domain::tree_repository::TreeRepository;
use std::io::Write;
use std::path::{Path};

pub struct DBTreeRepository {
    path: String,
}

impl DBTreeRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn save(&self, tree: &TreeNodeTreeHash) {
        for node in tree.flatten_nodes() {

            let root = format!("{}/{}/", self.path, node.prefix());
            let path = Path::new(&root);
            create_dir_all(path).unwrap();

            let file_to_save = format!("{}/{}", root, node.hash());

            let mut file = File::create(&file_to_save)
                .unwrap_or_else(|error| panic!("Cannot create file {}, {}", file_to_save, error));
            file.write_all(node.content().as_bytes())
                .unwrap_or_else(|error| panic!("Cannot write file {}, {}", file_to_save, error))
        }
    }

    fn get_tree_node_hash(&self, tree_hash: &String) -> TreeNodeTreeHash {
        let (prefix, hash) = Self::split_hash(tree_hash);
        let content = Self::get_content(&self.path, &prefix, &hash);
        Self::to_tree_node_hash(content, self.path.to_string(), prefix.to_string(), hash.to_string())
    }

    fn to_tree_node_hash(content: String, root_path: String, prefix: String, hash: String) -> TreeNodeTreeHash {
        let nodes = content.split("\n").map(|entry| {
            Self::create_tree_node_hash(entry, root_path.clone())
        }).collect();

        TreeNodeTreeHash::new(prefix, hash, content, nodes)
    }

    fn create_tree_node_hash(row: &str, root_path: String) -> TreeNodeTreeHash {
        let (_, type_node, hash, _) = Self::row_to_data(row);
        let hash_str: String = hash.to_string();
        let (prefix, hash_not_prefix) = Self::split_hash(&hash_str);
        let content = Self::get_content(&root_path, prefix, hash_not_prefix);
        match type_node {
            "blob" => TreeNodeTreeHash::new(prefix.to_string(), hash_not_prefix.to_string(), content ,vec![]),
            "tree" => Self::to_tree_node_hash(content, root_path.to_string(), prefix.to_string(), hash_not_prefix.to_string()),
            _ => panic!("Type file not found")
        }
    }

    fn get_content(root_path: &str, prefix: &str, hash: &str) -> String {
        read_to_string(format!("{}/{}/{}", &root_path, &prefix, &hash)).expect("Unable to read file")
    }

    fn split_hash(tree_hash: &String) -> (&str, &str) {
        let (prefix, hash) = tree_hash.split_at(2);
        (prefix, hash)
    }

    fn row_to_data(row: &str) -> (&str, &str, &str, &str) {
        let col = row.split(" ").collect::<Vec<&str>>();
        (col.get(0).unwrap(), col.get(1).unwrap(), col.get(2).unwrap(), col.get(3).unwrap())
    }
}

impl TreeRepository for DBTreeRepository {
    fn save(&self, tree: &TreeNodeTreeHash) {
        self.save(tree)
    }

    fn get_tree_node_hash(&self, tree_hash: &String) -> TreeNodeTreeHash {
        self.get_tree_node_hash(tree_hash)
    }
}