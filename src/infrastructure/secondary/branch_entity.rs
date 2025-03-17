use chrono::NaiveDateTime;
use crate::domain::branch::Branch;

pub struct BranchEntity {
    name: String,
    created_at: NaiveDateTime,
    is_current: bool
}

impl BranchEntity {
    pub fn from_string(line: &str) -> BranchEntity {
        let line: Vec<&str> = line.split(",").collect();

        BranchEntity {
            name: line.get(0).unwrap().to_string(),
            created_at: NaiveDateTime::parse_from_str(line.get(1).unwrap(), "%Y-%m-%d %H:%M:%S").unwrap(),
            is_current: BranchEntity::to_bool(line.get(2).unwrap().to_string())
        }
    }

    fn to_bool(value: String) -> bool {
        match value.to_ascii_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => false
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{},{}", self.name, self.created_at, self.is_current)
    }
}

impl PartialEq for BranchEntity {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.created_at == other.created_at && self.is_current == other.is_current
    }
}

impl BranchEntity {
    pub fn from(commit: &Branch) -> Self {
        Self {
            name: commit.name(),
            created_at: commit.created_at(),
            is_current: commit.is_current()
        }
    }

    pub fn to_domain(&self) -> Branch {
        Branch::new(self.name.clone(), self.created_at, self.is_current)
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }

    pub fn is_current(&self) -> &bool {
        &self.is_current
    }
}