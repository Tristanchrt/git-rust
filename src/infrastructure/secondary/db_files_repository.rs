use fs::read_to_string;
use std::fs;
use std::fs::{DirEntry, ReadDir};
use std::os::unix::fs::PermissionsExt;
use crate::domain::files_repository::FilesRepository;
use crate::domain::tree::{TreeNodeTree, TreeNodeType};

pub struct DBFilesRepository {
    path: String,
}

impl DBFilesRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn restore_tree(&self, tree: TreeNodeTree)  {
        todo!()
    }

    pub fn get_state(&self)-> TreeNodeTree {
        let paths = fs::read_dir(&self.path).unwrap();

        Self::to_tree_node(paths, &self.path)
    }

    fn to_tree_node(paths: ReadDir, current_file: &String) -> TreeNodeTree {
        let files = paths.map(|entry| {
            Self::create_tree_node(&entry.unwrap(), current_file)
        }).collect();

        TreeNodeTree::new(Self::get_file_permissions(current_file), current_file.to_string(), TreeNodeType::TREE, None, files)
    }

    fn create_tree_node(path: &DirEntry, current: &String) -> TreeNodeTree {
        let file_name = format!("{}/{}", current, Self::get_file_name(path));

        let node_type = Self::tree_node_type(path);
        match node_type {
            TreeNodeType::BLOB => Self::to_tree_node_file(&file_name),
            TreeNodeType::TREE => {
                Self::to_tree_node(fs::read_dir(&file_name).unwrap(), &file_name)
            }
        }
    }

    fn to_tree_node_file(file: &String) -> TreeNodeTree {
        TreeNodeTree::new(Self::get_file_permissions(file), file.to_string(), TreeNodeType::BLOB, Some(Self::get_file_content(file)), vec![])
    }

    fn get_file_permissions(file: &String) -> String {
        fs::metadata(file).unwrap().permissions().mode().to_string()
    }

    fn tree_node_type(path: &DirEntry) -> TreeNodeType {
        if path.metadata().unwrap().file_type().is_file() {
            TreeNodeType::BLOB
        }else {
            TreeNodeType::TREE
        }
    }

    fn get_file_name(path: &DirEntry) -> String {
        path.file_name().into_string().expect("Cannot convert file name into string")
    }

    fn get_file_content(file: &String) -> String {
        read_to_string(file).expect("Unable to read file")
    }
}

impl FilesRepository for DBFilesRepository {
    fn get_current_state(&self) -> TreeNodeTree {
        self.get_state()
    }

    fn restore_tree(&self, tree: TreeNodeTree) {
        self.restore_tree(tree)
    }
}