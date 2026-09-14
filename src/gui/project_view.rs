use crossterm::event::{KeyCode::{self, Tab}, KeyEvent};
use ratatui::{Frame, layout::{Constraint, Direction, Layout, Rect, Spacing}, style::{Color, Style}, symbols::merge::MergeStrategy, widgets::{Block, Borders, Row, Table, TableState}};

use crate::{config::Config, project_data::{self, ProjectData, Shared}};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProjectViewState {
    IDLE,
}

pub struct ProjectView {
    project_data: Option<Shared<ProjectData>>,
    state: ProjectViewState,
    selection_state: TableState,
}

impl ProjectView {
    pub fn new(

    ) -> Self {
        let mut selection_state = TableState::default();
        selection_state.select_first();
        selection_state.select_next();

        return Self {
            project_data: None,
            state: ProjectViewState::IDLE,
            selection_state,
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

        let vertical = Layout::vertical([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
            ]).spacing(Spacing::Overlap(1)).split(area);

        let body = Layout::horizontal([
                Constraint::Percentage(30),
                Constraint::Fill(1),
            ]).spacing(Spacing::Overlap(1)).split(vertical[1]);

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
                    .title("Viewer")
                    .borders(Borders::ALL)
                    .merge_borders(MergeStrategy::Exact),
            );
        frame.render_widget(header, vertical[0]);

        // selection
        {
            let selection = Block::bordered()
                .merge_borders(MergeStrategy::Exact);
            frame.render_widget(&selection, body[0].clone());

            let inner = selection.inner(body[0]);
            self.render_info(frame, inner);
        }

        // info
        {
            let info = Block::bordered()
                .merge_borders(MergeStrategy::Exact);
            frame.render_widget(info.clone(), body[1]);

            let project_info_layout = Layout::vertical([
                Constraint::Percentage(40),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
            ]).split(info.inner(body[1]));

            let sources_data = (data.vhdl + data.sv).to_string();
            let testbenches_data = data.testbench.to_string();
            
            let overview_table = Table::new(
                [
                    Row::new(["Name", &data.name]),
                    Row::new(["Top", &data.top]),
                    Row::new(["Sources", &sources_data]),
                    Row::new(["Testbenches", &testbenches_data]),
                    Row::new(["Part", &data.part]),
                ], 
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ],
            );

            frame.render_widget(overview_table, project_info_layout[0]);
        }


        // footer
        let footer = Block::bordered()
            .merge_borders(MergeStrategy::Exact);
        frame.render_widget(footer, vertical[2]);

    }

    pub fn handle_events(&mut self, key_code: &KeyCode) -> ProjectViewState {
        match key_code {
            KeyCode::Char('j') => { 
                self.selection_state.select_next();
                if let Some(index) = self.selection_state.selected() {
                    // comparison is based off table headers to skip over them
                    if index == 3 || index == 6 || index == 10 {
                        self.selection_state.select_next();
                    }
                }
            },
            KeyCode::Char('k') => { 
                if let Some(index) = self.selection_state.selected() {
                    // comparison is based off table headers to skip over them
                    if index != 1 {
                        if index == 4 || index == 7 || index == 11 {
                            self.selection_state.select_previous();
                        } 
                        self.selection_state.select_previous();
                    }
                }
            },
            _ => {},
        }

        return self.state;
    }

    pub fn set_project_data(&mut self, project_data: Shared<ProjectData>) {
        self.project_data = Some(project_data);
    }

    fn render_info(&mut self, frame: &mut Frame, area: Rect) {
        
        let rows = vec![
            Row::new(["PROJECT"]).style(Style::default().bold()), // 0
            Row::new(["  Sources"]),
            Row::new(["  Testbenches"]),

            Row::new(["SIMULATION"]).style(Style::default().bold()), // 3
            Row::new(["  Run Simulation"]),
            Row::new(["  Testbenches"]),

            Row::new(["BUILD"]).style(Style::default().bold()), // 6
            Row::new(["  Synthesize"]),
            Row::new(["  Implement"]),
            Row::new(["  Generate Bitstream"]),

            Row::new(["FPGA"]).style(Style::default().bold()), // 10
            Row::new(["  Connect"]),
            Row::new(["  Program FPGA"]),
            Row::new(["  Hardware Manager"]),
        ];

        let table = Table::new(
                rows,
                [Constraint::Fill(1)],
            )
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .column_highlight_style(Color::Gray);

        frame.render_stateful_widget(table, area, &mut self.selection_state);

    }
}
