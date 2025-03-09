use crate::domain::commit::CommitToCreate;

pub struct CliCommitToCreate {
    pub message: String,
}

impl CliCommitToCreate {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }

    pub fn to_domain(&self) -> CommitToCreate {
        CommitToCreate::new(self.message.clone())
    }
}