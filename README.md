rchat
=====

Rust based implementation for command AI chat application.

At the moment this is app used is to learn Rust. Code is certainly not
most beautiful and contains lots of todos. But there is a vision behind all.

MVP 1 is ready and next MVP is under work in `develop` branch

## MVP 1

Implemented  
✅ Send prompt from cli to ChatGPT  
✅ Ask prompt by reading stdin interactively  
✅ Pipe text to prompt  
✅ Format response using markdown formatter

## MVP 2

Next MVP 2 is planned to contain

- support for custom instructions as first prompt in chat
- read stdin loop for continuous chat
- start writing developer tests
- experiment with a coverage tool

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