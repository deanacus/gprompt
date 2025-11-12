use ansi_term::Colour;

pub enum GitStatusSymbol {
    Branch,
    Ahead,
    Behind,
    Unstaged,
    Staged,
    Stashed,
    Untracked,
}

impl GitStatusSymbol {
    pub fn symbol(&self) -> &'static str {
        match self {
            GitStatusSymbol::Branch => "",
            GitStatusSymbol::Ahead => "↑",
            GitStatusSymbol::Behind => "↓",
            GitStatusSymbol::Unstaged => "×",
            GitStatusSymbol::Staged => "+",
            GitStatusSymbol::Stashed => "•",
            GitStatusSymbol::Untracked => "*",
        }
    }

    pub fn color(&self) -> Colour {
        match self {
            GitStatusSymbol::Branch => Colour::White,
            GitStatusSymbol::Ahead => Colour::Cyan,
            GitStatusSymbol::Behind => Colour::Cyan,
            GitStatusSymbol::Unstaged => Colour::Red,
            GitStatusSymbol::Staged => Colour::Cyan,
            GitStatusSymbol::Stashed => Colour::Yellow,
            GitStatusSymbol::Untracked => Colour::Yellow,
        }
    }
}
