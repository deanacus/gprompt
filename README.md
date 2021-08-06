# Gprompt

## What is it?

My shell prompt, written in Rust, because I wanted to play with rust.

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

Not many. Some basic git stuff, that's about it. Everything is hardcoded,
because I'm not interested in making it configurable. If you want to add that,
feel free to send a PR my way.
