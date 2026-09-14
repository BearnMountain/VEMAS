use crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout}};

use crate::{gui::{project_browser::{ProjectBrowser, ProjectBrowserState}, project_simulate::ProjectSimulate, project_view::ProjectView}, project_data::ProjectData};

pub mod project_view;
pub mod project_browser;
pub mod widget;
pub mod project_simulate;

#[allow(non_camel_case_types)]
pub enum GuiState {
    BROWSER,
    VIEW,
    SIMULATE,
}

pub struct Gui {
    project_browser: ProjectBrowser,
    project_view: ProjectView,
    project_simulate: ProjectSimulate,
    state: GuiState,
}

impl Gui {
    pub fn init() -> Self {
        let data = [
            ProjectData::shared("calculator".to_string(), "calculator_top".to_string(), "xc7a100tcsg324-1".to_string(), 8, 0, 2),
            ProjectData::shared("uart".to_string(), "uart_top".to_string(), "xc7a100tcsg324-1".to_string(), 5, 3, 1),
            ProjectData::shared("memory".to_string(), "memory_top".to_string(), "xc7a100tcsg324-1".to_string(), 4, 2, 2),
            ProjectData::shared("counter".to_string(), "counter_top".to_string(), "xc7a100tcsg324-1".to_string(), 2, 0, 1),
        ];

        let mut project_browser = ProjectBrowser::new();
        project_browser.add_project(&data[0]);
        project_browser.add_project(&data[1]);
        project_browser.add_project(&data[2]);
        project_browser.add_project(&data[3]);

        return Self {
            project_browser,
            project_view: ProjectView::new(),
            project_simulate: ProjectSimulate::new(),
            state: GuiState::BROWSER,
        };
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        
        match self.state {
            GuiState::BROWSER => self.project_browser.render(frame, area),
            GuiState::VIEW => self.project_view.render(frame, area),
            GuiState::SIMULATE => self.project_simulate.render(frame, area),
        }
    }

    pub fn handle_event(&mut self, event: Event) {

        let key_code: Option<KeyCode>;
        match event.as_key_press_event() {
            Some(code) => key_code = Some(code.code),
            None => key_code = None,
        }

        match self.state {
            GuiState::BROWSER => {
                let browser_state = self.project_browser.handle_events(&event);

                // update files based on returned info
                match browser_state {
                    ProjectBrowserState::IDLE => {},
                    ProjectBrowserState::OPEN_PROJECT => {
                        self.project_view.set_project_data(ProjectData::shared(
                            "Calculator".to_string(), 
                            "calculator_top".to_string(), 
                            "xc7a100tcsg324-1".to_string(), 
                            8, 
                            0, 
                            3
                        ));
                        self.state = GuiState::VIEW;
                    },
                    ProjectBrowserState::NEW_PROJECT => {},
                    ProjectBrowserState::DELETE_PROJECT => {},
                }
            },
            GuiState::VIEW => {
                if key_code == None {
                    return;
                }
                let view_state = self.project_view.handle_events(&key_code.unwrap());
            },
            GuiState::SIMULATE => {

            },
        }
        

    }
}

