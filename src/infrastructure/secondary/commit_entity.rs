use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::domain::commit::Commit;

pub struct CommitEntity {
    id: Uuid,
    message: String,
    parent_id: Uuid,
    created_at: NaiveDateTime,
}

impl CommitEntity {
    pub fn from_string(line: &str) -> CommitEntity {
        CommitEntity {
            id: Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a8").unwrap(),
            message: "Init commit".to_string(),
            parent_id: Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap(),
            created_at: NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        }
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

    pub fn to_domain(&self) -> Commit {
        Commit::new(self.id.clone(), self.parent_id.clone(), self.message.clone(), self.created_at.clone())
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

    pub fn parent_id(&self) -> &Uuid {
        &self.parent_id
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}