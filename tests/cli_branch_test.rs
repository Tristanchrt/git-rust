mod branch_fixtures;

#[cfg(test)]
mod cli_branch_test {
    use git_rust::infrastructure::primary::cli_branches::{CliBranch, CliBranchToCreate};
    use crate::branch_fixtures::sample_branch;

    #[test]
    fn test_should_transform_to_domain() {
        let cli_commit = CliBranchToCreate::new("toto".to_string());
        let commit_to_create = cli_commit.to_domain();
        assert_eq!(commit_to_create.name(), "toto");
    }

    #[test]
    fn test_should_display_cli_commit() {
        let branch = sample_branch();
        let cli_branch = CliBranch::from(branch);
        assert_eq!(cli_branch.to_display(), "toto | 2023-01-01 12:00:00");
    }
}