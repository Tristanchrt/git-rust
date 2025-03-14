mod branch_fixtures;

#[cfg(test)]
mod cli_branch_test {
    use git_rust::infrastructure::secondary::branch_entity::BranchEntity;
    use crate::branch_fixtures::sample_branch;

    #[test]
    fn test_should_convert_from_and_to_domain() {
        let branch = sample_branch();
        let branch_entity = BranchEntity::from(&branch);
        let branch_from_entity = branch_entity.to_domain();

        assert_eq!(branch_from_entity.name().to_string(), "toto");
        assert_eq!(branch_from_entity.created_at().to_string(), "2023-01-01 12:00:00");
    }

    #[test]
    fn test_should_convert_to_string() {
        let branch = sample_branch();
        let branch_entity = BranchEntity::from(&branch);

        assert_eq!(branch_entity.to_string(), "toto,2023-01-01 12:00:00");
    }

    #[test]
    fn test_should_convert_from_string() {
        let line = "toto,2023-01-01 12:00:00";

        let branch_entity = BranchEntity::from_string(line);

        assert_eq!(branch_entity.name().to_string(), "toto");
        assert_eq!(branch_entity.created_at().to_string(), "2023-01-01 12:00:00");
    }
}