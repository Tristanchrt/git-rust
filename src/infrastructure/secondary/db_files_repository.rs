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

    pub fn get_state(&self)-> TreeNodeTree {
        let paths = fs::read_dir(&self.path).unwrap();

        Self::to_tree_node(paths, self.path.clone())
    }

    fn to_tree_node(paths: ReadDir, current_file: String) -> TreeNodeTree {
        let files = paths.map(|entry| {
            Self::to_tree_node_tree(&current_file, &entry.unwrap())
        }).collect();

        let file = fs::metadata(current_file.clone()).unwrap();
        TreeNodeTree::new(file.permissions().mode().to_string(), current_file, TreeNodeType::TREE, None, files)
    }

    fn to_tree_node_tree(current: &String, path: &DirEntry) -> TreeNodeTree {
        let node_type = Self::tree_node_type(&path);
        match node_type {
            TreeNodeType::BLOB => Self::to_tree_node_file(&path, node_type, current.clone()),
            TreeNodeType::TREE => {
                let file = format!("{}/{}", current.clone(), Self::get_file_name(&path));
                Self::to_tree_node(fs::read_dir(&file).unwrap(), file)
            }
        }
    }

    fn to_tree_node_file(path: &DirEntry, node_type: TreeNodeType, root_path: String) -> TreeNodeTree {
        let file = format!("{}/{}", root_path.clone(), Self::get_file_name(&path));
        TreeNodeTree::new(Self::get_mode_file(&path), file, node_type, Some(Self::get_file_content(&path, root_path)), vec![])
    }

    fn tree_node_type(path: &DirEntry) -> TreeNodeType {
        if path.metadata().unwrap().file_type().is_file() {
            TreeNodeType::BLOB
        }else {
            TreeNodeType::TREE
        }
    }

    fn get_mode_file(path: &DirEntry) -> String {
        path.metadata().unwrap().permissions().mode().to_string()
    }

    fn get_file_name(path: &DirEntry) -> String {
        path.file_name().into_string().expect("Cannot convert file name into string")
    }

    fn get_file_content(path: &DirEntry, root_path: String) -> String {
        fs::read_to_string(format!("{}/{}", root_path, path.file_name().into_string().unwrap())).expect("Unable to read file")
    }
}

impl FilesRepository for DBFilesRepository {
    fn get_current_state(&self) -> TreeNodeTree {
        self.get_state()
    }
}