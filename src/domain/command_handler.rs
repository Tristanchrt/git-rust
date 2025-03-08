use crate::domain::commit::CommitHandler;

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
                Some(COMMAND::COMMIT(|args| {
                    CommitHandler::commit_command(args)
                }))
            },
            _ => None,
        }
    }
}
