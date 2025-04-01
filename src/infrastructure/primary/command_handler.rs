use crate::applications::branches_application_service::BranchesApplicationService;
use crate::applications::commits_application_service::CommitsApplicationService;
use crate::domain::commits_repository::CommitsRepository;
use crate::infrastructure::primary::cli_branches::{CliBranch, CliBranchToCreate};
use crate::infrastructure::primary::cli_commit::{CliCommit, CliCommitToCreate, CliCommits};
use crate::infrastructure::secondary::db_branches_repository::DBBranchesRepository;
use crate::infrastructure::secondary::db_commits_repository::DBCommitsRepository;
use crate::infrastructure::secondary::db_current_branch_repository::DBCurrentBranchRepository;
use crate::infrastructure::secondary::db_tree_repository::DBTreeRepository;

pub type ArgsCLI = Vec<String>;
type CommandHandler = fn(args: ArgsCLI) -> String;

pub enum COMMAND {
    COMMIT(CommandHandler),
    BRANCH(CommandHandler),
}

impl COMMAND {
    pub(crate) fn from_str(input: &str) -> Option<COMMAND> {
        match input {
            "commit" => Self::commit_commands(),
            "branch" => Self::branch_commands(),
            _ => None,
        }
    }

    fn commit_commands() -> Option<COMMAND> {
        Some(COMMAND::COMMIT(|args| {
            // TODO find a better way
            let commit_repo: Box<dyn CommitsRepository> =
                Box::new(DBCommitsRepository::new("db/commits.txt".to_string()));
            let current_branch_repo = Box::new(DBCurrentBranchRepository::new(
                "db/current_branch.txt".to_string(),
            ));
            let tree_repository = Box::new(DBTreeRepository::new(
                "objects/".to_string()
            ));

            let service = CommitsApplicationService::new(commit_repo, current_branch_repo, tree_repository);

            if args.len() < 3 {
                return "No command provided".to_string();
            }

            match args.get(2).unwrap().as_str() {
                "-m" => {
                    if args.len() < 4 {
                        return "No message provided".to_string();
                    }

                    let cli_commit = CliCommitToCreate::new(args[3].clone());
                    let commit = service.save(cli_commit.to_domain());
                    format!(
                        "Committing changes {:?}",
                        CliCommit::from(&commit).to_display()
                    )
                }
                "-l" => {
                    if args.len() < 4 {
                        return "No branch name provided".to_string();
                    }

                    let commits = service.get_commits(args[3].clone());
                    format!(
                        "Commits listed {:?}",
                        CliCommits::from(commits).to_display()
                    )
                }
                _ => "Commit Command not found".to_string(),
            }
        }))
    }

    fn branch_commands() -> Option<COMMAND> {
        Some(COMMAND::BRANCH(|args| {
            // TODO
            let branches_repo = Box::new(DBBranchesRepository::new("db/branches.txt".to_string()));
            let current_branch_repo = Box::new(DBCurrentBranchRepository::new(
                "db/current_branch.txt".to_string(),
            ));
            let service = BranchesApplicationService::new(branches_repo, current_branch_repo);

            if args.len() < 3 {
                return "No command provided".to_string();
            }

            match args.get(2).unwrap().as_str() {
                "-c" => {
                    if args.len() < 4 {
                        return "No branch name provided".to_string();
                    }

                    let cli_branch = CliBranchToCreate::new(args[3].clone());
                    let branch = service.save(cli_branch.to_domain());
                    format!("Branch created {:?}", CliBranch::from(branch).to_display())
                }
                "-m" => {
                    if args.len() < 4 {
                        return "No branch name provided".to_string();
                    }

                    let branch_name = args[3].clone();
                    let branch = service.checkout(branch_name.clone());
                    format!(
                        "Checkout branch from {:?} to {:?}",
                        branch_name,
                        branch.name()
                    )
                }
                _ => "Branch Command not found".to_string(),
            }
        }))
    }
}
