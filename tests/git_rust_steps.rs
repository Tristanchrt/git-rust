

#[cfg(test)]
mod commit_handler_test {
    use std::process::Command;

    // TODO
    #[test]
    fn should_save_commit() {
        let output = Command::new("cargo")
            .args(&["run", "--", "branch", "-m", "I'm a new Commit"])
            .output()
            .expect("Failed to execute cargo run");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Committing changes "));
        assert!(String::from_utf8_lossy(&output.stdout).contains("I'm a new Commit"));
    }

    #[test]
    fn should_save_branch() {
        let output = Command::new("cargo")
            .args(&["run", "--", "branch", "-c", "feat/toto"])
            .output()
            .expect("Failed to execute cargo run");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Branch created "));
        assert!(String::from_utf8_lossy(&output.stdout).contains("feat/toto"));
    }

    #[test]
    fn should_checkout_branch() {
        let _ = Command::new("cargo")
            .args(&["run", "--", "branch", "-c", "feat/toto"])
            .output()
            .expect("Failed to execute cargo run");

        let output = Command::new("cargo")
            .args(&["run", "--", "branch", "-m", "feat/toto"])
            .output()
            .expect("Failed to execute cargo run");

        assert!(String::from_utf8_lossy(&output.stdout).contains("Branch checkout "));
        assert!(String::from_utf8_lossy(&output.stdout).contains("feat/toto"));
    }
}