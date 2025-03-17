use chrono::NaiveDateTime;
use git_rust::domain::branch::{Branch, BranchToCreate};

pub fn sample_branch() -> Branch {
    Branch::new("toto".to_string(), NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap())
}

pub fn sample_branch_two() -> Branch {
    Branch::new("tata".to_string(), NaiveDateTime::parse_from_str("2021-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap())
}

pub fn sample_branch_to_create() -> BranchToCreate {
    BranchToCreate::new("toto".to_string())
}