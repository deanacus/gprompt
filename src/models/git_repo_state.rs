// src/models/git_repo_state.rs

#[derive(Debug, Default)]
pub struct GitRepoState {
    pub branch: Option<String>,
    pub ahead: usize,
    pub behind: usize,
    pub staged: usize,
    pub unstaged: usize,
    pub untracked: usize,
    pub stashed: usize,
}
