#[cfg(test)]
mod cli_commit_test {
    use chrono::NaiveDateTime;
    use uuid::Uuid;
    use git_rust::domain::commit::Commit;
    use git_rust::infrastructure::primary::cli_commit::{CliCommit, CliCommitToCreate};

    #[test]
    fn test_should_transform_to_domain() {
        let cli_commit = CliCommitToCreate::new("Init commit".to_string());
        let commit_to_create = cli_commit.to_domain();
        assert_eq!(commit_to_create.message(), "Init commit");
    }

    #[test]
    fn test_should_transform_() {
        let commit = Commit::new(
            Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a8").unwrap(),
            Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap(),
            "Init commit".to_string(),
            NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        let cli_commit = CliCommit::from(commit);
        assert_eq!(cli_commit.to_display(), "936da01f-9abd-4d9d-80c7-02af85c822a8 | 936da01f-9abd-4d9d-80c7-02af85c822a7 | 2023-01-01 12:00:00 | Init commit");
    }
}