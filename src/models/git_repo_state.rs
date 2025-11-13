// src/models/git_repo_state.rs

use crate::models::git_special_state::GitSpecialState;

#[derive(Debug, Default)]
pub struct GitRepoState {
    /// Branch name (remains populated even in special states; display logic determines what to show)
    pub branch: Option<String>,

    /// Special git state (if any)
    pub special_state: GitSpecialState,

    pub ahead: usize,
    pub behind: usize,
    pub staged: usize,
    pub unstaged: usize,
    pub untracked: usize,
    pub stashed: usize,
}
