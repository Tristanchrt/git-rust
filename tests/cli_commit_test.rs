mod commit_fixtures;

#[cfg(test)]
mod cli_commit_test {
    use git_rust::infrastructure::primary::cli_commit::{CliCommit, CliCommitToCreate};
    use crate::commit_fixtures::sample_commit;

    #[test]
    fn test_should_transform_to_domain() {
        let cli_commit = CliCommitToCreate::new("Init commit".to_string());
        let commit_to_create = cli_commit.to_domain();
        assert_eq!(commit_to_create.message(), "Init commit");
    }

    #[test]
    fn test_should_display_cli_commit() {
        let commit = sample_commit();
        let cli_commit = CliCommit::from(commit);
        assert_eq!(cli_commit.to_display(), "936da01f-9abd-4d9d-80c7-02af85c822a8 | 936da01f-9abd-4d9d-80c7-02af85c822a7 | 2023-01-01 12:00:00 | Init commit");
    }
}