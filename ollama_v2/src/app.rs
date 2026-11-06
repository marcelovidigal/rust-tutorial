// Application state (no I/O): everything the UI shows lives here.
// main.rs mutates it in response to events; ui.rs only reads it.
use ollama_rs::generation::chat::{ChatMessage, MessageRole};

pub struct App {
	/// Conversation so far; the last item is the reply being streamed.
	pub history: Vec<ChatMessage>,
	/// Text currently typed in the prompt box.
	pub input: String,
	/// Names of the models installed in Ollama.
	pub models: Vec<String>,
	/// Index into `models` of the selected model.
	pub model: Option<usize>,
	/// Highlighted row of the model picker; `None` when it is closed.
	pub picker: Option<usize>,
	/// True while a reply is being generated.
	pub busy: bool,
	/// Short message shown in the footer.
	pub status: String,
	/// Lines scrolled up from the bottom of the chat (0 = follow the end).
	pub scroll: u16,
	/// Set to true to leave the main loop.
	pub quit: bool,
}

impl App {
	pub fn new() -> Self {
		Self {
			history: vec![],
			input: String::new(),
			models: vec![],
			model: None,
			picker: None,
			busy: false,
			status: "loading models...".into(),
			scroll: 0,
			quit: false,
		}
	}

	/// Name of the selected model, or a placeholder when none is available.
	pub fn model_name(&self) -> &str {
		self.model.map_or("(none)", |i| self.models[i].as_str())
	}

	/// Stores the installed models and selects the first one.
	pub fn set_models(&mut self, models: Vec<String>) {
		self.model = if models.is_empty() { None } else { Some(0) };
		self.status = if models.is_empty() { "no models found".into() } else { "ready".into() };
		self.models = models;
	}

	/// Moves the typed text into the history as a user message and opens an
	/// empty assistant reply. Returns the messages to send, or `None` if
	/// there is nothing to send (empty input, busy, or no model).
	pub fn submit(&mut self) -> Option<Vec<ChatMessage>> {
		let text = self.input.trim().to_string();

		if text.is_empty() || self.busy || self.model.is_none() {
			return None;
		}

		self.input.clear();
		self.history.push(ChatMessage::user(text));
		// The request is cloned before the empty reply is added, so the
		// model only sees real messages.
		let req = self.history.clone();
		self.history.push(ChatMessage::assistant(String::new()));
		self.busy = true;
		self.scroll = 0;
		self.status = "generating...".into();
		Some(req)
	}

	/// Shows why the model list could not be loaded (e.g. Ollama is down).
	pub fn models_error(&mut self, err: String) {
		self.status = format!("cannot reach Ollama: {err}");
	}

	/// Appends a streamed token to the assistant reply being built.
	/// Ignored when not busy, so late tokens after a cancel are dropped.
	pub fn push_token(&mut self, tok: &str) {
		if !self.busy {
			return;
		}

		if let Some(m) = self.history.last_mut().filter(|m| m.role == MessageRole::Assistant) {
			m.content.push_str(tok);
		}
	}

	/// Ends generation; `err` replaces the status when something went wrong.
	/// An empty assistant reply is removed so it is not resent next turn.
	pub fn finish(&mut self, err: Option<String>) {
		if self.history.last().is_some_and(|m| m.role == MessageRole::Assistant && m.content.is_empty()) {
			self.history.pop();
		}

		self.busy = false;
		self.status = err.unwrap_or_else(|| "ready".into());
	}

	/// Moves the picker highlight by `delta`, wrapping around both ends.
	pub fn move_picker(&mut self, delta: isize) {
		if let Some(i) = self.picker {
			let n = self.models.len() as isize;
			self.picker = Some((i as isize + delta).rem_euclid(n.max(1)) as usize);
		}
	}
}
