use git_rust::domain::commit_handler::CommitHandler;

#[cfg(test)]
mod commit_test {
    use git_rust::domain::commit::Commit;
    use super::*;

    #[test]
    fn test_create_commit() {
        let args = vec!["file".to_string(), "-m".to_string(), "Init commit".to_string()];
        let result = CommitHandler::create_commit(args);
        let commit =  Commit::new(
            "1".to_string(),
            "0".to_string(),
            "Init commit".to_string(),
            chrono::Local::now().naive_local()
        );
        assert_eq!(result.id(), commit.id());
        assert_eq!(result.message(), commit.message());
        assert_eq!(result.parent_id(), commit.parent_id());
    }
}
