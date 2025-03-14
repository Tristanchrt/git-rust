use chrono::NaiveDateTime;
use git_rust::domain::branch::Branch;

pub fn sample_branch() -> Branch {
    Branch::new("toto".to_string(), NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap())
}