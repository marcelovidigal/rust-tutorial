# Tutorial: how Ollama Chat works

A guided tour of the code, one idea at a time. Read it next to `src/`.

## 1. The big idea: state, events, drawing

Every interactive terminal app does the same loop:

```
wait for an event → update the state → draw the state
```

Here the state is `App` ([app.rs](../src/app.rs)), events are `Msg` ([main.rs](../src/main.rs)), and drawing is `ui::draw` ([ui.rs](../src/ui.rs)). Keeping these three apart makes each one easy to understand.

## 2. State: `App`

`App` is a plain struct with no network or terminal code. Important fields:

- `history`: the conversation, a list of `ChatMessage`.
- `input`: the text being typed.
- `models` / `model`: installed models and the selected index.
- `picker`: `Some(row)` when the model pop-up is open, `None` otherwise.
- `busy`: true while a reply streams.
- `scroll`: how many lines the user scrolled up from the bottom (0 follows the end).

Because `App` has no I/O, you can reason about it (and test it) on its own.

## 3. Sending a message: `submit`

`submit` does three things in order:

1. Moves `input` into `history` as a user message.
2. Clones `history` as the request. It is cloned **before** the next step, so the model never sees the empty reply.
3. Pushes an empty assistant message and sets `busy`.

That empty assistant message is the trick behind streaming: each token is appended to it with `push_token`, and the UI simply draws the last message as it grows.

## 4. Async tasks and the channel

Waiting on the keyboard, the network and the screen at once would block. Instead, `run` in main.rs spawns tasks that each send `Msg` values into one channel:

- **events task**: terminal keys and resizes → `Msg::Term`
- **models task**: asks Ollama for models once → `Msg::Models`
- **chat task** (`spawn_chat`): streams tokens → `Msg::Token`, then `Msg::Done`

The main loop does `rx.recv().await` and handles one message at a time. Since only this loop touches `App`, no locks are needed. This pattern is often called "message passing" or the actor style.

## 5. Handling keys

Inside the loop, a single `match (key, ctrl)` decides what a key does. Order matters:

1. Global shortcuts (Ctrl+C, Ctrl+O).
2. If the picker is open, it captures ↑ ↓ Enter Esc.
3. Otherwise chat keys: Enter sends, Esc cancels, ↑ ↓ scroll, characters edit the input.

Moving the picker case above the chat keys is what stops Enter from sending a message while you are choosing a model.

## 6. Cancelling

`spawn_chat` returns a `JoinHandle`, kept in `task`. Pressing Esc while `busy` calls `handle.abort()`, which drops the HTTP stream. Then `app.finish(Some("cancelled"))` resets the state and removes the assistant message if it is still empty. Tokens already in the channel may still arrive afterwards, so `push_token` ignores them when the app is not `busy`.

The same guard idea applies to the model picker: Ctrl+O does nothing while `busy`, so the model cannot change mid-reply.

## 6b. Errors

Failures travel as messages too. If listing models fails, the task sends `Msg::Models(Err(text))` and `App::models_error` puts `cannot reach Ollama: ...` in the footer. Stream failures arrive in `Msg::Done(Some(text))` and `finish` shows them the same way.

## 7. Drawing: layout

`draw` splits the screen vertically into four areas with `Layout::vertical`:

| Area | Size |
|---|---|
| header | fixed 3 rows |
| chat | whatever is left (`Min`) |
| prompt | grows with the text, 3 to 10 rows |
| footer | 1 row |

Each area is then handed to a function (`header`, `messages`, `prompt`) that renders widgets into it. The model picker is drawn last so it sits on top of everything.

## 8. Drawing: the chat and scrolling

`messages` turns each message into `Line`s: the first line gets a tag (`>` for you, `●` for the model), later lines are indented. Long lines are wrapped by `Paragraph` with `Wrap`.

To follow the end of the conversation, the code estimates the wrapped height (`total`), computes the maximum scroll (`total - visible rows`), and subtracts the user's `scroll` offset. The first visible row (`top`) is passed to `Paragraph::scroll`.

## 9. Drawing: the prompt box

`prompt` wraps `"> " + input` by hand with `wrap_ranges`, which returns one `(start, end)` range per row. A row breaks after its last space, so words stay whole; a word longer than the row is split. One column is kept free so the cursor always fits.

Wrapping by hand (instead of using `Wrap`) is deliberate: the cursor is at the end of the text, so its position is simply the end of the last range. `input_rows` uses the same function, so the box height and the drawn rows always agree. If the text is taller than the box, only the last rows are shown. `wrap_ranges` has unit tests (`cargo test`).

## 10. Terminal safety

`main` calls `ratatui::init()` before the loop and `ratatui::restore()` after it, even if the loop returned an error. Without the restore, the shell would be left in raw mode with no echo.

## 11. Exercises

1. Add ← and → to move a cursor inside `input`.
2. Add a `/clear` command that empties `history`.
3. Write unit tests for `App::submit`, `finish` and `move_picker`.
4. Include the real error detail in `stream error`.
