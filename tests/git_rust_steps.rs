

#[cfg(test)]
mod commit_handler_test {
    use std::process::{Command, Output};

    fn run_cargo(args: Vec<&str>) -> Output {
        Command::new("cargo")
            .args(&["run", "--"])
            .args(args)
            .output()
            .expect("Failed to execute cargo run")
    }

    #[test]
    fn should_save_commit() {
        let output = run_cargo(vec!["commit", "-m", "I'm a new Commit"]);

        assert!(String::from_utf8_lossy(&output.stdout).contains("Committing changes "));
        assert!(String::from_utf8_lossy(&output.stdout).contains("I'm a new Commit"));
    }

    #[test]
    fn should_get_error_when_missing_message() {
        let output = run_cargo(vec!["commit", "-m"]);

        assert!(String::from_utf8_lossy(&output.stdout).contains("No message provided"));
    }

    #[test]
    fn should_get_error_when_missing_command() {
        let output = run_cargo(vec!["branch"]);

        assert!(String::from_utf8_lossy(&output.stdout).contains("No command provided"));
    }

    #[test]
    fn should_get_error_when_unknown_command() {
        let output = run_cargo(vec!["branch", "-z"]);

        assert!(String::from_utf8_lossy(&output.stdout).contains("Branch Command not found"));
    }

    #[test]
    fn should_not_save_when_missing_name() {
        let output = run_cargo(vec!["branch", "-c"]);

        assert!(String::from_utf8_lossy(&output.stdout).contains("No branch name provided"));
    }

    #[test]
    fn should_save_branch() {
        let output = run_cargo(vec!["branch", "-c", "feat/toto"]);

        assert!(String::from_utf8_lossy(&output.stdout).contains("Branch created "));
        assert!(String::from_utf8_lossy(&output.stdout).contains("feat/toto"));
    }

    #[test]
    fn should_not_checkout_when_missing_name() {
        let output = run_cargo(vec!["branch", "-m"]);

        assert!(String::from_utf8_lossy(&output.stdout).contains("No branch name provided"));
    }

    #[test]
    fn should_checkout_branch() {
        let _ = run_cargo(vec!["branch", "-c", "feat/toto"]);
        let output = run_cargo(vec!["branch", "-m", "feat/toto"]);

        assert!(String::from_utf8_lossy(&output.stdout).contains("Checkout branch "));
        assert!(String::from_utf8_lossy(&output.stdout).contains("feat/toto"));
    }
}