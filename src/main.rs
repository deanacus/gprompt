mod models;
mod services;
use ansi_term::Colour;
use dirs::home_dir;

use crate::services::git_status::get_git_repo_state;

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
    print!("{} ", Colour::Blue.paint(path_segment));
    if let Some(state) = git_state {
        if let Some(branch) = state.branch {
            print!("{} ", Colour::White.dimmed().paint(branch));
        }
        if state.ahead > 0 {
            print!("{}", Colour::Cyan.paint("↑"));
        }
        if state.behind > 0 {
            print!("{}", Colour::Cyan.paint("↓"));
        }
        if state.unstaged > 0 {
            print!("{}", Colour::Red.paint("×"));
        }
        if state.staged > 0 {
            print!("{}", Colour::Cyan.paint("+"));
        }
        if state.stashed > 0 {
            print!("{}", Colour::Yellow.paint("•"));
        }
        if state.untracked > 0 {
            print!("{}", Colour::Yellow.paint("*"));
        }
    }
    println!();
    print!("{} ", Colour::Purple.paint("❯"));
}
