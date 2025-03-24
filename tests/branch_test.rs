mod branch_fixtures;

#[cfg(test)]
mod branch_test {
    use git_rust::domain::branch::BranchToCreate;

    #[test]
    fn test_should_create_branch_from_to_create() {
        let branch_to_create = BranchToCreate::new("toto".to_string());
        let branch = branch_to_create.create();

        assert_eq!(branch.name(), "toto");
    }
}
