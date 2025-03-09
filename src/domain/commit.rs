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

    pub fn default_parent_id() -> Uuid {
        Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
    }
}

impl CommitToCreate {
    pub fn create(commit: CommitToCreate, parent_id: Uuid) -> Commit {
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
    parent_id: Uuid,
    message: String,
    created_at: NaiveDateTime
}

impl Commit {
    pub fn new(id: Uuid, parent_id: Uuid, message: String, created_at: NaiveDateTime) -> Self {
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

    pub fn parent_id(&self) -> &Uuid {
        &self.parent_id
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}