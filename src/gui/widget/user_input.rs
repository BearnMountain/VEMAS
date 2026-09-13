use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, style::Style, widgets::{Block, Paragraph}};

pub struct UserInput {
    pub input: String,
    character_index: usize,
    active: bool,
}

impl UserInput {
    pub fn new() -> Self {
        return Self {
            input: String::new(),
            character_index: 0,
            active: false,
        };
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let mut outline = Block::bordered();
        if self.active {
            outline = outline.border_style(Style::new().green().bold());
            // Put the terminal cursor at the current character position
            frame.set_cursor_position((
                area.x + self.character_index as u16 + 1,
                area.y + 1,
            ));
        }
        let paragraph = Paragraph::new(self.input.as_str())
            .block(outline);

        frame.render_widget(paragraph, area);
    }

    pub fn handle_event(&mut self, event: KeyCode) {
        match event {
            KeyCode::Char(c) => {
                self.enter_char(c);
            }

            KeyCode::Backspace => {
                self.delete_char();
            }

            KeyCode::Delete => {
                self.delete_next_char();
            }

            KeyCode::Left => {
                self.move_cursor_left();
            }

            KeyCode::Right => {
                self.move_cursor_right();
            }

            _ => {}
        }
    }

    pub fn toggle_active(&mut self) {
        self.active = !self.active;
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        if self.character_index == 0 {
            return;
        }

        let current_index = self.character_index;

        let from_left = self.input
            .chars()
            .take(current_index - 1)
            .collect::<String>();

        let byte_index = from_left.len();

        self.input.remove(byte_index);

        self.move_cursor_left();
    }

    fn delete_next_char(&mut self) {
        if self.character_index >= self.input.chars().count() {
            return;
        }

        let byte_index = self.byte_index();

        self.input.remove(byte_index);
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(index, _)| index)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
}
