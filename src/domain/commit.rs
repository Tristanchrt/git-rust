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
            timestamp: chrono::Local::now().naive_local()
        }
    }
}

#[derive(Debug)]
pub struct Commit {
    id: Uuid,
    parent_id: String,
    message: String,
    timestamp: NaiveDateTime
}

impl Commit {
    pub fn new(id: Uuid, parent_id: String, message: String, timestamp: NaiveDateTime) -> Self {
        Self {
            id,
            parent_id,
            message,
            timestamp
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.id, self.parent_id, self.message)
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
}