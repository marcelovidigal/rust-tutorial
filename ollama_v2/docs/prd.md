# Product Requirements Document

The original request was a one-line description ("chat UI for Ollama with ratatui"), so these requirements are derived from the code as it stands.

## Goal

Let a developer chat with a local Ollama model from the terminal, with replies appearing as they are generated.

## Users

Developers who run Ollama locally and prefer a keyboard-driven interface.

## Functional requirements

| ID | Requirement | Status |
|---|---|---|
| F1 | List installed models and let the user pick one | Done (Ctrl+O) |
| F2 | Send a message and show the reply streaming in | Done |
| F3 | Keep conversation context across turns | Done (in memory) |
| F4 | Cancel a reply in progress | Done (Esc) |
| F5 | Scroll the conversation | Done (↑ ↓) |
| F6 | Multi-line prompt that wraps and grows | Done (up to 8 rows) |
| F7 | Show status (loading, generating, ready, errors) | Done |
| F8 | Explain why models could not load (for example, Ollama not running) | Done |
| F9 | Cancel or error leaves no empty reply in the history | Done |
| F10 | Model cannot be changed while a reply is generating | Done |

## Non-functional requirements

- Interface text in English.
- UI stays responsive while a reply streams (async tasks).
- Terminal is always restored on exit.
- No configuration needed: defaults to `localhost:11434`.

## Out of scope (today)

Saving conversations, system prompts, model options (temperature, context size), markdown rendering, remote Ollama hosts, cursor movement inside the prompt, mouse support, tools and agent features. The larger items are detailed under "Future requirements".

## Candidate next steps

1. Cursor editing and paste support in the prompt.
2. Remember the last used model; support `OLLAMA_HOST`.
3. Trim history to the model's context window.
4. Unit tests for `App`.
5. Include the error detail for stream failures (currently only `stream error`).

## Future requirements

Longer-term goal: grow from a plain chat into an agentic terminal assistant with the same kind of features as Claude Code, and inspired by its interface and by [OpenAI Codex CLI](https://github.com/openai/codex). Everything here is planned, not built.

### Conversation storage

| ID | Requirement |
|---|---|
| S1 | Save every conversation automatically (one file per session, for example JSON Lines under the user's config directory) |
| S2 | List past sessions with title, model and date; open one to read it |
| S3 | Resume a session and continue the conversation (`--resume`, `/resume`) |
| S4 | Rename, delete and export sessions (Markdown) |
| S5 | Persist the prompt history so ↑ recalls earlier inputs across runs |
| S6 | Store settings (last model, host, system prompt) in a config file |

### Claude Code–style chat

| ID | Requirement |
|---|---|
| C1 | Slash commands: `/help`, `/clear`, `/model`, `/resume`, `/compact`, `/save`, `/copy`, `/quit` with autocomplete |
| C2 | Markdown rendering: headings, lists, bold, inline code, fenced code blocks with syntax highlighting |
| C3 | Full prompt editor: cursor movement, multi-line input (Shift+Enter), paste, input history |
| C4 | Project instructions file (like `CLAUDE.md` / `AGENTS.md`) loaded into the system prompt |
| C5 | File references with `@path` that attach file contents to the message |
| C6 | Tools the model can call (read, search, edit files, run commands) via Ollama tool calling |
| C7 | Approval prompts before running a command or editing a file, with modes: ask every time, auto-approve edits, read-only |
| C8 | Diff view of proposed file changes before applying them |
| C9 | Visible "thinking" and tool-call blocks, collapsible, with a spinner while working |
| C10 | Context management: token usage display, automatic trimming, and `/compact` to summarize old turns |
| C11 | Interrupt (Esc) at any point, including during a tool run, and undo of the last file changes |
| C12 | Non-interactive mode (`-p "prompt"`) for scripts, printing the answer to stdout |
| C13 | MCP support so external tool servers can be plugged in |
| C14 | Configurable sandbox: restrict file access to the project directory and limit network use for commands |
| C15 | Status line with model, directory, git branch and token count; theme and key bindings configurable |

A feature-by-feature analysis of the Codex CLI source, with effort and priority, is in [codex-features.md](codex-features.md).

### Suggested order

1. S1, S2, S3 (storage and resume), then C1 and C3 (commands and prompt editor).
2. C2 and C10 (readable output and context control).
3. C4, C5, then tools and approvals (C6, C7, C8, C11, C14).
4. C12, C13, C15 and polish.

## Success criteria

A user can start the app, pick a model, hold a multi-turn conversation, cancel a long reply, and quit, without the terminal being left in a broken state.
