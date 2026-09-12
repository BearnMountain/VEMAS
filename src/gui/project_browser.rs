use crossterm::event::{self, Event, KeyCode};
use ratatui::{Frame, layout::{Constraint, Rect}, style::{Color, Style}, widgets::{Row, Table, TableState}};

use crate::project_data::ProjectData;



pub struct ProjectBrowser {
    projects: Vec<ProjectData>,
    table_state: TableState,
}

impl ProjectBrowser {
    pub fn new(

    ) -> Self {
        let mut table_state = TableState::default();
        table_state.select_first();
        table_state.select_first_column();

        return Self {
            projects: Vec::default(),
            table_state,
        };
    }

    pub fn render(
        &mut self, 
        frame: &mut Frame, 
        area: Rect,
    ) {
        let header = Row::new(["Name", "VHDL", "SV", "TOP", "TESTBENCH", "PART"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let rows = self.projects.iter().clone()
            .into_iter()
            .map(|obj| Row::new(obj.to_string()))
            .collect::<Vec<Row>>();

        let widths = [
            Constraint::Min(12),
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Length(11),
            Constraint::Length(9),
            Constraint::Min(18),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .column_highlight_style(Color::Gray)
            .cell_highlight_style(Style::new().reversed().yellow())
            .highlight_symbol("> ");

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    pub fn handle_events(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.table_state.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.table_state.select_previous(),
                KeyCode::Char('g') => self.table_state.select_first(),
                KeyCode::Char('G') => self.table_state.select_last(),
                _ => {},
            }
        }
    }

    pub fn add_project(&mut self, project: ProjectData) {
        self.projects.push(project);
    }

    pub fn remove_project(&mut self, index: usize) {
        self.projects.remove(index);
    }

    pub fn select_project(&mut self) {

    }
}
