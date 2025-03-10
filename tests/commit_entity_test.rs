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
            Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap(),
            "Init commit".to_string(),
            NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        let commit_entity = CommitEntity::from(&commit);
        let commit_from_entity = commit_entity.to_domain();

        assert_eq!(commit_from_entity.id().to_string(), "936da01f-9abd-4d9d-80c7-02af85c822a8");
        assert_eq!(commit_from_entity.parent_id().to_string(), Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap().to_string());
        assert_eq!(commit_from_entity.message(), "Init commit");
        assert_eq!(commit_from_entity.created_at().to_string(), NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().to_string());

    }

    #[test]
    fn test_should_convert_to_string() {
        let commit = Commit::new(
            Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a8").unwrap(),
            Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap(),
            "Init commit".to_string(),
            NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        let commit_entity = CommitEntity::from(&commit);

        assert_eq!(commit_entity.to_string(), "936da01f-9abd-4d9d-80c7-02af85c822a8,936da01f-9abd-4d9d-80c7-02af85c822a7,Init commit,2023-01-01 12:00:00");
    }

    #[test]
    fn test_should_convert_from_string() {
        let line = "936da01f-9abd-4d9d-80c7-02af85c822a8,936da01f-9abd-4d9d-80c7-02af85c822a7,Init commit,2023-01-01 12:00:00";

        let commit_entity = CommitEntity::from_string(line);

        assert_eq!(commit_entity.id().to_string(), "936da01f-9abd-4d9d-80c7-02af85c822a8");
        assert_eq!(commit_entity.parent_id().to_string(), Uuid::parse_str("936da01f-9abd-4d9d-80c7-02af85c822a7").unwrap().to_string());
        assert_eq!(commit_entity.message(), "Init commit");
        assert_eq!(commit_entity.created_at().to_string(), NaiveDateTime::parse_from_str("2023-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap().to_string());
    }
}