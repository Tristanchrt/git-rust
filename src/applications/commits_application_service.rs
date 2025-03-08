use crate::domain::commit::Commit;
use crate::domain::commit_handler::CommitHandler;
use crate::domain::commits_repository::CommitsRepository;
use crate::infrastructure::primary::command_handler::ArgsCLI;

pub struct CommitsApplicationService {
    db_repository: Box<dyn CommitsRepository>,
}

impl CommitsApplicationService {

    pub fn new(db_repository: Box<dyn CommitsRepository>) -> Self {
        Self { db_repository }
    }

    pub fn save(&self, values: ArgsCLI) -> Commit {
        let commit = CommitHandler::create_commit(values);
        self.db_repository.save(&commit);
        commit
    }
}