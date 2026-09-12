use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, widgets::{Block, Borders, Row, Table}};

use crate::project_data::{ProjectData, Shared};

pub struct ProjectView {
    project_data: Option<Shared<ProjectData>>,
}

impl ProjectView {
    pub fn new(

    ) -> Self {


        return Self {
            project_data: None,
        };
    }

    pub fn render(
        &mut self,
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

        // header
        let header = Table::new(
            [Row::new(["Hello", "World"])],
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
        ).block(
                Block::default()
                    // .title("Projects")
                    .borders(Borders::ALL),
            );
        frame.render_widget(header, vertical[0]);

        // selection
        // info
        // footer
    }

    pub fn set_project_data(&mut self, project_data: Shared<ProjectData>) {
        self.project_data = Some(project_data);
    }
}
