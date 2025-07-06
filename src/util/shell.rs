use std::io::Result;
use std::process::{Command, Output};

pub fn git_add_jsons() -> Result<Output> {
    Command::new("git")
        .args(["add", "-A", "admins.json", "authors.json", "corpora.json", "likes.json", "links.json", "layouts.json", "cached_stats.json"])
        .output()
}

pub fn git_commit_sync_data() -> Result<Output> {
    Command::new("git")
        .args(["commit", "-m", "Sync data"])
        .output()
}

pub fn git_pull() -> Result<Output> {
    Command::new("git")
        .arg("pull")
        .output()
}

pub fn git_push() -> Result<Output> {
    Command::new("git")
        .arg("push")
        .output()
}