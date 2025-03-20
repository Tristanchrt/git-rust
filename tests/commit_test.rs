mod commit_fixtures;

#[cfg(test)]
mod commit_test {
    use uuid::Uuid;
    use crate::commit_fixtures::{commit_to_create};

    #[test]
    fn test_commit_to_create_create() {
        let commit_to_create = commit_to_create();
        let commit = commit_to_create.create(Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap(),"toto".to_string());

        assert_eq!(commit.message(), "Init commit");
        assert_eq!(commit.parent_id().to_string(), Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap().to_string());
        assert_eq!(commit.branch_id().clone(), "toto".to_string());
    }
}
