use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::TempDir;

fn init_git_repo(path: &std::path::Path) {
    StdCommand::new("git")
        .arg("init")
        .current_dir(path)
        .output()
        .unwrap();
    // Set user config to avoid git warnings
    StdCommand::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(path)
        .output()
        .unwrap();
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
    StdCommand::new("git")
        .args(["add", "foo.txt"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
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
    StdCommand::new("git")
        .args(["add", "foo.txt"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    // Commit the file
    StdCommand::new("git")
        .args(["commit", "-m", "add foo"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
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
    StdCommand::new("git")
        .args(["add", "foo.txt"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "add foo"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    // Modify and stash
    fs::write(&file, "baz").unwrap();
    StdCommand::new("git")
        .args(["stash"])
        .current_dir(tmp.path())
        .output()
        .unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());
    cmd.assert().stdout(contains("•"));
}

fn setup_remote_and_clones() -> (TempDir, TempDir, TempDir) {
    // Returns (bare_remote, clone1, clone2)
    let bare = TempDir::new().unwrap();
    let c1 = TempDir::new().unwrap();
    let c2 = TempDir::new().unwrap();
    StdCommand::new("git")
        .arg("init")
        .arg("--bare")
        .current_dir(bare.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .arg("clone")
        .arg(bare.path())
        .arg(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .arg("clone")
        .arg(bare.path())
        .arg(c2.path())
        .output()
        .unwrap();
    // Set user config for both clones
    for c in [&c1, &c2] {
        StdCommand::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(c.path())
            .output()
            .unwrap();
        StdCommand::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(c.path())
            .output()
            .unwrap();
    }
    (bare, c1, c2)
}

#[test]
fn prompt_shows_ahead_of_remote() {
    let (_bare, c1, _c2) = setup_remote_and_clones();
    let file = c1.path().join("foo.txt");
    fs::write(&file, "bar").unwrap();
    StdCommand::new("git")
        .args(["add", "foo.txt"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "add foo"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    // Initial push to create remote-tracking branch
    StdCommand::new("git")
        .args(["push", "-u", "origin", "master"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    // New commit to be ahead
    fs::write(c1.path().join("ahead.txt"), "a").unwrap();
    StdCommand::new("git")
        .args(["add", "ahead.txt"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "ahead"])
        .current_dir(c1.path())
        .output()
        .unwrap();
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
    StdCommand::new("git")
        .args(["add", "initial.txt"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "initial"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["push", "-u", "origin", "master"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    // c2 pulls, commits, and pushes (making c1 behind)
    StdCommand::new("git")
        .arg("pull")
        .current_dir(c2.path())
        .output()
        .unwrap();
    let file2 = c2.path().join("foo.txt");
    fs::write(&file2, "bar").unwrap();
    StdCommand::new("git")
        .args(["add", "foo.txt"])
        .current_dir(c2.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "add foo"])
        .current_dir(c2.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["push"])
        .current_dir(c2.path())
        .output()
        .unwrap();
    // c1 fetches but does not pull, so it is behind
    StdCommand::new("git")
        .arg("fetch")
        .current_dir(c1.path())
        .output()
        .unwrap();
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
    StdCommand::new("git")
        .args(["add", "initial.txt"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "initial"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["push", "-u", "origin", "master"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    // c2 pulls, commits, and pushes (making c1 behind)
    StdCommand::new("git")
        .arg("pull")
        .current_dir(c2.path())
        .output()
        .unwrap();
    let file2 = c2.path().join("foo.txt");
    fs::write(&file2, "bar").unwrap();
    StdCommand::new("git")
        .args(["add", "foo.txt"])
        .current_dir(c2.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "add foo"])
        .current_dir(c2.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["push"])
        .current_dir(c2.path())
        .output()
        .unwrap();
    // c1 fetches (now behind)
    StdCommand::new("git")
        .arg("fetch")
        .current_dir(c1.path())
        .output()
        .unwrap();
    // c1 also commits locally (now ahead and behind)
    let file1 = c1.path().join("bar.txt");
    fs::write(&file1, "baz").unwrap();
    StdCommand::new("git")
        .args(["add", "bar.txt"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "add bar"])
        .current_dir(c1.path())
        .output()
        .unwrap();
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
    StdCommand::new("git")
        .args(["add", "foo.txt"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "add foo"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["push", "-u", "origin", "master"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    // New untracked file
    fs::write(c1.path().join("untracked.txt"), "u").unwrap();
    // Unstaged change
    fs::write(&file, "baz").unwrap();
    // Stash
    StdCommand::new("git")
        .args(["stash"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    // Commit and do not push (ahead)
    fs::write(c1.path().join("ahead.txt"), "a").unwrap();
    StdCommand::new("git")
        .args(["add", "ahead.txt"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "ahead"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    // Staged file (add after all commits so it is staged and not committed)
    fs::write(c1.path().join("staged.txt"), "s").unwrap();
    StdCommand::new("git")
        .args(["add", "staged.txt"])
        .current_dir(c1.path())
        .output()
        .unwrap();
    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(c1.path());
    let out = cmd.assert().get_output().stdout.clone();
    let s = String::from_utf8_lossy(&out);
    for sym in ["*", "+", "×", "•", "↑"] {
        assert!(s.contains(sym), "missing symbol: {} in {}", sym, s);
    }
}

// T018: Integration test helper to create repository in rebase state
fn create_repo_in_rebase_state(path: &std::path::Path) {
    init_git_repo(path);

    // Create initial commit on master
    fs::write(path.join("file.txt"), "line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "initial commit"])
        .current_dir(path)
        .output()
        .unwrap();

    // Create a feature branch and modify the same file
    StdCommand::new("git")
        .args(["checkout", "-b", "feature"])
        .current_dir(path)
        .output()
        .unwrap();
    fs::write(path.join("file.txt"), "FEATURE line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "feature change"])
        .current_dir(path)
        .output()
        .unwrap();

    // Add another commit to have multiple commits to rebase
    fs::write(path.join("file2.txt"), "feature file 2").unwrap();
    StdCommand::new("git")
        .args(["add", "file2.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "add feature file 2"])
        .current_dir(path)
        .output()
        .unwrap();

    // Go back to master and create a conflicting commit
    StdCommand::new("git")
        .args(["checkout", "master"])
        .current_dir(path)
        .output()
        .unwrap();
    fs::write(path.join("file.txt"), "MASTER line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "master change"])
        .current_dir(path)
        .output()
        .unwrap();

    // Go back to feature and rebase onto master (this WILL cause a conflict and pause the rebase)
    StdCommand::new("git")
        .args(["checkout", "feature"])
        .current_dir(path)
        .output()
        .unwrap();
    let _ = StdCommand::new("git")
        .args(["rebase", "master"])
        .current_dir(path)
        .output();
    // Rebase will stop at the conflict, leaving the repo in rebase state
}

// T019: Integration test to verify rebase state detection with progress
#[test]
fn test_rebase_state_detection() {
    let tmp = TempDir::new().unwrap();
    create_repo_in_rebase_state(tmp.path());

    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());

    // Should display "Rebasing" in the output
    cmd.assert().stdout(contains("Rebasing"));
}

// T030: Integration test helper to create repository in cherry-pick state
fn create_repo_in_cherry_pick_state(path: &std::path::Path) {
    init_git_repo(path);

    // Create initial commit on master
    fs::write(path.join("file.txt"), "line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "initial commit"])
        .current_dir(path)
        .output()
        .unwrap();

    // Create a feature branch with multiple commits to cherry-pick
    StdCommand::new("git")
        .args(["checkout", "-b", "feature"])
        .current_dir(path)
        .output()
        .unwrap();

    // First commit on feature
    fs::write(path.join("cherry1.txt"), "cherry 1").unwrap();
    StdCommand::new("git")
        .args(["add", "cherry1.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "cherry 1"])
        .current_dir(path)
        .output()
        .unwrap();
    let commit1 = StdCommand::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(path)
        .output()
        .unwrap();
    let commit1_sha = String::from_utf8_lossy(&commit1.stdout).trim().to_string();

    // Second commit on feature (this will conflict)
    fs::write(path.join("file.txt"), "FEATURE line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "cherry 2 - conflict"])
        .current_dir(path)
        .output()
        .unwrap();
    let commit2 = StdCommand::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(path)
        .output()
        .unwrap();
    let commit2_sha = String::from_utf8_lossy(&commit2.stdout).trim().to_string();

    // Third commit on feature
    fs::write(path.join("cherry3.txt"), "cherry 3").unwrap();
    StdCommand::new("git")
        .args(["add", "cherry3.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "cherry 3"])
        .current_dir(path)
        .output()
        .unwrap();
    let commit3 = StdCommand::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(path)
        .output()
        .unwrap();
    let commit3_sha = String::from_utf8_lossy(&commit3.stdout).trim().to_string();

    // Go back to master and make a conflicting change
    StdCommand::new("git")
        .args(["checkout", "master"])
        .current_dir(path)
        .output()
        .unwrap();
    fs::write(path.join("file.txt"), "MASTER line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "master change"])
        .current_dir(path)
        .output()
        .unwrap();

    // Cherry-pick multiple commits - will stop at conflict
    let _ = StdCommand::new("git")
        .args(["cherry-pick", &commit1_sha, &commit2_sha, &commit3_sha])
        .current_dir(path)
        .output();
    // Cherry-pick will stop at the conflict, leaving the repo in cherry-pick state
}

// T031: Integration test to verify cherry-pick state detection
#[test]
fn test_cherry_pick_state_detection() {
    let tmp = TempDir::new().unwrap();
    create_repo_in_cherry_pick_state(tmp.path());

    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());

    // Should display "Cherry-picking" in the output
    cmd.assert().stdout(contains("Cherry-picking"));
}

// T038: Integration test helper to create repository in detached HEAD state
fn create_repo_in_detached_head_state(path: &std::path::Path) -> String {
    init_git_repo(path);

    // Create initial commit
    fs::write(path.join("file.txt"), "initial content").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "initial commit"])
        .current_dir(path)
        .output()
        .unwrap();

    // Get the commit SHA
    let commit = StdCommand::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(path)
        .output()
        .unwrap();
    let commit_sha = String::from_utf8_lossy(&commit.stdout).trim().to_string();

    // Create another commit so we can checkout the previous one
    fs::write(path.join("file2.txt"), "second file").unwrap();
    StdCommand::new("git")
        .args(["add", "file2.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "second commit"])
        .current_dir(path)
        .output()
        .unwrap();

    // Checkout the first commit to create detached HEAD state
    StdCommand::new("git")
        .args(["checkout", &commit_sha])
        .current_dir(path)
        .output()
        .unwrap();

    // Return the short SHA (first 7 characters)
    commit_sha[..7].to_string()
}

// T039: Integration test to verify detached HEAD detection with SHA
#[test]
fn test_detached_head_state_detection() {
    let tmp = TempDir::new().unwrap();
    let short_sha = create_repo_in_detached_head_state(tmp.path());

    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());

    let out = cmd.assert().get_output().stdout.clone();
    let s = String::from_utf8_lossy(&out);

    // Should display "Detached" in the output
    assert!(
        s.contains("Detached"),
        "Expected 'Detached' in output: {}",
        s
    );

    // Should display the short SHA
    assert!(
        s.contains(&short_sha),
        "Expected SHA '{}' in output: {}",
        short_sha,
        s
    );
}

// T047: Integration test helper to create repository in merge state
fn create_repo_in_merge_state(path: &std::path::Path) {
    init_git_repo(path);

    // Create initial commit on master
    fs::write(path.join("file.txt"), "line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "initial commit"])
        .current_dir(path)
        .output()
        .unwrap();

    // Create a feature branch and modify the same file
    StdCommand::new("git")
        .args(["checkout", "-b", "feature"])
        .current_dir(path)
        .output()
        .unwrap();
    fs::write(path.join("file.txt"), "FEATURE line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "feature change"])
        .current_dir(path)
        .output()
        .unwrap();

    // Go back to master and create a conflicting commit
    StdCommand::new("git")
        .args(["checkout", "master"])
        .current_dir(path)
        .output()
        .unwrap();
    fs::write(path.join("file.txt"), "MASTER line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "master change"])
        .current_dir(path)
        .output()
        .unwrap();

    // Merge feature into master - this WILL cause a conflict and pause the merge
    let _ = StdCommand::new("git")
        .args(["merge", "feature"])
        .current_dir(path)
        .output();
    // Merge will stop at the conflict, leaving the repo in merge state
}

// T048: Integration test to verify merge state detection
#[test]
fn test_merge_state_detection() {
    let tmp = TempDir::new().unwrap();
    create_repo_in_merge_state(tmp.path());

    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());

    // Should display "Merging" in the output
    cmd.assert().stdout(contains("Merging"));
}

// T053: Integration test helper to create repository in revert state
fn create_repo_in_revert_state(path: &std::path::Path) {
    init_git_repo(path);

    // Create initial commit on master
    fs::write(path.join("file.txt"), "line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "initial commit"])
        .current_dir(path)
        .output()
        .unwrap();

    // Create a second commit that we'll want to revert
    fs::write(path.join("file.txt"), "MODIFIED line 1\nline 2\nline 3\n").unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "change to revert"])
        .current_dir(path)
        .output()
        .unwrap();

    // Create a third commit that conflicts with the revert
    fs::write(
        path.join("file.txt"),
        "CONFLICTING line 1\nline 2\nline 3\n",
    )
    .unwrap();
    StdCommand::new("git")
        .args(["add", "file.txt"])
        .current_dir(path)
        .output()
        .unwrap();
    StdCommand::new("git")
        .args(["commit", "-m", "conflicting commit"])
        .current_dir(path)
        .output()
        .unwrap();

    // Revert the second commit (HEAD~1) - this WILL cause a conflict
    let _ = StdCommand::new("git")
        .args(["revert", "HEAD~1"])
        .current_dir(path)
        .output();
    // Revert will stop at the conflict, leaving the repo in revert state
}

// T054: Integration test to verify revert state detection
#[test]
fn test_revert_state_detection() {
    let tmp = TempDir::new().unwrap();
    create_repo_in_revert_state(tmp.path());

    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());

    // Should display "Reverting" in the output
    cmd.assert().stdout(contains("Reverting"));
}

// T064: Test for missing .git directory (non-git repository)
#[test]
fn test_non_git_directory() {
    let tmp = TempDir::new().unwrap();
    // Don't initialize git - just create an empty directory

    let mut cmd = Command::cargo_bin("gprompt").unwrap();
    cmd.current_dir(tmp.path());

    // Should handle gracefully - output should be empty or show error message
    // The program should not panic
    cmd.assert().success();
}
