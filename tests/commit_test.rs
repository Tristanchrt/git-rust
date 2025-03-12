use git_rust::domain::commit_handler::CommitHandler;
mod commit_fixtures;

#[cfg(test)]
mod commit_test {
    use uuid::Uuid;
    use git_rust::domain::commit::CommitToCreate;
    use crate::commit_fixtures::sample_commit;
    use super::*;

    #[test]
    fn test_commit_to_create_create() {
        let commit = CommitToCreate::create(CommitToCreate::new("Init commit".to_string()), Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap());

        assert_eq!(commit.message(), "Init commit");
        assert_eq!(commit.parent_id().to_string(), Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap().to_string());
    }

    #[test]
    fn test_create_commit() {
        let result = CommitHandler::create_commit(CommitToCreate::new("Init commit".to_string()), Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap());
        let commit = sample_commit();

        assert_eq!(result.message(), commit.message());
        assert_eq!(result.parent_id(), commit.parent_id());
    }
}
