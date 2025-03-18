use chrono::NaiveDateTime;
use uuid::Uuid;
use git_rust::domain::commit::{Commit, CommitToCreate};

pub fn sample_commit() -> Commit {
    Commit::new(
        Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a8").unwrap(),
        Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap(),
        "Init commit".to_string(),
        NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
    )
}

pub fn sample_default_commit() -> Commit {
    Commit::new(
        Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a8").unwrap(),
        Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap(),
        "Init commit".to_string(),
        NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
    )
}

pub fn commit_to_create() -> CommitToCreate {
    CommitToCreate::new("Init commit".to_string())
}
