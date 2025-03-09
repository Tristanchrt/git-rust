use crate::applications::commits_application_service::CommitsApplicationService;
use crate::domain::commits_repository::CommitsRepository;
use crate::infrastructure::primary::cli_commit::CliCommitToCreate;
use crate::infrastructure::secondary::db_commits_repository::DBCommitsRepository;

pub type ArgsCLI = Vec<String>;
type CommandHandler = fn(args: ArgsCLI) -> String;

pub enum COMMAND {
    COMMIT(CommandHandler),
}

impl COMMAND {
    pub(crate) fn from_str(
        input: &str,
    ) -> Option<COMMAND> {
        match input {
            "commit" => {
                Self::commit_commands()
            },
            _ => None,
        }
    }

    fn commit_commands() -> Option<COMMAND> {
        Some(COMMAND::COMMIT(|args| {
            // TODO find a better way
            let repo: Box<dyn CommitsRepository> = Box::new(DBCommitsRepository::new("db/commits.txt".to_string()));
            let service = CommitsApplicationService::new(repo);

            if args.len() < 4 {
                return "No message provided".to_string();
            }

            let cli_commit = CliCommitToCreate::new(args[3].clone());
            let commit = service.save(cli_commit.to_domain());
            // TODO add CliCommit
            return format!("Committing changes {:?}", commit)
        }))
    }
}
