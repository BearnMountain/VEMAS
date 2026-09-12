use crossterm::event::Event;
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Direction, Layout}};

use crate::{gui::project_browser::ProjectBrowser, project_data::ProjectData};

mod project_view;
mod project_browser;

pub struct Gui {
    project_browser: ProjectBrowser,
}

impl Gui {
    pub fn init() -> Self {
        let mut project_browser = ProjectBrowser::new();
        project_browser.add_project(ProjectData::new("calculator".to_string(), "calculator_top".to_string(), "xc7a100tcsg324-1".to_string(), 8, 0, 2));
        project_browser.add_project(ProjectData::new("uart".to_string(), "uart_top".to_string(), "xc7a100tcsg324-1".to_string(), 5, 3, 1));
        project_browser.add_project(ProjectData::new("memory".to_string(), "memory_top".to_string(), "xc7a100tcsg324-1".to_string(), 4, 2, 2));
        project_browser.add_project(ProjectData::new("counter".to_string(), "counter_top".to_string(), "xc7a100tcsg324-1".to_string(), 2, 0, 1));

                                 

        return Self {
            project_browser,
        };
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        self.project_browser.render(frame, area);
    }

    pub fn handle_event(&mut self, event: Event) {
        self.project_browser.handle_events(&event);
    }
}

