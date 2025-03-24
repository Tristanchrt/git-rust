use crate::domain::branch::Branch;
use chrono::NaiveDateTime;

pub struct BranchEntity {
    name: String,
    created_at: NaiveDateTime,
}

impl BranchEntity {
    pub fn from_string(line: &str) -> BranchEntity {
        let line: Vec<&str> = line.split(",").collect();

        BranchEntity {
            name: line.get(0).unwrap().to_string(),
            created_at: NaiveDateTime::parse_from_str(line.get(1).unwrap(), "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{},{}", self.name, self.created_at)
    }
}

impl PartialEq for BranchEntity {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.created_at == other.created_at
    }
}

impl BranchEntity {
    pub fn from(commit: &Branch) -> Self {
        Self {
            name: commit.name().clone(),
            created_at: commit.created_at().clone(),
        }
    }

    pub fn to_domain(&self) -> Branch {
        Branch::new(self.name.clone(), self.created_at)
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}
