use std::io::{Result, Write};
use std::process::{Command, Output};

fn git_add_jsons() -> Result<Output> {
    Command::new("git")
        .args(["add", "-A", "admins.json", "authors.json", "corpora.json", "likes.json", "links.json", "layouts.json", "cached_stats.json"])
        .output()
}

fn git_commit_sync_data() -> Result<Output> {
    Command::new("git")
        .args(["commit", "-m", "Sync data"])
        .output()
}

pub fn git_pull() -> Result<Output> {
    Command::new("git")
        .arg("pull")
        .output()
}

fn git_push() -> Result<Output> {
    Command::new("git")
        .arg("push")
        .output()
}

pub fn sync_github() -> Result<()> {
    let outputs = [
        git_add_jsons()?,
        git_commit_sync_data()?,
        git_pull()?,
        git_push()?,
    ];
    for output in outputs {
        std::io::stdout().write_all(&output.stdout)?;
        std::io::stderr().write_all(&output.stderr)?;
    }
    Ok(())
}