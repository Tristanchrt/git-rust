use uuid::Uuid;
use crate::domain::files_repository::FilesRepository;

pub struct DBFilesRepository {
    path: String,
}

impl DBFilesRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn save_tree_files(&self, branch: String, commit:  Uuid) {
        todo!()
    }
}

impl FilesRepository for DBFilesRepository {
    fn save(&self, branch_id: String, commit_id: Uuid) {
        self.save_tree_files(branch_id, commit_id)
    }
}