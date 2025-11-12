use assert_cmd::Command;
use predicates::str::contains;
use tempfile::TempDir;
use std::fs;
use std::process::Command as StdCommand;

fn init_git_repo(path: &std::path::Path) {
    StdCommand::new("git").arg("init").current_dir(path).output().unwrap();
    // Set user config to avoid git warnings
    StdCommand::new("git").args(["config", "user.email", "test@example.com"]).current_dir(path).output().unwrap();
    StdCommand::new("git").args(["config", "user.name", "Test User"]).current_dir(path).output().unwrap();
}

#[test]
fn prompt_shows_branch_clean_repo() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());
    // Should show branch name, but no status symbols
    cmd.assert().stdout(contains(" "));
}

#[test]
fn prompt_shows_untracked_file() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    fs::write(tmp.path().join("foo.txt"), "bar").unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());
    cmd.assert().stdout(contains("*"));
}

#[test]
fn prompt_shows_staged_file() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    let file = tmp.path().join("foo.txt");
    fs::write(&file, "bar").unwrap();
    StdCommand::new("git").args(["add", "foo.txt"]).current_dir(tmp.path()).output().unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());
    cmd.assert().stdout(contains("+"));
}

#[test]
fn prompt_shows_unstaged_changes() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    let file = tmp.path().join("foo.txt");
    fs::write(&file, "bar").unwrap();
    StdCommand::new("git").args(["add", "foo.txt"]).current_dir(tmp.path()).output().unwrap();
    // Commit the file
    StdCommand::new("git").args(["commit", "-m", "add foo"]).current_dir(tmp.path()).output().unwrap();
    // Modify the file (unstaged change)
    fs::write(&file, "baz").unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());
    cmd.assert().stdout(contains("×"));
}

#[test]
fn prompt_shows_stashed_changes() {
    let tmp = TempDir::new().unwrap();
    init_git_repo(tmp.path());
    let file = tmp.path().join("foo.txt");
    fs::write(&file, "bar").unwrap();
    StdCommand::new("git").args(["add", "foo.txt"]).current_dir(tmp.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "add foo"]).current_dir(tmp.path()).output().unwrap();
    // Modify and stash
    fs::write(&file, "baz").unwrap();
    StdCommand::new("git").args(["stash"]).current_dir(tmp.path()).output().unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());
    cmd.assert().stdout(contains("•"));
}

fn setup_remote_and_clones() -> (TempDir, TempDir, TempDir) {
    // Returns (bare_remote, clone1, clone2)
    let bare = TempDir::new().unwrap();
    let c1 = TempDir::new().unwrap();
    let c2 = TempDir::new().unwrap();
    StdCommand::new("git").arg("init").arg("--bare").current_dir(bare.path()).output().unwrap();
    StdCommand::new("git").arg("clone").arg(bare.path()).arg(c1.path()).output().unwrap();
    StdCommand::new("git").arg("clone").arg(bare.path()).arg(c2.path()).output().unwrap();
    // Set user config for both clones
    for c in [&c1, &c2] {
        StdCommand::new("git").args(["config", "user.email", "test@example.com"]).current_dir(c.path()).output().unwrap();
        StdCommand::new("git").args(["config", "user.name", "Test User"]).current_dir(c.path()).output().unwrap();
    }
    (bare, c1, c2)
}

#[test]
fn prompt_shows_ahead_of_remote() {
    let (_bare, c1, _c2) = setup_remote_and_clones();
    let file = c1.path().join("foo.txt");
    fs::write(&file, "bar").unwrap();
    StdCommand::new("git").args(["add", "foo.txt"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "add foo"]).current_dir(c1.path()).output().unwrap();
    // Initial push to create remote-tracking branch
    StdCommand::new("git").args(["push", "-u", "origin", "master"]).current_dir(c1.path()).output().unwrap();
    // New commit to be ahead
    fs::write(c1.path().join("ahead.txt"), "a").unwrap();
    StdCommand::new("git").args(["add", "ahead.txt"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "ahead"]).current_dir(c1.path()).output().unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(c1.path());
    cmd.assert().stdout(contains("↑"));
}

#[test]
fn prompt_shows_behind_remote() {
    let (_bare, c1, c2) = setup_remote_and_clones();
    // c1 creates initial commit and pushes
    let file1 = c1.path().join("initial.txt");
    fs::write(&file1, "initial").unwrap();
    StdCommand::new("git").args(["add", "initial.txt"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "initial"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["push", "-u", "origin", "master"]).current_dir(c1.path()).output().unwrap();
    // c2 pulls, commits, and pushes (making c1 behind)
    StdCommand::new("git").arg("pull").current_dir(c2.path()).output().unwrap();
    let file2 = c2.path().join("foo.txt");
    fs::write(&file2, "bar").unwrap();
    StdCommand::new("git").args(["add", "foo.txt"]).current_dir(c2.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "add foo"]).current_dir(c2.path()).output().unwrap();
    StdCommand::new("git").args(["push"]).current_dir(c2.path()).output().unwrap();
    // c1 fetches but does not pull, so it is behind
    StdCommand::new("git").arg("fetch").current_dir(c1.path()).output().unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(c1.path());
    cmd.assert().stdout(contains("↓"));
}

#[test]
fn prompt_shows_ahead_and_behind() {
    let (_bare, c1, c2) = setup_remote_and_clones();
    // c1 creates initial commit and pushes
    let file_init = c1.path().join("initial.txt");
    fs::write(&file_init, "initial").unwrap();
    StdCommand::new("git").args(["add", "initial.txt"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "initial"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["push", "-u", "origin", "master"]).current_dir(c1.path()).output().unwrap();
    // c2 pulls, commits, and pushes (making c1 behind)
    StdCommand::new("git").arg("pull").current_dir(c2.path()).output().unwrap();
    let file2 = c2.path().join("foo.txt");
    fs::write(&file2, "bar").unwrap();
    StdCommand::new("git").args(["add", "foo.txt"]).current_dir(c2.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "add foo"]).current_dir(c2.path()).output().unwrap();
    StdCommand::new("git").args(["push"]).current_dir(c2.path()).output().unwrap();
    // c1 fetches (now behind)
    StdCommand::new("git").arg("fetch").current_dir(c1.path()).output().unwrap();
    // c1 also commits locally (now ahead and behind)
    let file1 = c1.path().join("bar.txt");
    fs::write(&file1, "baz").unwrap();
    StdCommand::new("git").args(["add", "bar.txt"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "add bar"]).current_dir(c1.path()).output().unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(c1.path());
    let out = cmd.assert().get_output().stdout.clone();
    let s = String::from_utf8_lossy(&out);
    assert!(s.contains("↑") && s.contains("↓"));
}

#[test]
fn prompt_shows_all_states() {
    let (_bare, c1, _c2) = setup_remote_and_clones();
    // Commit and push in c1
    let file = c1.path().join("foo.txt");
    fs::write(&file, "bar").unwrap();
    StdCommand::new("git").args(["add", "foo.txt"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "add foo"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["push", "-u", "origin", "master"]).current_dir(c1.path()).output().unwrap();
    // New untracked file
    fs::write(c1.path().join("untracked.txt"), "u").unwrap();
    // Unstaged change
    fs::write(&file, "baz").unwrap();
    // Stash
    StdCommand::new("git").args(["stash"]).current_dir(c1.path()).output().unwrap();
    // Commit and do not push (ahead)
    fs::write(c1.path().join("ahead.txt"), "a").unwrap();
    StdCommand::new("git").args(["add", "ahead.txt"]).current_dir(c1.path()).output().unwrap();
    StdCommand::new("git").args(["commit", "-m", "ahead"]).current_dir(c1.path()).output().unwrap();
    // Staged file (add after all commits so it is staged and not committed)
    fs::write(c1.path().join("staged.txt"), "s").unwrap();
    StdCommand::new("git").args(["add", "staged.txt"]).current_dir(c1.path()).output().unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(c1.path());
    let out = cmd.assert().get_output().stdout.clone();
    let s = String::from_utf8_lossy(&out);
    for sym in ["*", "+", "×", "•", "↑"] {
        assert!(s.contains(sym), "missing symbol: {} in {}", sym, s);
    }
}
