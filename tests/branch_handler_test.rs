mod branch_fixtures;

use mockall::{mock, Predicate};
use git_rust::domain::branch::Branch;
use git_rust::domain::branches_repository::BranchesRepository;
use git_rust::domain::current_branch_repository::CurrentBranchRepository;

mock! {
    pub BranchesRepository {}

    impl BranchesRepository for BranchesRepository {
        fn save(&self, branch: &Branch);
        fn get_branches(&self) -> Vec<Branch>;
        fn get_by_name(&self, branch_name: String) -> Option<Branch>;
    }
}

mock! {
    pub CurrentBranchRepository {}

    impl CurrentBranchRepository for CurrentBranchRepository {
        fn save(&self, branch: &Branch);
        fn get(&self) -> Option<Branch>;
    }
}

#[cfg(test)]
mod branch_handler_test {
    use mockall::predicate::eq;
    use git_rust::domain::branch_handler::BranchHandler;
    use crate::branch_fixtures::{sample_branch, sample_branch_to_create};
    use crate::{MockBranchesRepository, MockCurrentBranchRepository};

    #[test]
    fn test_should_create_branch() {

        let mut mock_branches_repo = MockBranchesRepository::new();
        mock_branches_repo
            .expect_save()
            .times(1)
            .with(eq(sample_branch()))
            .return_const(());

        let mock_current_branch_repo = MockCurrentBranchRepository::new();

        let branch_handler = BranchHandler::new(Box::new(mock_branches_repo), Box::new(mock_current_branch_repo));
        let branch = branch_handler.create_branch(sample_branch_to_create());

        assert_eq!(branch.name(), "toto");
    }

    #[test]
    fn test_should_checkout_branch() {

        let mut mock_branches_repo = MockBranchesRepository::new();
        mock_branches_repo.expect_get_by_name().times(1).with(eq("toto".to_string())).return_const(Some(sample_branch()));

        let mut mock_current_branch_repo = MockCurrentBranchRepository::new();
        mock_current_branch_repo.expect_save().times(1).with(eq(sample_branch())).return_const(());

        let branch_handler = BranchHandler::new(Box::new(mock_branches_repo), Box::new(mock_current_branch_repo));
        let branch = branch_handler.checkout("toto".to_string());

        assert_eq!(branch.name(), "toto");
    }

    #[test]
    #[should_panic(expected = "Branch not found")]
    fn test_should_panic_error_when_branch_not_found() {

        let mut mock_branches_repo = MockBranchesRepository::new();
        mock_branches_repo.expect_get_by_name().times(1).with(eq("toto".to_string())).return_const(None);

        let mut mock_current_branch_repo = MockCurrentBranchRepository::new();
        mock_current_branch_repo.expect_save().times(1).with(eq(sample_branch())).return_const(());

        let branch_handler = BranchHandler::new(Box::new(mock_branches_repo), Box::new(mock_current_branch_repo));
        let _ = branch_handler.checkout("toto".to_string());
    }
}