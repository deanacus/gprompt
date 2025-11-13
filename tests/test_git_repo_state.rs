use gprompt::models::git_repo_state::GitRepoState;
use gprompt::models::git_special_state::{GitSpecialState, OperationProgress};

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
        special_state: GitSpecialState::Normal,
    };
    assert_eq!(state.branch.as_deref(), Some("main"));
    assert_eq!(state.ahead, 2);
    assert_eq!(state.behind, 1);
    assert_eq!(state.staged, 3);
    assert_eq!(state.unstaged, 4);
    assert_eq!(state.untracked, 5);
    assert_eq!(state.stashed, 6);
}

#[test]
fn test_operation_progress_validation() {
    // Valid cases
    assert!(OperationProgress::new(1, 1).is_some());
    assert!(OperationProgress::new(3, 7).is_some());
    assert!(OperationProgress::new(10, 10).is_some());

    // Invalid: current = 0
    assert!(OperationProgress::new(0, 5).is_none());

    // Invalid: total = 0
    assert!(OperationProgress::new(1, 0).is_none());

    // Invalid: current > total
    assert!(OperationProgress::new(8, 7).is_none());
    assert!(OperationProgress::new(100, 1).is_none());
}

#[test]
fn test_rebasing_display_name_with_progress() {
    let progress = OperationProgress::new(3, 7).unwrap();
    let state = GitSpecialState::Rebasing(Some(progress));
    assert_eq!(state.display_name(), Some("Rebasing 3/7".to_string()));
}

#[test]
fn test_rebasing_display_name_without_progress() {
    let state = GitSpecialState::Rebasing(None);
    assert_eq!(state.display_name(), Some("Rebasing".to_string()));
}

#[test]
fn test_cherry_picking_display_name_with_progress() {
    let progress = OperationProgress::new(2, 4).unwrap();
    let state = GitSpecialState::CherryPicking(Some(progress));
    assert_eq!(state.display_name(), Some("Cherry-picking 2/4".to_string()));
}

#[test]
fn test_cherry_picking_display_name_without_progress() {
    let state = GitSpecialState::CherryPicking(None);
    assert_eq!(state.display_name(), Some("Cherry-picking".to_string()));
}

#[test]
fn test_detached_display_name_with_sha() {
    let state = GitSpecialState::Detached("a1b2c3d".to_string());
    assert_eq!(state.display_name(), Some("Detached @ a1b2c3d".to_string()));
}

#[test]
fn test_detached_sha_length() {
    // Valid 7-character SHA
    let state = GitSpecialState::Detached("a1b2c3d".to_string());
    assert_eq!(state.display_name(), Some("Detached @ a1b2c3d".to_string()));

    // Longer SHAs should also work (we'll truncate to 7 in the detector)
    let state = GitSpecialState::Detached("a1b2c3d4e5f6789".to_string());
    assert_eq!(
        state.display_name(),
        Some("Detached @ a1b2c3d4e5f6789".to_string())
    );

    // "unknown" fallback for missing HEAD
    let state = GitSpecialState::Detached("unknown".to_string());
    assert_eq!(state.display_name(), Some("Detached @ unknown".to_string()));
}

#[test]
fn test_merging_display_name() {
    let state = GitSpecialState::Merging;
    assert_eq!(state.display_name(), Some("Merging".to_string()));
}

#[test]
fn test_reverting_display_name() {
    let state = GitSpecialState::Reverting;
    assert_eq!(state.display_name(), Some("Reverting".to_string()));
}

#[test]
fn test_bisecting_display_name() {
    let state = GitSpecialState::Bisecting;
    assert_eq!(state.display_name(), Some("Bisecting".to_string()));
}

#[test]
fn test_applying_patches_display_name() {
    let state = GitSpecialState::ApplyingPatches;
    assert_eq!(state.display_name(), Some("Applying patches".to_string()));
}
