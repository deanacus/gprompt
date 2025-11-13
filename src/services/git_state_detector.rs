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
    // T021: Add rebase state detection logic using Repository::state()
    let state = repo.state();

    #[allow(unreachable_patterns)]
    match state {
        git2::RepositoryState::Rebase
        | git2::RepositoryState::RebaseInteractive
        | git2::RepositoryState::RebaseMerge => {
            // T020: Call detect_rebase_state() helper function
            detect_rebase_state(repo)
        }
        // T032: Add cherry-pick state detection using RepositoryState::CherryPick
        git2::RepositoryState::CherryPick | git2::RepositoryState::CherryPickSequence => {
            // T033: Implement cherry-pick sequence detection
            // NOTE: git2 API does not expose step-by-step progress for sequences
            // T034: Fallback to display "Cherry-picking" without progress
            GitSpecialState::CherryPicking(None)
        }
        // T049: Add merge state detection using RepositoryState::Merge
        git2::RepositoryState::Merge => {
            // T050: Return GitSpecialState::Merging when merge state detected
            GitSpecialState::Merging
        }
        // T055: Add revert state detection using RepositoryState::Revert
        git2::RepositoryState::Revert | git2::RepositoryState::RevertSequence => {
            // T056: Handle Revert and RevertSequence states
            // T057: Return GitSpecialState::Reverting when detected
            GitSpecialState::Reverting
        }
        // T059: Add Bisecting state detection using RepositoryState::Bisect
        git2::RepositoryState::Bisect => GitSpecialState::Bisecting,
        // T060: Add ApplyingPatches state detection using RepositoryState::ApplyMailbox
        git2::RepositoryState::ApplyMailbox | git2::RepositoryState::ApplyMailboxOrRebase => {
            GitSpecialState::ApplyingPatches
        }
        git2::RepositoryState::Clean => {
            // T044: Ensure detached HEAD check only runs when repo state is Clean
            // T041: Add detached HEAD check using Repository::head_detached()
            detect_detached_head(repo)
        }
        _ => GitSpecialState::Normal,
    }
}

// T020: Implement detect_rebase_state() helper function
/// Detects rebase state and extracts progress information
///
/// # Arguments
/// * `repo` - Reference to an open git2::Repository
///
/// # Returns
/// * `GitSpecialState::Rebasing` - With progress if available, None otherwise
fn detect_rebase_state(repo: &git2::Repository) -> GitSpecialState {
    // T022: Implement rebase progress extraction using Repository::open_rebase()
    // T023: Add error handling for corrupted rebase state (fallback to Rebasing(None))
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
            // T023: Error opening rebase (corrupted state or other issues)
            // Fallback to Rebasing without progress
            GitSpecialState::Rebasing(None)
        }
    }
}

// T040: Implement detect_detached_head() helper function
/// Detects detached HEAD state and extracts short commit SHA
///
/// # Arguments
/// * `repo` - Reference to an open git2::Repository
///
/// # Returns
/// * `GitSpecialState::Detached` - With short SHA (7 chars) if available
/// * `GitSpecialState::Normal` - If HEAD is not detached
fn detect_detached_head(repo: &git2::Repository) -> GitSpecialState {
    // T041: Add detached HEAD check using Repository::head_detached()
    match repo.head_detached() {
        Ok(true) => {
            // HEAD is detached, extract the short SHA
            // T042: Extract short SHA (7 characters) from HEAD reference
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
                        // T043: Add error handling for missing HEAD target (fallback to "unknown")
                        GitSpecialState::Detached("unknown".to_string())
                    }
                }
                Err(_) => {
                    // T043: Error reading HEAD (fallback to "unknown")
                    GitSpecialState::Detached("unknown".to_string())
                }
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
