use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct CommitToCreate {
    message: String,
}

impl CommitToCreate {
    pub fn new(message: String) -> CommitToCreate {
        CommitToCreate {
            message
        }
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl CommitToCreate {
    pub fn create(commit: CommitToCreate, parent_id: String) -> Commit {
        Commit {
            id: Uuid::new_v4(),
            parent_id,
            message: commit.message,
            created_at: chrono::Local::now().naive_local()
        }
    }
}

#[derive(Debug)]
pub struct Commit {
    id: Uuid,
    parent_id: String,
    message: String,
    created_at: NaiveDateTime
}

impl Commit {
    pub fn new(id: Uuid, parent_id: String, message: String, created_at: NaiveDateTime) -> Self {
        Self {
            id,
            parent_id,
            message,
            created_at
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn parent_id(&self) -> &String {
        &self.parent_id
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}