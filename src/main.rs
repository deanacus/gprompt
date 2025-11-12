mod models;
mod services;
use ansi_term::Colour::Blue;
use ansi_term::Colour::Purple;
use dirs::home_dir;

use crate::services::git_status::get_git_repo_state;

const SYMBOL_PROMPT: &str = "❯";
const COLOR_PROMPT: ansi_term::Colour = Purple;

use std::path::Path;

fn get_path(cwd: &Path) -> String {
    let home_path = match home_dir() {
        Some(p) => p,
        None => return cwd.display().to_string(),
    };
    let home_str = home_path.to_str().unwrap_or("");
    let cwd_str = cwd.display().to_string();
    if cwd_str.starts_with(home_str) {
        cwd_str.replacen(home_str, "~", 1)
    } else {
        cwd_str
    }
}


fn main() {
    use crate::models::prompt::GitStatusSymbol;
    let path = match std::env::current_dir() {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Failed to get current directory");
            return;
        }
    };

    let path_segment = get_path(&path);
    let git_state = get_git_repo_state(&path);

    println!();
    print!("{} ", Blue.paint(path_segment));
    if let Some(state) = git_state {
        if let Some(branch) = state.branch {
            print!("{} ", GitStatusSymbol::Branch.color().dimmed().paint(branch));
        }
        if state.ahead > 0 {
            print!("{}", GitStatusSymbol::Ahead.color().paint(GitStatusSymbol::Ahead.symbol()));
        }
        if state.behind > 0 {
            print!("{}", GitStatusSymbol::Behind.color().paint(GitStatusSymbol::Behind.symbol()));
        }
        if state.unstaged > 0 {
            print!("{}", GitStatusSymbol::Unstaged.color().paint(GitStatusSymbol::Unstaged.symbol()));
        }
        if state.staged > 0 {
            print!("{}", GitStatusSymbol::Staged.color().paint(GitStatusSymbol::Staged.symbol()));
        }
        if state.stashed > 0 {
            print!("{}", GitStatusSymbol::Stashed.color().paint(GitStatusSymbol::Stashed.symbol()));
        }
        if state.untracked > 0 {
            print!("{}", GitStatusSymbol::Untracked.color().paint(GitStatusSymbol::Untracked.symbol()));
        }
    }
    println!();
    print!("{} ", COLOR_PROMPT.paint(SYMBOL_PROMPT));
}
