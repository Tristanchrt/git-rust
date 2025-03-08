use git_rust::domain::commit::CommitHandler;

#[cfg(test)]
mod commit_test {
    use super::*;

    #[test]
    #[should_panic(expected = "No valid parameters provided")]
    fn test_command_not_found_panic() {
        let args = vec!["-z".to_string(), "Init commit".to_string()];
        let result = CommitHandler::commit_command(args);
    }

    #[test]
    fn test_create_commit() {
        let args = vec!["file".to_string(), "-m".to_string(), "Init commit".to_string()];
        let result = CommitHandler::commit_command(args);
        assert_eq!(result, "Init commit".to_string());
    }
}
