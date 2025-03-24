use crate::domain::branch::{Branch, BranchToCreate};
use chrono::NaiveDateTime;

pub struct CliBranchToCreate {
    pub name: String,
}

pub struct CliBranch {
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl CliBranch {
    pub fn from(branch: Branch) -> Self {
        Self {
            name: branch.name().clone(),
            created_at: branch.created_at(),
        }
    }

    pub fn to_display(&self) -> String {
        format!("{} | {}", self.name, self.created_at)
    }
}

impl CliBranchToCreate {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn to_domain(&self) -> BranchToCreate {
        BranchToCreate::new(self.name.clone())
    }
}
