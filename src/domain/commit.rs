use chrono::NaiveDateTime;
use crate::domain::command_handler::ArgsCLI;

pub struct CommitHandler {
    commit_command: fn(args: ArgsCLI) -> String,
    create_commit: String
}

struct Commit {
    id: String,
    parent_id: String,
    message: String,
    timestamp: NaiveDateTime
}

impl CommitHandler {
    fn create_commit(args: ArgsCLI) -> String {
        let value = "Init commit".to_string();
        value
    }

    pub fn commit_command(values: ArgsCLI) -> String {
        match values.get(1) {
            Some(value) if value == "-m" => CommitHandler::create_commit(values),
            _ => panic!("No valid parameters provided")
        }
    }
}