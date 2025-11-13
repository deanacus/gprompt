// src/services/git_status.rs

use crate::models::git_repo_state::GitRepoState;
use crate::services::git_state_detector;
use git2::{Repository, Status, StatusOptions};

pub fn get_git_repo_state(cwd: &std::path::Path) -> Option<GitRepoState> {
    let mut repo = Repository::discover(cwd).ok()?;
    if repo.is_bare() {
        return None;
    }

    let branch = branch_name(&repo);
    let (ahead, behind) = get_ahead_behind(&repo);
    let staged = get_staged(&repo);
    let unstaged = get_unstaged(&repo);
    let untracked = get_untracked(&repo);
    let stashed = get_stash(&mut repo);

    let special_state = git_state_detector::detect_special_state(&repo);

    Some(GitRepoState {
        branch,
        special_state,
        ahead,
        behind,
        staged,
        unstaged,
        untracked,
        stashed,
    })
}

fn branch_name(repository: &Repository) -> Option<String> {
    match repository.head_detached() {
        Ok(true) => return None,
        Err(_) => return None,
        _ => {}
    }
    let head = repository.head().ok()?;
    head.shorthand().map(|s| s.to_string())
}

fn get_staged(repository: &Repository) -> usize {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = match repository.statuses(Some(&mut opts)) {
        Ok(s) => s,
        Err(_) => return 0,
    };
    statuses
        .iter()
        .filter(|entry| {
            entry.status().intersects(
                Status::INDEX_NEW
                    | Status::INDEX_MODIFIED
                    | Status::INDEX_DELETED
                    | Status::INDEX_TYPECHANGE
                    | Status::INDEX_RENAMED,
            )
        })
        .count()
}

fn get_unstaged(repository: &Repository) -> usize {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = match repository.statuses(Some(&mut opts)) {
        Ok(s) => s,
        Err(_) => return 0,
    };
    statuses
        .iter()
        .filter(|entry| {
            entry.status().intersects(
                Status::WT_MODIFIED
                    | Status::WT_DELETED
                    | Status::WT_TYPECHANGE
                    | Status::WT_RENAMED,
            )
        })
        .count()
}

fn get_untracked(repository: &Repository) -> usize {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = match repository.statuses(Some(&mut opts)) {
        Ok(s) => s,
        Err(_) => return 0,
    };
    statuses
        .iter()
        .filter(|entry| entry.status().intersects(Status::WT_NEW))
        .count()
}

fn get_stash(repo: &mut Repository) -> usize {
    let mut count = 0;
    let _ = repo.stash_foreach(|_, _, _| {
        count += 1;
        true
    });
    count
}

fn get_ahead_behind(repo: &Repository) -> (usize, usize) {
    let head = match repo.head() {
        Ok(h) => h,
        Err(_) => return (0, 0),
    };
    let head_name = match head.shorthand() {
        Some(name) => name,
        None => return (0, 0),
    };
    let head_branch = match repo.find_branch(head_name, git2::BranchType::Local) {
        Ok(b) => b,
        Err(_) => return (0, 0),
    };
    let upstream = match head_branch.upstream() {
        Ok(up) => up,
        Err(_) => return (0, 0),
    };
    let head_oid = match head.target() {
        Some(oid) => oid,
        None => return (0, 0),
    };
    let upstream_oid = match upstream.get().target() {
        Some(oid) => oid,
        None => return (0, 0),
    };
    let (ahead, behind) = repo
        .graph_ahead_behind(head_oid, upstream_oid)
        .unwrap_or((0, 0));
    (ahead, behind)
}
