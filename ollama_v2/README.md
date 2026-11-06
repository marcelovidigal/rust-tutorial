# Ollama Chat

A small terminal chat client for [Ollama](https://ollama.com), written in Rust with [ratatui](https://ratatui.rs). Replies stream in token by token.

## Requirements

- Rust (edition 2024)
- Ollama running on `localhost:11434` with at least one model (`ollama pull <model>`)

## Run

```
cargo run
```

## Keys

| Key | Action |
|---|---|
| Enter | Send message |
| Esc | Cancel the reply being generated |
| Ctrl+O | Open the model picker (↑↓ to move, Enter to select, Esc to close). Disabled while a reply is generating |
| ↑ / ↓ | Scroll the conversation |
| Ctrl+C | Quit |

## Project layout

```
src/main.rs   event loop, background tasks, key handling
src/app.rs    application state (no I/O)
src/ui.rs     rendering
docs/         architecture, product requirements, tutorial
```

## Documentation

- [Architecture](docs/architecture.md)
- [Product requirements (PRD)](docs/prd.md)
- [Tutorial](docs/tutorial.md)
- [Codex CLI features worth implementing](docs/codex-features.md)

## Roadmap

Planned: saving and resuming chats, slash commands, markdown rendering, and Claude Code / Codex style agent features (tools, approvals, project instructions). See [Future requirements](docs/prd.md#future-requirements).

## Known limitations

The first installed model is selected on startup, history is not saved, the input has no cursor movement (append and backspace only), and the whole history is resent on every turn.
