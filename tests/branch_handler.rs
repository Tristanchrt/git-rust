mod branch_fixtures;

use std::fmt::Display;
use chrono::NaiveDateTime;
use mockall::{mock, predicate::*, Predicate};
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

fn branch_matcher(name: &str, date: &str) -> Branch {
    let created_at = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").unwrap();
    Branch::new(name.to_string(), created_at)
}

#[cfg(test)]
mod branch_handler_test {
    use mockall::predicate::eq;
    use git_rust::domain::branch_handler::BranchHandler;
    use crate::branch_fixtures::sample_branch_to_create;
    use crate::{branch_matcher, MockBranchesRepository, MockCurrentBranchRepository};

    // TODO : https://github.com/asomers/mockall
    #[test]
    fn test_should_create_branch() {

        let mut mock_branches_repo = MockBranchesRepository::new();
        mock_branches_repo.expect_save()
            .times(1).with();

        let mock_current_branch_repo = MockCurrentBranchRepository::new();

        let branch_handler = BranchHandler::new(Box::new(mock_branches_repo), Box::new(mock_current_branch_repo));
        let branch = branch_handler.create_branch(sample_branch_to_create());

        assert_eq!(branch.name(), "toto");
    }
}