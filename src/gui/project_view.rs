use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect}, widgets::{Block, Borders, Row, Table}};

use crate::{config::Config, project_data::{self, ProjectData, Shared}};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProjectViewState {
    IDLE,
}

pub struct ProjectView {
    project_data: Option<Shared<ProjectData>>,
    state: ProjectViewState,
}

impl ProjectView {
    pub fn new(

    ) -> Self {


        return Self {
            project_data: None,
            state: ProjectViewState::IDLE,
        };
    }

    pub fn render(
        &mut self,
        frame: &mut Frame, 
        area: Rect,
    ) {
        let pdata = match &self.project_data {
            Some(project_data) => project_data.clone(),
            None => ProjectData::shared(
                "invalid".into(), 
                "invalid".into(), 
                "invalid".into(), 
                0, 
                0, 
                0,
            ),
        };

        let data = pdata.borrow();

        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Percentage(90),
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
            [
                Row::new([
                    format!("Project: {}", data.name),
                    format!("Part: {}", data.part),
                    format!("Vivado: {}", Config::get().general.vivado_version),
                ])
            ],
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
        ).block(
                Block::default()
                    // .title("VEM - Vivado Environment Manager")
                    .borders(Borders::ALL),
            );
        frame.render_widget(header, vertical[0]);

        // selection
        let selection = Block::bordered();
        frame.render_widget(selection, body[0]);

        // info
        let info = Block::bordered();
        frame.render_widget(info, body[1]);

        // footer
        let footer = Block::bordered();
        frame.render_widget(footer, vertical[2]);

    }

    pub fn handle_events(&mut self, event: &KeyCode) -> ProjectViewState {


        return self.state;
    }

    pub fn set_project_data(&mut self, project_data: Shared<ProjectData>) {
        self.project_data = Some(project_data);
    }
}
