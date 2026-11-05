//! Estado da aplicação (sem I/O).
use ollama_rs::generation::chat::{ChatMessage, MessageRole};

pub struct App {
    pub history: Vec<ChatMessage>,
    pub input: String,
    pub models: Vec<String>,
    pub model: Option<usize>,
    pub picker: Option<usize>,
    pub busy: bool,
    pub status: String,
    /// linhas roladas a partir do fim
    pub scroll: u16,
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
            status: "carregando modelos...".into(),
            scroll: 0,
            quit: false,
        }
    }

    pub fn model_name(&self) -> &str {
        self.model.map_or("(nenhum)", |i| self.models[i].as_str())
    }

    pub fn set_models(&mut self, models: Vec<String>) {
        self.model = if models.is_empty() { None } else { Some(0) };
        self.status = if models.is_empty() { "nenhum modelo encontrado".into() } else { "pronto".into() };
        self.models = models;
    }

    /// Adiciona a mensagem do usuário e abre uma resposta vazia do assistente.
    pub fn submit(&mut self) -> Option<Vec<ChatMessage>> {
        let text = self.input.trim().to_string();
        if text.is_empty() || self.busy || self.model.is_none() {
            return None;
        }
        self.input.clear();
        self.history.push(ChatMessage::user(text));
        let req = self.history.clone();
        self.history.push(ChatMessage::assistant(String::new()));
        self.busy = true;
        self.scroll = 0;
        self.status = "gerando...".into();
        Some(req)
    }

    pub fn push_token(&mut self, tok: &str) {
        if let Some(m) = self.history.last_mut().filter(|m| m.role == MessageRole::Assistant) {
            m.content.push_str(tok);
        }
    }

    pub fn finish(&mut self, err: Option<String>) {
        self.busy = false;
        self.status = err.unwrap_or_else(|| "pronto".into());
    }

    pub fn move_picker(&mut self, delta: isize) {
        if let Some(i) = self.picker {
            let n = self.models.len() as isize;
            self.picker = Some((i as isize + delta).rem_euclid(n.max(1)) as usize);
        }
    }
}
