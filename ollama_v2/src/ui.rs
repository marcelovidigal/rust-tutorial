//! Renderização com ratatui.
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, List, ListItem, ListState, Paragraph, Wrap},
};
use ollama_rs::generation::chat::MessageRole;

use crate::app::App;

pub const NAME: &str = "Ollama Chat";
const ACCENT: Color = Color::Rgb(217, 119, 87);

pub fn draw(f: &mut Frame, app: &App) {
    let [head, body, input, foot] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(3),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .areas(f.area());

    header(f, head, app);
    messages(f, body, app);
    prompt(f, input, app);
    f.render_widget(
        Line::from(vec![
            Span::raw(format!(" {} ", app.status)).fg(ACCENT),
            "│ Enter envia · Ctrl+O modelo · ↑↓ rola · Esc cancela · Ctrl+C sai".dark_gray(),
        ]),
        foot,
    );
    if app.picker.is_some() {
        picker(f, app);
    }
}

fn header(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered().border_type(BorderType::Rounded).border_style(Style::new().fg(ACCENT));
    let line = Line::from(vec![
        " ◖●◗ ".fg(ACCENT).bold(), // logo genérico
        NAME.bold(),
        "  ·  modelo: ".dark_gray(),
        app.model_name().cyan(),
    ]);
    f.render_widget(Paragraph::new(line).block(block), area);
}

fn messages(f: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = vec![];
    for m in &app.history {
        let (tag, color) = match m.role {
            MessageRole::User => ("> ", Color::Cyan),
            _ => ("● ", ACCENT),
        };
        for (i, l) in m.content.lines().chain(m.content.is_empty().then_some("")).enumerate() {
            let prefix = if i == 0 { tag } else { "  " };
            lines.push(Line::from(vec![Span::styled(prefix, Style::new().fg(color).add_modifier(Modifier::BOLD)), Span::raw(l.to_string())]));
        }
        lines.push(Line::raw(""));
    }
    if lines.is_empty() {
        lines.push(Line::from("Digite uma mensagem para começar.".dark_gray()));
    }

    // rolagem ancorada no fim (estimativa de quebra de linha)
    let w = area.width.saturating_sub(1).max(1) as usize;
    let total: usize = lines.iter().map(|l| l.width().max(1).div_ceil(w)).sum();
    let max = total.saturating_sub(area.height as usize) as u16;
    let top = max.saturating_sub(app.scroll.min(max));
    f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }).scroll((top, 0)), area);
}

fn prompt(f: &mut Frame, area: Rect, app: &App) {
    let block = Block::bordered().border_type(BorderType::Rounded).border_style(Style::new().dark_gray());
    f.render_widget(Paragraph::new(format!("> {}", app.input)).block(block), area);
    if app.picker.is_none() {
        let x = area.x + 3 + app.input.chars().count() as u16;
        f.set_cursor_position((x.min(area.right().saturating_sub(2)), area.y + 1));
    }
}

fn picker(f: &mut Frame, app: &App) {
    let area = f.area();
    let h = (app.models.len() as u16 + 2).clamp(3, area.height.saturating_sub(2));
    let w = 44.min(area.width);
    let r = Rect::new(area.x + (area.width - w) / 2, area.y + (area.height - h) / 2, w, h);
    let items: Vec<ListItem> = app.models.iter().map(|m| ListItem::new(m.as_str())).collect();
    let list = List::new(items)
        .block(Block::bordered().title(" Modelo ").border_style(Style::new().fg(ACCENT)))
        .highlight_style(Style::new().bg(ACCENT).fg(Color::Black))
        .highlight_symbol("▸ ");
    f.render_widget(Clear, r);
    f.render_stateful_widget(list, r, &mut ListState::default().with_selected(app.picker));
}
