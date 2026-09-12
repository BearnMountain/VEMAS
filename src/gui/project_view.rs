use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}};



pub struct ProjectView {
    name: String,
    part: String,
}

impl ProjectView {
    pub fn new(

    ) -> Self {


        return Self {
            name: todo!(),
            part: todo!(),
        };
    }

    pub fn render(
        frame: &mut Frame, 
        area: Rect,
    ) {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(area);

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ])
            .split(vertical[1]);
    }
}
