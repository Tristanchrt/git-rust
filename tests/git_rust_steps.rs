mod file_shared;

#[cfg(test)]
mod git_rust_steps_test {
    use std::process::{Command, Output};

    mod commit_steps {
        use crate::file_shared::{clean_db_test, clean_dir};
        use crate::git_rust_steps_test::run_cargo;

        #[test]
        fn should_get_error_when_unknown_command() {
            let output = run_cargo(vec!["commit", "-z"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("Commit Command not found"));

            clean_db_test()
        }

        #[test]
        fn should_get_error_when_missing_command() {
            let output = run_cargo(vec!["commit"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("No command provided"));

            clean_db_test()
        }

        #[test]
        fn should_save_commit_and_checkout() {
            let _ = run_cargo(vec!["commit", "-m", "I'm a new Commit"]);
            let _ = run_cargo(vec!["branch", "-c", "feat/toto"]);
            let _ = run_cargo(vec!["branch", "-m", "feat/toto"]);
            let output = run_cargo(vec!["commit", "-m", "I'm a new Commit for feat/toto"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("Committing changes "));
            assert!(
                String::from_utf8_lossy(&output.stdout).contains("I'm a new Commit for feat/toto")
            );
            assert!(String::from_utf8_lossy(&output.stdout).contains("feat/toto"));
            assert!(String::from_utf8_lossy(&output.stdout).contains("6cd715972837aa872957f1826ac0d82836f3e3b8"));

            clean_db_test()
        }

        #[test]
        fn should_get_error_when_missing_message() {
            let output = run_cargo(vec!["commit", "-m"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("No message provided"));

            clean_db_test()
        }

        #[test]
        fn should_get_error_when_missing_branch_name() {
            let output = run_cargo(vec!["commit", "-l"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("No branch name provided"));

            clean_db_test()
        }

        #[test]
        fn should_get_commits_from_branch() {
            let _ = run_cargo(vec!["branch", "-c", "feat/toto2"]);
            let output = run_cargo(vec!["branch", "-m", "feat/toto2"]);

            let _ = run_cargo(vec!["commit", "-m", "I m a new Commit"]);
            let _ = run_cargo(vec!["commit", "-m", "I m a second Commit"]);
            let output = run_cargo(vec!["commit", "-l", "feat/toto2"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("Commits listed"));
            assert!(String::from_utf8_lossy(&output.stdout).contains("I m a new Commit"));
            assert!(String::from_utf8_lossy(&output.stdout).contains("I m a second Commit"));
            assert!(String::from_utf8_lossy(&output.stdout).contains("feat/toto2"));

            clean_db_test()
        }
    }

    mod branch_steps {
        use crate::file_shared::clean_db_test;
        use crate::git_rust_steps_test::run_cargo;

        #[test]
        fn should_get_error_when_missing_command() {
            let output = run_cargo(vec!["branch"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("No command provided"));

            clean_db_test()
        }

        #[test]
        fn should_get_error_when_unknown_command() {
            let output = run_cargo(vec!["branch", "-z"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("Branch Command not found"));

            clean_db_test()
        }

        #[test]
        fn should_not_save_when_missing_name() {
            let output = run_cargo(vec!["branch", "-c"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("No branch name provided"));

            clean_db_test()
        }

        #[test]
        fn should_save_branch() {
            let output = run_cargo(vec!["branch", "-c", "feat/toto"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("Branch created "));
            assert!(String::from_utf8_lossy(&output.stdout).contains("feat/toto"));

            clean_db_test()
        }

        #[test]
        fn should_not_checkout_when_missing_name() {
            let output = run_cargo(vec!["branch", "-m"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("No branch name provided"));

            clean_db_test()
        }

        #[test]
        fn should_checkout_branch() {
            let _ = run_cargo(vec!["branch", "-c", "feat/toto2"]);
            let output = run_cargo(vec!["branch", "-m", "feat/toto2"]);

            assert!(String::from_utf8_lossy(&output.stdout).contains("Checkout branch "));
            assert!(String::from_utf8_lossy(&output.stdout).contains("feat/toto2"));

            clean_db_test()
        }
    }

    fn run_cargo(args: Vec<&str>) -> Output {
        Command::new("cargo")
            .args(["run", "--"])
            .args(args)
            .output()
            .expect("Failed to execute cargo run")
    }
}
