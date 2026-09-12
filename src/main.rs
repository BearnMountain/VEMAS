use crossterm::event::KeyCode;
use ratatui::{DefaultTerminal};

use crate::gui::Gui;

mod gui;
mod project_data;
mod config;

pub struct App {
    pub running: bool,

    pub gui: Gui,
}

impl App {
    pub fn new() -> Self {
        let gui = Gui::init();

        return Self {
            running: true,
            gui,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal 
    ) {
        loop {
            terminal.draw(|frame| self.gui.render(frame))
                .expect("failed to render frame");
            
            let event = crossterm::event::read().expect("failed to read terminal event");

            if let Some(key) = event.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => {
                        break
                    },
                    _ => {},
                }
            }

            self.gui.handle_event(event);
        }
    } 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui::run(|terminal|
        App::new().run(terminal)
    );

    return Ok(());
}
