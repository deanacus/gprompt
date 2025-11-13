// src/services/git_state_detector.rs
//! Logic for detecting git special states.
//!
//! This module provides functionality to detect various git repository states such as
//! rebasing, merging, cherry-picking, reverting, bisecting, detached HEAD, and more.
//!
//! # Performance
//!
//! State detection is designed to complete in <10ms for typical repositories.
//!
//! # Examples
//!
//! ```no_run
//! use git2::Repository;
//! use gprompt::services::git_state_detector::detect_special_state;
//!
//! let repo = Repository::open(".").unwrap();
//! let state = detect_special_state(&repo);
//! if let Some(name) = state.display_name() {
//!     println!("Repository is in state: {}", name);
//! }
//! ```

use crate::models::git_special_state::{GitSpecialState, OperationProgress};

/// Detects the special state of a git repository
///
/// # Arguments
/// * `repo` - Reference to an open git2::Repository
///
/// # Returns
/// * `GitSpecialState` - The detected special state (or Normal if none)
///
/// # Contract
/// - Always returns a valid GitSpecialState (never panics)
/// - Returns `GitSpecialState::Normal` if detection fails or repository is clean
/// - Side Effects: None (read-only operation)
/// - Performance: Completes in <10ms for typical repositories
pub fn detect_special_state(repo: &git2::Repository) -> GitSpecialState {
    let state = repo.state();

    #[allow(unreachable_patterns)]
    match state {
        git2::RepositoryState::Rebase
        | git2::RepositoryState::RebaseInteractive
        | git2::RepositoryState::RebaseMerge => detect_rebase_state(repo),
        git2::RepositoryState::CherryPick | git2::RepositoryState::CherryPickSequence => {
            // NOTE: git2 API does not expose step-by-step progress for sequences
            GitSpecialState::CherryPicking(None)
        }
        git2::RepositoryState::Merge => GitSpecialState::Merging,
        git2::RepositoryState::Revert | git2::RepositoryState::RevertSequence => {
            GitSpecialState::Reverting
        }
        git2::RepositoryState::Bisect => GitSpecialState::Bisecting,
        git2::RepositoryState::ApplyMailbox | git2::RepositoryState::ApplyMailboxOrRebase => {
            GitSpecialState::ApplyingPatches
        }
        git2::RepositoryState::Clean => detect_detached_head(repo),
        _ => GitSpecialState::Normal,
    }
}

/// Detects rebase state and extracts progress information
///
/// # Arguments
/// * `repo` - Reference to an open git2::Repository
///
/// # Returns
/// * `GitSpecialState::Rebasing` - With progress if available, None otherwise
fn detect_rebase_state(repo: &git2::Repository) -> GitSpecialState {
    match repo.open_rebase(None) {
        Ok(mut rebase) => {
            // Extract progress information
            let current_step = rebase.operation_current();
            let total_steps = rebase.len();

            // current_step is 0-based, we need 1-based for display
            if let Some(current) = current_step {
                // current is 0-based index, convert to 1-based step number
                let current_1based = current + 1;

                // Create OperationProgress with validation
                if let Some(progress) = OperationProgress::new(current_1based, total_steps) {
                    GitSpecialState::Rebasing(Some(progress))
                } else {
                    // Progress validation failed, return without progress
                    GitSpecialState::Rebasing(None)
                }
            } else {
                // No current step information available
                GitSpecialState::Rebasing(None)
            }
        }
        Err(_) => {
            // Fallback to Rebasing without progress
            GitSpecialState::Rebasing(None)
        }
    }
}

/// Detects detached HEAD state and extracts short commit SHA
///
/// # Arguments
/// * `repo` - Reference to an open git2::Repository
///
/// # Returns
/// * `GitSpecialState::Detached` - With short SHA (7 chars) if available
/// * `GitSpecialState::Normal` - If HEAD is not detached
fn detect_detached_head(repo: &git2::Repository) -> GitSpecialState {
    match repo.head_detached() {
        Ok(true) => {
            // HEAD is detached, extract the short SHA
            match repo.head() {
                Ok(head) => {
                    if let Some(oid) = head.target() {
                        // Get short SHA (7 characters)
                        let sha = oid.to_string();
                        let short_sha = if sha.len() >= 7 {
                            sha[..7].to_string()
                        } else {
                            sha
                        };
                        GitSpecialState::Detached(short_sha)
                    } else {
                        GitSpecialState::Detached("unknown".to_string())
                    }
                }
                Err(_) => GitSpecialState::Detached("unknown".to_string()),
            }
        }
        Ok(false) => {
            // HEAD is not detached, repository is in normal state
            GitSpecialState::Normal
        }
        Err(_) => {
            // Error checking HEAD detached status
            GitSpecialState::Normal
        }
    }
}
