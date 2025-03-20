use chrono::NaiveDateTime;
use uuid::Uuid;
use crate::domain::commit::Commit;

pub struct CommitEntity {
    id: Uuid,
    parent_id: Uuid,
    message: String,
    created_at: NaiveDateTime,
    branch_id: String
}

impl CommitEntity {
    pub fn from_string(line: &str) -> CommitEntity {
        let line: Vec<&str> = line.split(",").collect();

        CommitEntity {
            id: Uuid::parse_str(line.get(0).unwrap()).unwrap(),
            parent_id: Uuid::parse_str(line.get(1).unwrap()).unwrap(),
            message: line.get(2).unwrap().to_string(),
            created_at: NaiveDateTime::parse_from_str(line.get(3).unwrap(), "%Y-%m-%d %H:%M:%S").unwrap(),
            branch_id: line.get(4).unwrap().to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{},{},{}", self.id, self.parent_id, self.message, self.created_at, self.branch_id)
    }
}

impl PartialEq for CommitEntity {
    fn eq(&self, other: &Self) -> bool {
        self.id.to_string() == other.id.to_string()
            && self.parent_id.to_string() == other.parent_id.to_string()
            && self.created_at == other.created_at
            && self.message == other.message
            && self.branch_id == other.branch_id
    }
}

impl CommitEntity {
    pub fn from(commit: &Commit) -> Self {
        Self {
            id: commit.id().clone(),
            parent_id: commit.parent_id().clone(),
            message: commit.message().clone(),
            created_at: commit.created_at().clone(),
            branch_id: commit.branch_id().clone()
        }
    }

    pub fn to_domain(&self) -> Commit {
        Commit::new(self.id.clone(), self.parent_id.clone(), self.message.clone(), self.created_at.clone(), self.branch_id.clone())
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

    pub fn branch_id(&self) -> &String {
        &self.branch_id
    }
}