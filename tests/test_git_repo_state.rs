use gprompt::models::git_repo_state::GitRepoState;

#[test]
fn test_git_repo_state_default() {
    let state = GitRepoState::default();
    assert!(state.branch.is_none());
    assert_eq!(state.ahead, 0);
    assert_eq!(state.behind, 0);
    assert_eq!(state.staged, 0);
    assert_eq!(state.unstaged, 0);
    assert_eq!(state.untracked, 0);
    assert_eq!(state.stashed, 0);
}

#[test]
fn test_git_repo_state_custom() {
    let state = GitRepoState {
        branch: Some("main".to_string()),
        ahead: 2,
        behind: 1,
        staged: 3,
        unstaged: 4,
        untracked: 5,
        stashed: 6,
    };
    assert_eq!(state.branch.as_deref(), Some("main"));
    assert_eq!(state.ahead, 2);
    assert_eq!(state.behind, 1);
    assert_eq!(state.staged, 3);
    assert_eq!(state.unstaged, 4);
    assert_eq!(state.untracked, 5);
    assert_eq!(state.stashed, 6);
}
