//! Chat TUI para Ollama (ratatui). Uso: `cargo run`
mod app;
mod ui;

use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind, KeyModifiers};
use futures::StreamExt;
use ollama_rs::{Ollama, generation::chat::{ChatMessage, request::ChatMessageRequest}};
use tokio::{sync::mpsc, task::JoinHandle};

use app::App;

enum Msg {
    Term(Event),
    Models(Vec<String>),
    Token(String),
    Done(Option<String>),
}

/*
// original code, simple example using `GenerationRequest``
/// example client ollama with ratatui interface
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
	let mut term = ratatui::init();
	let res = run(&mut term).await;
	ratatui::restore();
	res
}

async fn run(term: &mut ratatui::DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
	let ollama = Ollama::default();
	let (tx, mut rx) = mpsc::unbounded_channel();

	// eventos do terminal
	let t = tx.clone();
    
	tokio::spawn(async move {
		let mut events = EventStream::new();
		
		while let Some(Ok(e)) = events.next().await {
			if t.send(Msg::Term(e)).is_err() { break; }
		}
	});

	// lista de modelos instalados
	let (t, o) = (tx.clone(), ollama.clone());
	
	tokio::spawn(async move {
		let names = o.list_local_models().await.map(|m| m.into_iter().map(|m| m.name).collect()).unwrap_or_default();
		let _ = t.send(Msg::Models(names));
	});

	let mut app = App::new();
	let mut task: Option<JoinHandle<()>> = None;

	while !app.quit {
		term.draw(|f| ui::draw(f, &app))?;
		let Some(msg) = rx.recv().await else { break };
		
		match msg {
			Msg::Models(m) => app.set_models(m),
			Msg::Token(s) => app.push_token(&s),
			Msg::Done(e) => app.finish(e),
			Msg::Term(Event::Key(k)) if k.kind == KeyEventKind::Press => {
				let ctrl = k.modifiers.contains(KeyModifiers::CONTROL);
				
				match (k.code, ctrl) {
					(KeyCode::Char('c'), true) => app.quit = true,
					(KeyCode::Char('o'), true) => app.picker = app.model.or(Some(0)).filter(|_| !app.models.is_empty()),
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
						if let Some(h) = task.take().filter(|_| app.busy) {
							h.abort();
							app.finish(Some("cancelado".into()));
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

fn spawn_chat(ollama: &Ollama, model: String, history: Vec<ChatMessage>, tx: mpsc::UnboundedSender<Msg>) -> JoinHandle<()> {
	let ollama = ollama.clone();
	tokio::spawn(async move {
		let mut err = None;
		
		match ollama.send_chat_messages_stream(ChatMessageRequest::new(model, history)).await {
			Ok(mut s) => {
				while let Some(r) = s.next().await {
					match r {
						Ok(r) => { let _ = tx.send(Msg::Token(r.message.content)); }
						Err(_) => { err = Some("erro no stream".to_string()); break; }
					}
				}
			}
			
			Err(e) => err = Some(format!("erro: {e}")),
		}
		
		let _ = tx.send(Msg::Done(err));
	})
}