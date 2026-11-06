# Architecture

## Overview

Ollama Chat is a single-process async terminal app. One loop owns all state; background tasks only send messages to it.

```
 terminal ──► [events task] ──┐
                              ▼
 Ollama ◄──► [models task] ─► mpsc channel ─► main loop ─► App (state) ─► ui::draw
 Ollama ◄──► [chat task]   ──┘                    ▲
                                  Esc aborts ─────┘
```

## Modules

| File | Responsibility |
|---|---|
| `src/main.rs` | Starts the terminal, spawns tasks, runs the loop, maps keys to state changes |
| `src/app.rs` | `App` struct and its operations. No I/O, no rendering |
| `src/ui.rs` | `draw(frame, &App)`. Read-only; builds widgets from state |

## Data flow

1. A task reads terminal events and sends `Msg::Term`.
2. A task lists installed models and sends `Msg::Models`.
3. On Enter, `App::submit` adds the user message and an empty assistant message, and returns the request. `spawn_chat` streams the reply as `Msg::Token`, then `Msg::Done`.
4. The loop applies each message to `App`, then redraws.

## Key decisions

- **Single owner of state.** Only the main loop mutates `App`, so there are no locks.
- **Unbounded channel.** Producers (keys, tokens) are slow compared with the loop, so back-pressure is not needed.
- **Streaming into history.** The assistant message exists from the start and grows with each token, so rendering needs no special case for "in progress".
- **Cancellation by abort.** Esc calls `JoinHandle::abort` on the chat task. A partial reply stays in the history; an empty one is removed by `App::finish`.
- **Late tokens are dropped.** `push_token` ignores tokens when the app is not `busy`, so tokens still in the channel after a cancel do no harm.
- **Errors reach the status line.** Failures listing models (`Msg::Models(Err)`) or streaming (`Msg::Done(Some(err))`) are shown in the footer.
- **No model switch mid-reply.** Ctrl+O is ignored while `busy`, so the header always shows the model that is answering.
- **UI as a pure function of state.** `draw` never changes `App`, which keeps rendering predictable.

## Dependencies

| Crate | Use |
|---|---|
| `ratatui` | Widgets and layout |
| `crossterm` | Raw terminal input (`EventStream` for async) |
| `ollama-rs` | Ollama HTTP client with streaming |
| `tokio` | Async runtime, tasks, channel |
| `futures` | `StreamExt` for event and token streams |

## Known weaknesses

- Errors are shown only as a short footer message; they are not kept in the conversation. Stream errors carry no detail (`stream error`).
- After a cancel or error, the user's message stays in the history without an answer, so two user messages can be sent in a row on the next turn.
- Chat scroll height is estimated from character width, so it can drift with wide Unicode characters.
- The whole history is resent every turn and is not trimmed to the model's context window.
