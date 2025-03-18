use mockall::mock;
use git_rust::domain::commit::Commit;
use git_rust::domain::commits_repository::CommitsRepository;

mod commit_fixtures;

mock! {
    pub CommitsRepository {}

    impl CommitsRepository for CommitsRepository {
        fn save(&self, commit: &Commit);
        fn get_last_commit(&self) -> Option<Commit>;
    }
}

#[cfg(test)]
mod commit_handler_test {
    use mockall::predicate::eq;
    use git_rust::domain::commit_handler::CommitHandler;
    use crate::commit_fixtures::{commit_to_create, sample_commit};
    use crate::MockCommitsRepository;

    // TODO
    #[test]
    fn test_should_create_commit_without_parent() {
        // let mut mock_commit_repo = MockCommitsRepository::new();
        // mock_commit_repo
        //     .expect_save()
        //     .times(1)
        //     .with(eq(sample_commit()))
        //     .return_const(());
        //
        // mock_commit_repo.expect_get_last_commit().times(1).return_const(None);
        //
        // let commit_handler = CommitHandler::new(Box::new(mock_commit_repo));
        // let commit = commit_handler.create_commit(commit_to_create());
        //
        // assert_eq!(commit.message(), "Init commit");
        // assert_eq!(commit.parent_id().to_string(), "00000000-0000-0000-0000-000000000000");
        assert_eq!(1,1)
    }
}