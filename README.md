# Gprompt

## What is it?

My shell prompt, written in Rust, because I wanted to play with rust.

### Project Structure (Idiomatic Rust)
- `src/models/`: Data structures (e.g., `GitRepoState`, prompt enums)
- `src/services/`: Business logic (e.g., git status computation)
- `src/main.rs`: Presentation layer (prompt rendering, CLI entrypoint)

All code is idiomatic, modular, and safe. See `specs/001-idiomatic-safe-rust/` for the full specification and rationale.

## Requirements

- Rust 1.70.0 or later (MSRV)

## Usage

Clone the repo, run `cargo build --release` and copy the binary to somewhere on
your path, then in your shell config, add this (ZSH):

```
autoload -Uz add-zsh-hook
_prompt() {
  PROMPT="$(gprompt)"
}
add-zsh-hook precmd _prompt
```

I might get around to doing a "release" sometime soon.

## Features

- Idiomatic, modular Rust codebase: core logic is split into `models/` (data structures) and `services/` (logic/services).
- All prompt symbols and colors are defined as enums/constants for clarity and extensibility.
- Safe error handling throughout: no panics or unwraps, all errors are handled gracefully.
- Uses `Path`/`PathBuf` for all filesystem operations, never raw strings.
- Easily extensible: to add new git states or prompt features, just add to the enums and update the rendering logic.

Not many. Some basic git stuff, that's about it. Everything is hardcoded,
because I'm not interested in making it configurable. If you want to add that,
feel free to send a PR my way.
