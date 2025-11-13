# Gprompt

## What is it?

My shell prompt, written in Rust, because I wanted to play with Rust.

## Usage

Clone the repo, run `cargo build --release` and copy the binary to somewhere on
your path, then configure your shell as follows:

### Bash

Add this to your `~/.bashrc`:

```bash
PROMPT_COMMAND='PS1="$(gprompt)"'
```

### Zsh

Add this to your `~/.zshrc`:

```zsh
autoload -Uz add-zsh-hook
_prompt() {
  PROMPT="$(gprompt)"
}
add-zsh-hook precmd _prompt
```

### Fish

Add this to your `~/.config/fish/config.fish`:

```fish
function fish_prompt
    gprompt
end
```

I might get around to doing a "release" sometime soon.

## Features

Not many. Some basic git stuff, that's about it. Everything is hardcoded,
because I'm not interested in making it configurable. If you want to add that,
feel free to send a PR my way.
