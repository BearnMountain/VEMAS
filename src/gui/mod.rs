use crossterm::event::Event;
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout}};

use crate::{gui::{project_browser::{ProjectBrowser, ProjectBrowserState}, project_view::ProjectView}, project_data::ProjectData};

pub mod project_view;
pub mod project_browser;
pub mod widget;

#[allow(non_camel_case_types)]
pub enum GuiState {
    BROWSER,
    VIEW,
}

pub struct Gui {
    project_browser: ProjectBrowser,
    project_view: ProjectView,
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
            state: GuiState::BROWSER,
        };
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        
        match self.state {
            GuiState::BROWSER => self.project_browser.render(frame, area),
            GuiState::VIEW => self.project_view.render(frame, area),
        }
    }

    pub fn handle_event(&mut self, event: Event) {

        match self.state {
            GuiState::BROWSER => {
                let browser_state = self.project_browser.handle_events(&event);

                // match browser_state {
                //     ProjectBrowserState::OPEN_PROJECT => {
                //         if let Some(project_data) = self.project_browser.select_project() {
                //             self.project_view.set_project_data(project_data);
                //             self.state = GuiState::VIEW;
                //         }
                //     },
                //     ProjectBrowserState::NEW_PROJECT => {
                //         let data = ProjectData::shared(
                //             "counter".to_string(), 
                //             "counter_top".to_string(), 
                //             "xc7a100tcsg324-1".to_string(), 
                //             2, 
                //             0, 
                //             1
                //         );
                //         self.project_browser.add_project(&data);
                //     },
                //     ProjectBrowserState::DELETE_PROJECT => {
                //         if let Some(project_data) = self.project_browser.select_project() {
                //             self.project_browser.remove_project(&project_data);                        
                //         }
                //     }
                //     _ => {}
                // }
            },
            GuiState::VIEW => {

            },
        }
        

    }
}

