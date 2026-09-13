use std::rc::Rc;

use crossterm::{event::{Event, KeyCode}};
use ratatui::{
    Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, widgets::{Block, Borders, Clear, Paragraph, Row, Table, TableState},
};

use crate::{
    gui::widget::{confirm_dialog::{ConfirmDialog, ConfirmSelection}, project_creation_wizard::ProjectCreationWizard}, project_data::{ProjectData, Shared}};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ProjectBrowserState {
    IDLE,
    OPEN_PROJECT,
    NEW_PROJECT,
    DELETE_PROJECT,
}

pub struct ProjectBrowser {
    projects: Vec<Shared<ProjectData>>,
    table_state: TableState,

    confirm_dialog: ConfirmDialog, // for delete
    project_creation_wizard: ProjectCreationWizard, // for add

    state: ProjectBrowserState,
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
            confirm_dialog: ConfirmDialog::new(
                "Are you sure you want to delete this project?".to_string()
            ),
            project_creation_wizard: ProjectCreationWizard::new(),
            state: ProjectBrowserState::IDLE,
        };
    }

    pub fn render(
        &mut self, 
        frame: &mut Frame, 
        area: Rect,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // top part
        let header = Row::new(["Name", "TOP", "VHDL", "SV", "TESTBENCH", "PART"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let rows = self.projects
            .iter()
            .map(|obj| Row::new(obj.borrow().to_string()))
            .collect::<Vec<Row>>();

        let widths = [
            Constraint::Min(12),
            Constraint::Min(11),
            Constraint::Length(6),
            Constraint::Length(5),
            Constraint::Length(9),
            Constraint::Min(18),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(1)
            .block(
                Block::default()
                    .title("Projects")
                    .borders(Borders::ALL),
            )
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .column_highlight_style(Color::Gray)
            // .cell_highlight_style(Style::new().reversed().yellow())bro[name_area, location_area, fpga_area]
            .highlight_symbol("> ");

        frame.render_stateful_widget(table, chunks[0], &mut self.table_state);

        // bottom 
        let rows = vec![
            Row::new([
                "j/k - Nav",
                "o - Open",
                "n - New",
                "d - Delete",
                "q - Quit",
            ]),
        ];

        let widths = [
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];

        let help = Table::new(rows, widths)
            .block(
                Block::default()
                    .title("Help")
                    .borders(Borders::ALL),
            )
            .column_spacing(1);

        frame.render_widget(help, chunks[1]);    

        // state rendering
        if self.state != ProjectBrowserState::IDLE {

            match self.state {
                ProjectBrowserState::NEW_PROJECT => {
                    let centered_area = area.centered(
                        Constraint::Percentage(90),
                        Constraint::Percentage(90),
                    );

                    frame.render_widget(Clear, centered_area);

                    let popup_block = Block::bordered().title("Popup");
                    frame.render_widget(popup_block, centered_area);

                    self.project_creation_wizard.render(frame, centered_area);
				},
                ProjectBrowserState::DELETE_PROJECT => {
                    let centered_area = area.centered(
                        Constraint::Percentage(60),
                        Constraint::Percentage(40),
                    );

                    frame.render_widget(Clear, centered_area);

                    self.confirm_dialog.render(frame, centered_area);
				},
                _ => {},
            }
        }
    }

    pub fn handle_events(&mut self, event: &Event) -> ProjectBrowserState {
        if self.state == ProjectBrowserState::IDLE {
            if let Some(key) = event.as_key_press_event() {
                match key.code {
                    KeyCode::Char('j') | KeyCode::Down => self.table_state.select_next(),
                    KeyCode::Char('k') | KeyCode::Up => self.table_state.select_previous(),
                    KeyCode::Char('g') => self.table_state.select_first(),
                    KeyCode::Char('G') => self.table_state.select_last(),
                    KeyCode::Char('o') => self.state = ProjectBrowserState::OPEN_PROJECT,
                    KeyCode::Char('n') => self.state = ProjectBrowserState::NEW_PROJECT,
                    KeyCode::Char('d') => self.state = ProjectBrowserState::DELETE_PROJECT,
                    KeyCode::Esc => self.state = ProjectBrowserState::IDLE,
                    _ => {},
                }
            }
        } else if self.state == ProjectBrowserState::DELETE_PROJECT {
            if  let Some(key_code) = event.as_key_press_event() && 
                let Some(confirmation) = self.confirm_dialog.handle_event(key_code) 
            {
                match confirmation {
                    ConfirmSelection::Yes => {
                        if let Some(index) = self.table_state.selected() {
                            self.projects.remove(index);
                        }
                        self.state = ProjectBrowserState::IDLE;
                    },
                    ConfirmSelection::No => {
                        self.state = ProjectBrowserState::IDLE;
                    },
                }
            }
        } else if self.state == ProjectBrowserState::NEW_PROJECT {
            if  let Some(key_code) = event.as_key_press_event() {
                self.project_creation_wizard.handle_event(key_code);
                
            }
        }

        return self.state;
    }

    pub fn add_project(&mut self, project: &Shared<ProjectData>) {
        self.projects.push(project.clone());
    }

    pub fn remove_project(&mut self, project: &Shared<ProjectData>) {
        if let Some(index) = self.projects
            .iter()
            .position(|data| Rc::ptr_eq(data, project))
        {
            self.projects.remove(index);
        }
    }

    pub fn select_project(&mut self) -> Option<Shared<ProjectData>> {
        if let Some(index) = self.table_state.selected() {
            return Some(self.projects[index].clone());
        } else {
            return None;
        }
    }
}
