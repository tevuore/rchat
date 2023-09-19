rchat
=====

Rust based implementation for command AI chat application.

At the moment this is app used is to learn Rust. Code is certainly not
most beautiful and contains lots of todos. But there is a vision behind all.

MVP under work in develop branch

- send prompt from cli to ChatGPT
- ask prompts by reading stdin interactively
- pipe text to prompt

## Usage

ChatGPT API key is read by default from `~/.chatgpt` file. See `config/settings.toml` for file syntax.

```bash
$ cargo run -- -help

$ cargo run -- "How are you?"

# Include debug prints for troubleshooting
$ cargo run -- -d "How are you?"

# Read prompt from stdin
$ cargo run
```