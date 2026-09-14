use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::{DefaultTerminal};

use crate::{config::Config, gui::Gui};

mod gui;
mod project_data;
mod config;
mod util;

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
                if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }
            }

            self.gui.handle_event(event);
        }
    } 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // setting up app defaults
    if let Err(e) = Config::init("config.toml") {
        eprintln!("Failed to load config: {e}");
        std::process::exit(1);
    }

    ratatui::run(|terminal|
        App::new().run(terminal)
    );

    return Ok(());
}
