// Terminal chat UI for Ollama, built with ratatui.
// Usage: `cargo run` (Ollama must be running on localhost:11434).
mod app;
mod ui;

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use ollama_rs::{Ollama, generation::chat::{ChatMessage, request::ChatMessageRequest}};
use tokio::{sync::mpsc, task::JoinHandle};

use app::App;

/// Everything the main loop can react to. Background tasks send these
/// through a channel, so the loop has a single place that mutates `App`.
enum Msg {
	Term(Event),
	Models(Result<Vec<String>, String>),
	Token(String),
	Done(Option<String>),
}

/*
// Original code: a simple example using `GenerationRequest`.
// Example Ollama client, before the ratatui interface existed.
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// connects to localhost:11434 by default
	let ollama = Ollama::default();

	//let model = "gpt-oss:20b".to_string();
	let model = "qwen3.5:9b".to_string();
	let prompt = "Why is the sky blue?".to_string();

	println!("ollama example");
	println!("model: {}", model);
	println!("\n{}", prompt);

	let res = ollama.generate(GenerationRequest::new(model, prompt)).await?;
	println!("\n{}", res.response);

	Ok(())
}
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// `init` puts the terminal in raw mode / alternate screen; `restore`
	// must run even when `run` fails, so the result is returned afterwards.
	let mut term = ratatui::init();
	let res = run(&mut term).await;
	ratatui::restore();
	res
}

/// The event loop: draw, wait for one message, update the state, repeat.
async fn run(term: &mut ratatui::DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
	let ollama = Ollama::default();
	let (tx, mut rx) = mpsc::unbounded_channel();

	// Task 1: forward terminal events (keys, resize...) to the loop.
	let t = tx.clone();

	tokio::spawn(async move {
		let mut events = EventStream::new();

		while let Some(Ok(e)) = events.next().await {
			if t.send(Msg::Term(e)).is_err() { break; }
		}
	});

	// Task 2: fetch the installed models once, without blocking the UI.
	let (t, o) = (tx.clone(), ollama.clone());

	tokio::spawn(async move {
		let names = o.list_local_models().await
			.map(|m| m.into_iter().map(|m| m.name).collect())
			.map_err(|e| e.to_string());
		let _ = t.send(Msg::Models(names));
	});

	let mut app = App::new();
	// Handle of the running chat task, kept so Esc can abort it.
	let mut task: Option<JoinHandle<()>> = None;

	while !app.quit {
		term.draw(|f| ui::draw(f, &app))?;
		let Some(msg) = rx.recv().await else { break };

		match msg {
			Msg::Models(Ok(m)) => app.set_models(m),
			Msg::Models(Err(e)) => app.models_error(e),
			Msg::Token(s) => app.push_token(&s),
			Msg::Done(e) => app.finish(e),
			// Presses and auto-repeats count; releases (reported on Windows) do not.
			Msg::Term(Event::Key(k)) if k.kind != KeyEventKind::Release => {
				// AltGr is reported as Ctrl+Alt on Windows and is how accents and
				// special characters (@ € { } ...) are typed, so it is not "Ctrl".
				let ctrl = k.modifiers.contains(KeyModifiers::CONTROL) && !k.modifiers.contains(KeyModifiers::ALT);

				// Order matters: global shortcuts first, then the picker
				// (which captures the arrows/Enter/Esc), then the chat keys.
				match (k.code, ctrl) {
					(KeyCode::Char('c'), true) => app.quit = true,
					// Changing the model mid-reply is blocked (busy).
					(KeyCode::Char('o'), true) if !app.busy => {
						app.picker = app.model.or(Some(0)).filter(|_| !app.models.is_empty());
					}
					_ if app.picker.is_some() => match k.code {
						KeyCode::Up => app.move_picker(-1),
						KeyCode::Down => app.move_picker(1),
						KeyCode::Enter => { app.model = app.picker.take(); }
						KeyCode::Esc => app.picker = None,
						_ => {}
					},
					(KeyCode::Enter, _) => {
						if let (Some(req), Some(_)) = (app.submit(), app.model) {
							task = Some(spawn_chat(&ollama, app.model_name().to_string(), req, tx.clone()));
						}
					}
					(KeyCode::Esc, _) => {
						if app.busy {
							if let Some(h) = task.take() {
								h.abort();
							}
							app.finish(Some("cancelled".into()));
						}
					}
					(KeyCode::Up, _) => app.scroll = app.scroll.saturating_add(1),
					(KeyCode::Down, _) => app.scroll = app.scroll.saturating_sub(1),
					(KeyCode::Backspace, _) => { app.input.pop(); }
					(KeyCode::Char(c), false) => app.input.push(c),
					_ => {}
				}
			}

			Msg::Term(_) => {}
		}
	}

	Ok(())
}

/// Starts the chat request in the background and streams the reply back as
/// `Token` messages, ending with a single `Done` (carrying an error, if any).
fn spawn_chat(ollama: &Ollama, model: String, history: Vec<ChatMessage>, tx: mpsc::UnboundedSender<Msg>) -> JoinHandle<()> {
	let ollama = ollama.clone();
	tokio::spawn(async move {
		let mut err = None;

		match ollama.send_chat_messages_stream(ChatMessageRequest::new(model, history)).await {
			Ok(mut s) => {
				while let Some(r) = s.next().await {
					match r {
						Ok(r) => { let _ = tx.send(Msg::Token(r.message.content)); }
						Err(_) => { err = Some("stream error".to_string()); break; }
					}
				}
			}

			Err(e) => err = Some(format!("error: {e}")),
		}

		let _ = tx.send(Msg::Done(err));
	})
}
