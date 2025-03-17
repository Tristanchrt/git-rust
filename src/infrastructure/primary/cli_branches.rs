use chrono::NaiveDateTime;
use crate::domain::branch::{Branch, BranchToCreate};

pub struct CliBranchToCreate {
    pub name: String,
    pub is_current: bool
}

pub struct CliBranch {
    pub name: String,
    pub created_at: NaiveDateTime,
    pub is_current: bool
}

impl CliBranch {
    pub fn from(branch: Branch) -> Self {
        Self {
            name: branch.name(),
            created_at: branch.created_at(),
            is_current: branch.is_current()
        }
    }

    pub fn to_display(&self) -> String {
        format!("{} | {} | {}", self.name, self.created_at, self.is_current)
    }
}

impl CliBranchToCreate {
    pub fn new(name: String, is_current: bool) -> Self {
        Self {
            name,
            is_current
        }
    }

    pub fn to_domain(&self) -> BranchToCreate {
        BranchToCreate::new(self.name.clone(), self.is_current)
    }
}