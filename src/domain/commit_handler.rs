use crate::domain::commit::Commit;
use crate::infrastructure::primary::command_handler::ArgsCLI;

pub struct CommitHandler {
    create_commit: String
}

impl CommitHandler {
    pub fn create_commit(args: ArgsCLI) -> Commit {
        Commit::new(
            "1".to_string(),
            "0".to_string(),
            "Init commit".to_string(),
            chrono::Local::now().naive_local()
        )
    }
}