use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Alignment, Constraint, Direction, Layout, Rect, VerticalAlignment}, style::Style, widgets::{Block, Paragraph}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmSelection {
    Yes,
    No,
}

pub struct ConfirmDialog {
    selection: ConfirmSelection,
    content: String,
}

impl ConfirmDialog {
    pub fn new(
        content: String,
    ) -> Self {
        return Self {
            selection: ConfirmSelection::Yes,
            content,
        };
    }

    pub fn handle_event(&mut self, event: KeyEvent) -> Option<ConfirmSelection> {
        let mut res = None;
        match event.code {
            KeyCode::Char('l') => self.selection = ConfirmSelection::No,
            KeyCode::Char('h') => self.selection = ConfirmSelection::Yes,
            KeyCode::Enter => res = Some(self.selection),
            _ => {},
        }

        return res;
    }

    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
    ) {
        let popup_block = Block::bordered()
            .title(" Confirm ")
            .border_style(Style::new());

        let inner = popup_block.inner(area);

        frame.render_widget(popup_block, area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
            ])
            .split(inner);

        let paragraph = Paragraph::new(self.content.clone())
            .alignment(Alignment::Center);

        frame.render_widget(
            paragraph, 
            layout[0].centered_vertically(Constraint::Length(1))
        );

        let buttons = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(layout[1]);

        let yes_style = if self.selection == ConfirmSelection::Yes {
            Style::new().green().bold()
        } else {
            Style::new()
        };

        let no_style = if self.selection == ConfirmSelection::No {
            Style::new().red().bold()
        } else {
            Style::new()
        };

        let yes = Paragraph::new("Yes")
            .alignment(Alignment::Center)
            .style(yes_style)
            .block(
                Block::bordered()
                    .border_style(yes_style),
            );

        let no = Paragraph::new("No")
            .alignment(Alignment::Center)
            .style(no_style)
            .block(
                Block::bordered()
                    .border_style(no_style),
            );

        frame.render_widget(yes, buttons[0]);
        frame.render_widget(no, buttons[1]);
    }
}
