use crate::domain::files_repository::FilesRepository;
use crate::domain::tree::TreeNodeTree;

pub struct DBFilesRepository {
    path: String,
}

impl DBFilesRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn get_state(&self)-> TreeNodeTree {
        todo!()
    }
}

impl FilesRepository for DBFilesRepository {
    fn get_current_state(&self) -> TreeNodeTree {
        self.get_state()
    }
}