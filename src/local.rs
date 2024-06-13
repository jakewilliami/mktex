use super::config;
use std::path::PathBuf;
use std::process::{self, Command, Stdio};

/// Get local resource path
pub fn local_resource_path() -> PathBuf {
    let resource_dir = home::home_dir()
        .expect("Cannot get home directory")
        .join(config::RESOURCE_ROOT)
        .join(config::GITHUB_REPO_NAME);

    if !resource_dir.as_path().exists() {
        panic!(
            "{}",
            format!(
                "No local resource path at ~/{}/{}/",
                config::RESOURCE_ROOT,
                config::GITHUB_REPO_NAME
            )
        )
    }

    resource_dir
}

/// Get latest commit hash (SHA1 ID) from local repo
pub fn latest_local_commit_hash() -> String {
    // See jakewilliami/gl :D
    let mut cmd = Command::new("git");
    cmd.arg("-C");
    cmd.arg(local_resource_path());
    cmd.arg("rev-parse");
    cmd.arg(format!("--short={}", config::SHORT_HASH_LENGTH));
    cmd.arg("--verify");
    cmd.arg("HEAD");

    let output = cmd
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute `git rev-parse` to obtain local commit hash");

    if !output.status.success() {
        eprintln!(
            "[ERROR] Could not run `git rev-parse --short={} --verify HEAD`",
            config::SHORT_HASH_LENGTH
        );
        process::exit(1);
    }

    String::from_utf8_lossy(&output.stdout).into_owned()
}
