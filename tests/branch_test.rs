mod branch_fixtures;

#[cfg(test)]
mod branch_test {
    use git_rust::domain::branch::BranchToCreate;
    use git_rust::domain::branch_handler::BranchHandler;
    use crate::branch_fixtures::{sample_branch, sample_branch_to_create};

    #[test]
    fn test_should_create_branch_from_to_create() {
        let branch_to_create = BranchToCreate::new("toto".to_string());
        let branch = branch_to_create.create();

        assert_eq!(branch.name(), "toto");
    }

    #[test]
    fn test_should_create_branch_from_to_create_with_parent() {
        let branch_to_create = BranchHandler::create_branch(sample_branch_to_create());

        let branch = sample_branch();
        assert_eq!(branch.name(), "toto");
    }
}