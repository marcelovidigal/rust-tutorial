# Codex CLI: features worth implementing

Analysis of the OpenAI Codex CLI source (`temp/codex`, Rust workspace under `codex-rs/`) to pick features for Ollama Chat. Each item says what Codex does, why it matters here, and how big the work is. Related requirement IDs are in [prd.md](prd.md).

## How this was done

- The repo's `docs/*.md` files are one-line pointers to online docs, so the source was the reference.
- Read: README, the `/` command list (`tui/src/slash_command.rs`), CLI flags (`cli/src/main.rs`, `exec/src/cli.rs`), approval and sandbox enums (`protocol/`), `AGENTS.md` loading (`core/src/agents_md.rs`), session storage (`rollout/`, `message-history/`), exec policy (`execpolicy/`), hooks, skills, the Ollama crate, and doc comments of the composer, diff, backtrack, resume and status-line modules.
- Not read line by line: the workspace has about 100 crates (cloud tasks, voice, Windows sandbox, analytics, app server, plugins...). Features there are listed only if they showed up in the command list.
- Codex is Apache-2.0. Use it as a reference for design; write the code yourself.

## Priority summary

| # | Feature | Value | Effort | PRD |
|---|---|---|---|---|
| 1 | Session storage as JSONL + resume picker | High | Medium | S1 to S3 |
| 2 | Persistent prompt history with Ctrl+R search | High | Low | S5 |
| 3 | Slash commands with popup | High | Medium | C1 |
| 4 | Real prompt editor (cursor, paste, external editor) | High | Medium | C3 |
| 5 | `AGENTS.md` project instructions | High | Low | C4 |
| 6 | `@` file mentions with fuzzy popup | High | Medium | C5 |
| 7 | `/compact` and token usage | High | Medium | C10 |
| 8 | Non-interactive `exec` mode | Medium | Low | C12 |
| 9 | Markdown + syntax-highlighted diffs | Medium | Medium | C2, C8 |
| 10 | Approval modes and sandbox | High | High | C6, C7, C14 |
| 11 | Exec policy rules | Medium | Medium | C7 |
| 12 | Backtrack (edit an earlier prompt) | Medium | Medium | new |
| 13 | Configurable status line | Medium | Low | C15 |
| 14 | `/fork`, `/rename`, `/archive`, `/export` | Medium | Low | S4 |
| 15 | Skills | Medium | Medium | new |
| 16 | Hooks | Low | Medium | new |
| 17 | Review mode | Medium | Medium | new |
| 18 | Goal / Plan mode | Low | Medium | new |
| 19 | Profiles and layered config | Medium | Low | S6 |
| 20 | MCP client | Medium | High | C13 |

## Storage and sessions

### 1. Session storage as JSONL + resume picker
- **Codex:** every session is written as an append-only `.jsonl` "rollout" file (`rollout/src/recorder.rs`), with an index and metadata (cwd, git info, name). `codex resume` shows a picker with search, pagination and a lazy transcript preview (Ctrl+E expands). Flags: `resume [SESSION_ID] --last --all`.
- **Here:** write one `.jsonl` file per chat (one line per message with role, text, timestamp, model). A `/resume` list sorted by date, filtered by directory, with the first user message as preview.
- **Why JSONL:** appending a line per message is crash-safe and needs no database.

### 2. Persistent prompt history with Ctrl+R search
- **Codex:** `message-history` keeps a global `~/.codex/history.jsonl` (`{session_id, ts, text}` per line), written with a single append so concurrent processes do not interleave. The composer recalls it with ↑/↓ and has shell-style reverse search on Ctrl+R.
- **Here:** same file format in the config directory; ↑/↓ recall, Ctrl+R incremental search. Small and very noticeable.

### 14. Session management commands
- **Codex:** `/new`, `/clear`, `/fork` (branch the current chat), `/rename`, `/archive`, `/delete`, `/export` (conversation as Markdown), `/copy` (last answer).
- **Here:** all are cheap once sessions are files. `/export` and `/copy` (via the OSC 52 terminal sequence, no dependency) are the quickest wins.

## Prompt and input

### 3. Slash commands with popup
- **Codex:** typing `/` opens a filtered popup with a description per command (`bottom_pane/command_popup.rs`). Commands worth copying: `/model`, `/new`, `/clear`, `/compact`, `/resume`, `/copy`, `/export`, `/status`, `/diff`, `/init`, `/mention`, `/review`, `/quit`.
- **Here:** a `SlashCommand` enum with `description()` and a parser; the popup lists matches as you type. Start with `/model`, `/clear`, `/save`, `/resume`, `/copy`, `/export`, `/status`, `/help`, `/quit`.

### 4. Real prompt editor
- **Codex:** full textarea (cursor, selection, word movement), multi-line input, `/vim` mode, and Ctrl+G style hand-off to `$VISUAL`/`$EDITOR` for long prompts (`external_editor.rs`).
- **Paste-burst detection:** on terminals without bracketed paste (Windows especially), a paste arrives as a fast stream of key events and Enter would submit the message. Codex detects the burst and treats Enter as a newline inside the paste (`paste_burst.rs`). This applies directly to this project, which runs on Windows.
- **Here:** cursor index in `App`, ←/→/Home/End/Delete, Ctrl+W, Shift+Enter or Ctrl+J for newline, bracketed paste, and a simple burst timer as fallback.

### 6. `@` file mentions with fuzzy popup
- **Codex:** typing `@` opens a fuzzy file search over the project (`file-search/`, `file_search_popup.rs`); the chosen path is inserted and the file is attached to the turn. `/mention` does the same from the menu.
- **Here:** walk the directory (respect `.gitignore`), fuzzy-match names, insert `@path`, and prepend the file content to the message when sending.

## Context and instructions

### 5. `AGENTS.md` project instructions
- **Codex:** collects every `AGENTS.md` from the project root (found by walking up to `.git`) down to the current directory and concatenates them, root first. `AGENTS.override.md` is a local override; a global file adds user-level instructions. `/init` generates one.
- **Here:** the same walk, then send the text as a system message. About 50 lines of code, large effect on answer quality. `/init` can ask the model to draft the file.

### 7. `/compact` and token usage
- **Codex:** `/compact` asks the model to summarize the conversation and replaces old turns with the summary; auto-compaction runs near the context limit; `/status` shows token usage.
- **Here:** Ollama returns `prompt_eval_count` and `eval_count` in the final stream chunk. Show them in the footer, warn near `num_ctx`, and implement `/compact` as "summarize these turns" followed by a history replacement. Prevents the silent truncation that small local contexts suffer from.

## Output

### 9. Markdown rendering and diffs
- **Codex:** `markdown_render.rs` styles answers; `diff_render.rs` draws unified diffs with line numbers, `+`/`-` gutter, syntax highlighting, and colors that adapt to dark or light terminals. `/diff` shows `git diff` including untracked files.
- **Here:** start with headings, lists, bold, inline code and fenced blocks; add `syntect` highlighting later. A `/diff` command that runs `git diff` is trivial.

### 13. Configurable status line
- **Codex:** `/statusline` lets the user pick items: model, reasoning level, current dir, project, git branch, PR number, branch diff stats, run state, permissions, token usage. The terminal title is configurable too (`/title`).
- **Here:** a list of items stored in config, rendered in the footer. Model, directory, git branch and tokens cover most of the value.

## Agent features

### 10. Approval modes and sandbox
- **Codex:** two independent settings.
  - Approval policy: `untrusted` (ask unless a rule allows), `on-request` (the model decides when to ask; default), `granular` (per category), `never`.
  - Sandbox mode: `read-only` (default), `workspace-write`, `danger-full-access`. Real isolation uses OS mechanisms (Seatbelt, Landlock/bwrap, a Windows sandbox).
  - The approval prompt offers: approve once, approve for the session, approve and add a policy rule, deny with a message so the model can try something else.
- **Here:** needs Ollama tool calling first. Implement tools `read_file`, `list_dir`, `grep` (read-only, no approval) and `write_file`, `run_command` (approval required). Keep the approval prompt with once / for this session / deny. Restrict paths to the project root. A real OS sandbox is a later, large step; path checks and a command allow-list are a reasonable first version.

### 11. Exec policy rules
- **Codex:** `execpolicy` evaluates commands against Starlark `prefix_rule(pattern=[...], decision="allow|prompt|forbidden", justification=..., match=[...], not_match=[...])`. The `match`/`not_match` examples are checked when the file loads, like unit tests. `codex execpolicy check` tests a command against rules.
- **Here:** a small rules file (TOML is simpler than Starlark): command prefix, decision, reason. Keep the idea of self-testing examples.

### 8. Non-interactive mode
- **Codex:** `codex exec "prompt"` runs once and exits; stdin is read when the prompt is `-` or appended as a `<stdin>` block; `--json` prints events as JSONL; `--output-last-message FILE` saves the answer; `--output-schema FILE` forces a JSON Schema on the final answer; `--ephemeral` skips saving the session; `--color`.
- **Here:** `ollama_v2 -p "prompt"` printing the streamed answer, plus `--json` and `--ephemeral`. Makes the tool usable in scripts and pipes. Ollama supports a `format` field for structured output, so `--output-schema` is also feasible.

### 12. Backtrack
- **Codex:** pressing Esc twice on an empty composer opens the transcript; Esc/Enter pick an earlier user prompt, the conversation is rolled back to before that turn, and the prompt returns to the composer for editing.
- **Here:** with the history as a list, truncate at the chosen index and put its text back into the input. Fits the current structure well.

### 17. Review mode
- **Codex:** `codex review` and `/review` ask the model to review the working tree: `--uncommitted`, `--base <branch>`, `--commit <sha>`, optional custom instructions.
- **Here:** run `git diff` (or `git show`), build a fixed review prompt, send. Mostly prompt text and a git call.

### 15. Skills
- **Codex:** reusable instruction packs loaded from skill directories, invoked with `/skills` or by name mentions; a skill can bundle scripts.
- **Here:** a `skills/<name>/SKILL.md` convention; list them in `/skills`; inject the chosen file into the prompt.

### 16. Hooks
- **Codex:** lifecycle hooks around events: session start/end, user prompt submit, pre/post tool use, permission request, compaction, stop. `/hooks` lists and manages them.
- **Here:** low priority. Useful later for logging or blocking tools; needs the tool system first.

### 18. Goal and Plan mode
- **Codex:** `/goal <objective>` (also `pause`, `resume`, `clear`) tracks a long-running objective; `/plan` switches to a mode where the model proposes a plan before acting.
- **Here:** `/plan` can be a system-prompt switch ("propose steps, do not execute"). `/goal` only makes sense once there are autonomous multi-step runs.

### 20. MCP client
- **Codex:** connects to Model Context Protocol servers; `/mcp` lists tools and `/mcp login <name>` authenticates.
- **Here:** large, and only after local tools exist. Defer.

## Configuration

### 19. Profiles and layered config
- **Codex:** `config.toml` in `CODEX_HOME`, plus project-level overrides; `-p/--profile` selects named profiles; `--strict-config` rejects unknown keys; `-C/--cd` sets the working directory; `--enable/--disable <feature>` toggles experimental features.
- **Here:** a `config.toml` (model, host, system prompt, temperature, context size, status-line items), a `--profile` flag, and project overrides. Reading `OLLAMA_HOST` and a `--host` flag is the first step.

### Related: the Ollama integration (`ollama/`)
- Codex supports local models through Ollama (`--oss`, default model `gpt-oss:20b`). It checks the server is reachable, and **pulls the model if missing**, with a progress reporter (`pull.rs`).
- **Worth copying:** when a model passed with `-m` is not installed, offer to pull it and show download progress, using Ollama's `/api/pull` stream.

## Suggested roadmap

1. **Foundation (small, big payoff):** editor with cursor and paste (4), prompt history (2), slash command popup (3), `/copy` and `/export`.
2. **Persistence:** JSONL sessions, resume picker, fork and rename (1, 14), config file (19).
3. **Context:** `AGENTS.md` (5), token usage and `/compact` (7), `@` mentions (6), status line (13).
4. **Output:** Markdown and `/diff` (9), backtrack (12), non-interactive `-p` (8).
5. **Agent:** read-only tools, then write tools with approvals (10, 11), review mode (17).
6. **Later:** skills (15), hooks (16), plan mode (18), MCP (20).
