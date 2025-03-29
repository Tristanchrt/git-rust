use uuid::Uuid;

pub trait FilesRepository {
    fn save(&self, branch_id: String, commit_id: Uuid);
}