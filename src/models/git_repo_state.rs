// src/models/git_repo_state.rs

#[derive(Debug, Default)]
pub struct GitRepoState {
    pub branch: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub staged: usize,
    pub unstaged: usize,
    pub untracked: usize,
    pub stashed: usize,
}
