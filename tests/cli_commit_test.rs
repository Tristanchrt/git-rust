#[cfg(test)]
mod cli_commit_test {
    use git_rust::infrastructure::primary::cli_commit::CliCommitToCreate;

    #[test]
    fn test_should_transform_to_domain() {
        let cli_commit = CliCommitToCreate::new("Init commit".to_string());
        let commit_to_create = cli_commit.to_domain();
        assert_eq!(commit_to_create.message(), "Init commit");
    }
}