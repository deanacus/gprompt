use gprompt::models::prompt::GitStatusSymbol;
use ansi_term::Colour;

#[test]
fn test_git_status_symbol_symbols() {
    assert_eq!(GitStatusSymbol::Ahead.symbol(), "↑");
    assert_eq!(GitStatusSymbol::Behind.symbol(), "↓");
    assert_eq!(GitStatusSymbol::Unstaged.symbol(), "×");
    assert_eq!(GitStatusSymbol::Staged.symbol(), "+");
    assert_eq!(GitStatusSymbol::Stashed.symbol(), "•");
    assert_eq!(GitStatusSymbol::Untracked.symbol(), "*");
}

#[test]
fn test_git_status_symbol_colors() {
    assert_eq!(GitStatusSymbol::Ahead.color(), Colour::Cyan);
    assert_eq!(GitStatusSymbol::Behind.color(), Colour::Cyan);
    assert_eq!(GitStatusSymbol::Unstaged.color(), Colour::Red);
    assert_eq!(GitStatusSymbol::Staged.color(), Colour::Cyan);
    assert_eq!(GitStatusSymbol::Stashed.color(), Colour::Yellow);
    assert_eq!(GitStatusSymbol::Untracked.color(), Colour::Yellow);
}
