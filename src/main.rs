use ansi_term::Colour::{Blue, Cyan, Purple, Red, White, Yellow};
use dirs::home_dir;
use git2::{self, Repository, Status, StatusOptions};

const SYMBOL_PROMPT: &str = "❯";
const SYMBOL_UNTRACKED: &str = "*";
const SYMBOL_DIRTY: &str = "×";
const SYMBOL_STAGED: &str = "+";
const SYMBOL_STASHED: &str = "•";
const SYMBOL_AHEAD: &str = "↑";
const SYMBOL_BEHIND: &str = "↓";

const COLOR_PROMPT: ansi_term::Colour = Purple;
const COLOR_BRANCH: ansi_term::Colour = White;
const COLOR_UNTRACKED: ansi_term::Colour = Yellow;
const COLOR_DIRTY: ansi_term::Colour = Red;
const COLOR_STAGED: ansi_term::Colour = Cyan;
const COLOR_STASHED: ansi_term::Colour = Yellow;
const COLOR_AHEAD: ansi_term::Colour = Cyan;
const COLOR_BEHIND: ansi_term::Colour = Cyan;

fn get_path(cwd: &str) -> String {
    let home_path = home_dir().unwrap();
    let display_path = match home_path.to_str() {
        Some(home) => cwd.replacen(home, "~", 1),
        None => cwd.to_owned(),
    };

    return display_path;
}

fn branch_name(repository: &Repository) -> String {
    let default = String::from("");
    let head = match repository.head() {
        Ok(head) => head,
        Err(_e) => return default,
    };

    if let Some(shorthand) = head.shorthand() {
        return COLOR_BRANCH
            .dimmed()
            .paint(shorthand.to_string())
            .to_string();
    } else {
        return default;
    }
}

fn get_staged(repository: &Repository) -> String {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);

    let statuses = repository.statuses(Some(&mut opts)).unwrap();
    let count = statuses
        .iter()
        .filter(|entry| {
            entry.status().intersects(
                Status::INDEX_NEW
                    | Status::INDEX_MODIFIED
                    | Status::INDEX_DELETED
                    | Status::INDEX_TYPECHANGE
                    | Status::INDEX_RENAMED,
            )
        })
        .count();

    if count > 0 {
        return COLOR_STAGED.paint(SYMBOL_STAGED).to_string();
    }

    return String::from("");
}

fn get_unstaged(repository: &Repository) -> String {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);

    let statuses = repository.statuses(Some(&mut opts)).unwrap();
    let count = statuses
        .iter()
        .filter(|entry| {
            entry.status().intersects(
                Status::WT_NEW
                    | Status::WT_MODIFIED
                    | Status::WT_DELETED
                    | Status::WT_TYPECHANGE
                    | Status::WT_RENAMED,
            )
        })
        .count();

    if count > 0 {
        return COLOR_DIRTY.paint(SYMBOL_DIRTY).to_string();
    }
    return String::from("");
}

fn get_untracked(repository: &Repository) -> String {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);

    let statuses = repository.statuses(Some(&mut opts)).unwrap();
    let count = statuses
        .iter()
        .filter(|entry| entry.status().intersects(Status::WT_NEW))
        .count();

    if count > 0 {
        return COLOR_UNTRACKED.paint(SYMBOL_UNTRACKED).to_string();
    }
    return String::from("");
}

fn get_stash(repo: &mut Repository) -> String {
    let default = String::from("");

    let mut count = 0;

    repo.stash_foreach(|_stash, _stashlabel, _stashid| {
        count += 1;
        return true;
    })
    .unwrap();

    if count > 0 {
        return COLOR_STASHED.paint(SYMBOL_STASHED).to_string();
    }
    return default;
}

fn get_ahead_behind(repo: &Repository) -> String {
    let default = String::from("");
    let mut out = vec![];

    let head = match repo.head() {
        Ok(repo) => repo,
        Err(_e) => return String::from(""),
    };

    let head_name = head.shorthand().unwrap_or("");
    let head_branch = repo
        .find_branch(head_name, git2::BranchType::Local)
        .unwrap();
    let upstream = match head_branch.upstream() {
        Ok(up) => up,
        Err(_e) => return default,
    };
    let head_oid = head.target().unwrap();
    let upstream_oid = upstream.get().target().unwrap();

    let (ahead, behind) = repo.graph_ahead_behind(head_oid, upstream_oid).unwrap();

    if ahead > 0 {
        out.push(COLOR_AHEAD.paint(SYMBOL_AHEAD).to_string());
    }
    if behind > 0 {
        out.push(COLOR_BEHIND.paint(SYMBOL_BEHIND).to_string());
    }

    return out.join("");
}

fn git_prompt(cwd: &str) -> String {
    let mut repo = match Repository::discover(cwd) {
        Ok(repo) => repo,
        Err(_e) => return String::from(""),
    };

    let mut out = vec![];

    if !repo.is_bare() {
        out.push(branch_name(&repo));
        out.push(" ".to_string());
        out.push(get_ahead_behind(&repo));
        out.push(get_unstaged(&repo));
        out.push(get_staged(&repo));
        out.push(get_stash(&mut repo));
        out.push(get_untracked(&repo));
    }

    return out.join("");
}

fn main() {
    let path = std::env::current_dir().unwrap();
    let cwd = path.to_str().unwrap_or("");

    let path_segment = get_path(&cwd);
    let git_segment = git_prompt(&cwd);

    println!();
    println!("{} {}", Blue.paint(path_segment), git_segment);
    print!("{} ", COLOR_PROMPT.paint(SYMBOL_PROMPT))
}
