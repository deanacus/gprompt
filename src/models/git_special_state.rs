// src/models/git_special_state.rs
//! Data structures for git special state handling.
//!
//! This module defines the `GitSpecialState` enum and related types for representing
//! and displaying various git repository states such as rebasing, merging, cherry-picking,
//! and detached HEAD.
//!
//! # Examples
//!
//! ```
//! use gprompt::models::git_special_state::{GitSpecialState, OperationProgress};
//!
//! // Create a rebasing state with progress
//! let progress = OperationProgress::new(3, 7).unwrap();
//! let state = GitSpecialState::Rebasing(Some(progress));
//! assert_eq!(state.display_name(), Some("Rebasing 3/7".to_string()));
//!
//! // Create a detached HEAD state
//! let state = GitSpecialState::Detached("a1b2c3d".to_string());
//! assert_eq!(state.display_name(), Some("Detached @ a1b2c3d".to_string()));
//! ```

/// Represents progress through a multi-step git operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationProgress {
    /// Current step number (1-indexed for display)
    pub current: usize,

    /// Total number of steps
    pub total: usize,
}

impl OperationProgress {
    /// Creates a new OperationProgress if values are valid
    ///
    /// # Arguments
    /// * `current` - Current operation step (must be > 0 and <= total)
    /// * `total` - Total operations (must be > 0)
    ///
    /// # Returns
    /// * `Some(OperationProgress)` if valid, `None` otherwise
    ///
    /// # Validation Rules
    /// - `current > 0` (operations are 1-indexed for display)
    /// - `total > 0` (must have at least one operation)
    /// - `current <= total` (current cannot exceed total)
    pub fn new(current: usize, total: usize) -> Option<Self> {
        if current > 0 && total > 0 && current <= total {
            Some(Self { current, total })
        } else {
            None
        }
    }
}

/// Represents special git operations in progress
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GitSpecialState {
    /// Repository is in normal state (on a branch, not in any special operation)
    #[default]
    Normal,

    /// Repository is in rebase state
    /// Contains optional progress information (`OperationProgress` with fields `current` and `total`)
    Rebasing(Option<OperationProgress>),

    /// Repository is in cherry-pick state
    /// Contains optional progress information (`OperationProgress` with fields `current` and `total`)
    CherryPicking(Option<OperationProgress>),

    /// Repository is in detached HEAD state
    /// Contains the short commit SHA (up to 7 characters, or full SHA if shorter)
    Detached(String),

    /// Repository is in merge state
    Merging,

    /// Repository is in revert state
    Reverting,

    /// Repository is in bisect state
    Bisecting,

    /// Repository is applying patches
    ApplyingPatches,
}

impl GitSpecialState {
    /// Returns the display string for this special state
    ///
    /// # Returns
    /// * `Option<String>` - Display string (e.g., "Rebasing 3/7"), or None if Normal
    ///
    /// # Examples
    /// ```
    /// use gprompt::models::git_special_state::{GitSpecialState, OperationProgress};
    ///
    /// let state = GitSpecialState::Rebasing(Some(OperationProgress::new(3, 7).unwrap()));
    /// assert_eq!(state.display_name(), Some("Rebasing 3/7".to_string()));
    ///
    /// let normal = GitSpecialState::Normal;
    /// assert_eq!(normal.display_name(), None);
    /// ```
    pub fn display_name(&self) -> Option<String> {
        match self {
            GitSpecialState::Normal => None,
            GitSpecialState::Rebasing(progress) => {
                Some(Self::format_with_progress("Rebasing", progress))
            }
            GitSpecialState::CherryPicking(progress) => {
                Some(Self::format_with_progress("Cherry-picking", progress))
            }
            GitSpecialState::Detached(sha) => Some(format!("Detached @ {sha}")),
            GitSpecialState::Merging => Some("Merging".to_string()),
            GitSpecialState::Reverting => Some("Reverting".to_string()),
            GitSpecialState::Bisecting => Some("Bisecting".to_string()),
            GitSpecialState::ApplyingPatches => Some("Applying patches".to_string()),
        }
    }

    fn format_with_progress(state_name: &str, progress: &Option<OperationProgress>) -> String {
        match progress {
            Some(p) => format!("{} {}/{}", state_name, p.current, p.total),
            None => state_name.to_string(),
        }
    }
}
