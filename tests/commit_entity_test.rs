#[cfg(test)]
mod cli_commit_test {
    use chrono::NaiveDateTime;
    use uuid::Uuid;
    use git_rust::domain::commit::Commit;
    use git_rust::infrastructure::secondary::commit_entity::CommitEntity;

    #[test]
    fn test_should_convert_from_and_to_domain() {
        let commit =  Commit::new(
            Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a8").unwrap(),
            "0".to_string(),
            "Init commit".to_string(),
            chrono::Local::now().naive_local()
        );
        let commit_entity = CommitEntity::from(&commit);
        let commit_from_entity = commit_entity.to_domain();

        assert_eq!(commit_from_entity.id().to_string(), "936da01f-9abd-4d9d-80c7-02af85c822a8");
        assert_eq!(commit_from_entity.message(), "Init commit");
        assert_eq!(commit_from_entity.parent_id(), "0");
    }

    #[test]
    fn test_should_convert_to_string() {
        let commit = Commit::new(
            Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a8").unwrap(),
            "0".to_string(),
            "Init commit".to_string(),
            NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        let commit_entity = CommitEntity::from(&commit);

        assert_eq!(commit_entity.to_string(), "936da01f-9abd-4d9d-80c7-02af85c822a8,0,Init commit,2023-01-01 12:00:00");
    }
}