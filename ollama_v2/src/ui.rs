// Rendering with ratatui: turns the `App` state into widgets. Read-only.
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
/// Maximum text rows the prompt box may grow to.
const MAX_INPUT_ROWS: usize = 8;

/// Draws one frame: header, chat, prompt, footer, and the model picker on top.
pub fn draw(f: &mut Frame, app: &App) {
	// The prompt height depends on the text, so it is computed first.
	// The chat takes whatever space is left (`Min`).
	let input_h = input_rows(app, f.area().width) as u16 + 2;
	let [head, body, input, foot] = Layout::vertical([
		Constraint::Length(3),
		Constraint::Min(3),
		Constraint::Length(input_h),
		Constraint::Length(1),
	])
	.areas(f.area());

	header(f, head, app);
	messages(f, body, app);
	prompt(f, input, app);
	f.render_widget(
		Line::from(vec![
			Span::raw(format!(" {} ", app.status)).fg(ACCENT),
			"│ Enter send · Ctrl+O model · ↑↓ scroll · Esc cancel · Ctrl+C quit".dark_gray(),
		]),
		foot,
	);

	// Drawn last so it appears above everything else.
	if app.picker.is_some() {
		picker(f, app);
	}
}

/// Top bar with the app name and the selected model.
fn header(f: &mut Frame, area: Rect, app: &App) {
	let block = Block::bordered().border_type(BorderType::Rounded).border_style(Style::new().fg(ACCENT));
	let line = Line::from(vec![
		" ◖●◗ ".fg(ACCENT).bold(), // generic logo
		NAME.bold(),
		"  ·  model: ".dark_gray(),
		app.model_name().cyan(),
	]);
	f.render_widget(Paragraph::new(line).block(block), area);
}

/// The conversation: one tagged block per message, scrolled from the bottom.
fn messages(f: &mut Frame, area: Rect, app: &App) {
	let mut lines: Vec<Line> = vec![];

	for m in &app.history {
		let (tag, color) = match m.role {
			MessageRole::User => ("> ", Color::Cyan),
			_ => ("● ", ACCENT),
		};

		// Only the first line of a message gets the tag; the rest is
		// indented. The `chain` keeps an empty reply visible as one line.
		for (i, l) in m.content.lines().chain(m.content.is_empty().then_some("")).enumerate() {
			let prefix = if i == 0 { tag } else { "  " };
			lines.push(Line::from(vec![Span::styled(prefix, Style::new().fg(color).add_modifier(Modifier::BOLD)), Span::raw(l.to_string())]));
		}

		lines.push(Line::raw(""));
	}

	if lines.is_empty() {
		lines.push(Line::from("Type a message to start.".dark_gray()));
	}

	// Scroll is anchored to the end: estimate how many rows the text takes
	// once wrapped, then `top` is the first visible row (scroll goes up from the end).
	let w = area.width.saturating_sub(1).max(1) as usize;
	let total: usize = lines.iter().map(|l| l.width().max(1).div_ceil(w)).sum();
	let max = total.saturating_sub(area.height as usize) as u16;
	let top = max.saturating_sub(app.scroll.min(max));
	f.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }).scroll((top, 0)), area);
}

/// Splits `"> " + input` into rows that fit the box. Returns the chars and
/// each row as a `(start, end)` range (end exclusive).
fn input_ranges(app: &App, width: u16) -> (Vec<char>, Vec<(usize, usize)>) {
	let chars: Vec<char> = format!("> {}", app.input).chars().collect();
	let ranges = wrap_ranges(&chars, width.saturating_sub(2) as usize);
	(chars, ranges)
}

/// Text rows needed by the prompt (capped at `MAX_INPUT_ROWS`).
fn input_rows(app: &App, width: u16) -> usize {
	input_ranges(app, width).1.len().min(MAX_INPUT_ROWS)
}

/// Word wrap: a row breaks after its last space, so words stay whole;
/// a word longer than the row is split. One column is kept free for the cursor.
fn wrap_ranges(chars: &[char], width: usize) -> Vec<(usize, usize)> {
	let w = width.saturating_sub(1).max(1);
	let n = chars.len();
	let mut out = vec![];
	let mut start = 0;

	loop {
		let mut i = start;
		let mut last_space = None;

		while i < n && i - start < w {
			if chars[i] == ' ' {
				last_space = Some(i);
			}
			i += 1;
		}

		if i >= n {
			out.push((start, n));
			break;
		}

		if chars[i] == ' ' {
			// The row ends exactly before a space: break there and skip it.
			out.push((start, i));
			start = i + 1;
		} else {
			let brk = last_space.map_or(i, |s| s + 1);
			out.push((start, brk));
			start = brk;
		}
	}

	out
}

/// The input box, word-wrapped by `wrap_ranges`. The cursor is always at the
/// end of the text, so its position comes from the last row.
fn prompt(f: &mut Frame, area: Rect, app: &App) {
	let block = Block::bordered().border_type(BorderType::Rounded).border_style(Style::new().dark_gray());
	let (chars, ranges) = input_ranges(app, area.width);
	// When the text is taller than the box, show only the last rows.
	let scroll = ranges.len().saturating_sub(area.height.saturating_sub(2) as usize);
	let lines: Vec<Line> = ranges.iter().map(|&(s, e)| Line::raw(chars[s..e].iter().collect::<String>())).collect();
	f.render_widget(Paragraph::new(lines).block(block).scroll((scroll as u16, 0)), area);

	if app.picker.is_none() {
		let row = ranges.len() - 1 - scroll.min(ranges.len() - 1);
		let col = chars.len() - ranges[ranges.len() - 1].0;
		f.set_cursor_position((area.x + 1 + col as u16, area.y + 1 + row as u16));
	}
}

#[cfg(test)]
mod tests {
	use super::wrap_ranges;

	fn rows(s: &str, width: usize) -> Vec<String> {
		let c: Vec<char> = s.chars().collect();
		wrap_ranges(&c, width).iter().map(|&(a, b)| c[a..b].iter().collect()).collect()
	}

	#[test]
	fn keeps_words_whole() {
		assert_eq!(rows("> hello wonderful world", 12), vec!["> hello ", "wonderful ", "world"]);
	}

	#[test]
	fn splits_a_word_longer_than_the_row() {
		assert_eq!(rows("abcdefghij", 5), vec!["abcd", "efgh", "ij"]);
	}

	#[test]
	fn short_text_is_one_row() {
		assert_eq!(rows("> hi", 20), vec!["> hi"]);
	}
}

/// Centered pop-up list for choosing the model (opened with Ctrl+O).
fn picker(f: &mut Frame, app: &App) {
	let area = f.area();
	let h = (app.models.len() as u16 + 2).clamp(3, area.height.saturating_sub(2));
	let w = 44.min(area.width);
	let r = Rect::new(area.x + (area.width - w) / 2, area.y + (area.height - h) / 2, w, h);
	let items: Vec<ListItem> = app.models.iter().map(|m| ListItem::new(m.as_str())).collect();
	let list = List::new(items)
		.block(Block::bordered().title(" Model ").border_style(Style::new().fg(ACCENT)))
		.highlight_style(Style::new().bg(ACCENT).fg(Color::Black))
		.highlight_symbol("▸ ");
	// `Clear` erases what is behind the pop-up before drawing it.
	f.render_widget(Clear, r);
	f.render_stateful_widget(list, r, &mut ListState::default().with_selected(app.picker));
}
