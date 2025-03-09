use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::domain::commit::Commit;

pub struct CommitEntity {
    id: Uuid,
    message: String,
    parent_id: String,
    created_at: NaiveDateTime,
}

impl CommitEntity {
    pub fn to_domain(&self) -> Commit {
        Commit::new(self.id.clone(), self.parent_id.clone(), self.message.clone(), self.created_at.clone())
    }
}

impl CommitEntity {
    pub fn from(commit: &Commit) -> Self {
        Self {
            id: commit.id().clone(),
            message: commit.message().clone(),
            parent_id: commit.parent_id().clone(),
            created_at: commit.created_at().clone()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{}", self.id, self.parent_id, self.message, self.created_at)
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn parent_id(&self) -> &String {
        &self.parent_id
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}