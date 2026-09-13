use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Alignment, Constraint, Direction, Layout, Margin, Rect}, widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState}};

use crate::{gui::widget::{project_creation_wizard::ActiveFormQuestion::PROJECT_NAME, search_window::SearchWidget, user_input::UserInput}, project_data::ProjectData};

#[derive(Debug, Copy, Clone)]
pub enum ActiveFormQuestion {
    PROJECT_NAME,
    LOCATION,
    FPGA_PART,
}

impl ActiveFormQuestion {
    fn next(self) -> Self {
        match self {
            ActiveFormQuestion::PROJECT_NAME => ActiveFormQuestion::LOCATION,
            ActiveFormQuestion::LOCATION => ActiveFormQuestion::FPGA_PART,
            ActiveFormQuestion::FPGA_PART => ActiveFormQuestion::PROJECT_NAME,
        }
    }

    fn prev(self) -> Self {
        match self {
            ActiveFormQuestion::PROJECT_NAME => ActiveFormQuestion::FPGA_PART,
            ActiveFormQuestion::LOCATION => ActiveFormQuestion::PROJECT_NAME,
            ActiveFormQuestion::FPGA_PART => ActiveFormQuestion::LOCATION,
        }
    }
}

/// Compacted and stripped down form of vivado new project wizard
/// supports: 
/// - project name
/// - location
/// - fpga part + part selection wizard: 
///     - comes with xdc file for supported boards
/// - 
pub struct ProjectCreationWizard {
    project_data: ProjectData,
    
    vertical_scroll_state: ScrollbarState,
    vertical_scroll: usize,

    active: ActiveFormQuestion,

    project_name_info: UserInput,
    location_info: UserInput,
    fpga_part_info: SearchWidget,
    fpga_dialog_active: bool,
}

impl ProjectCreationWizard {
    pub fn new() -> Self {
        let mut project_name_info = UserInput::new();
        project_name_info.toggle_active();

        return Self {
            project_data: ProjectData::default(),
            vertical_scroll_state: ScrollbarState::new(100),
            vertical_scroll: 100,
            active: ActiveFormQuestion::PROJECT_NAME,
            project_name_info,
            location_info: UserInput::new(),
            fpga_part_info: SearchWidget::new(vec![
                "xc7a100tcsg383-1".to_string(),
                "xc7a100fiwe291-1".to_string(),
                "xc7a103tcsg244-1".to_string(),
                "xc7a100txjg510-1".to_string(),
            ]),
            fpga_dialog_active: false,
        };
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let popup_block = Block::bordered()
            .title("Create Project")
            .title_alignment(Alignment::Center);

        frame.render_widget(&popup_block, area);

        let inner = popup_block.inner(area);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ])
            .split(inner);

        // project name
        let name_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length(2), Constraint::Length(3)],
        ).split(layout[0]);
        let name = Paragraph::new("Project Name");
        frame.render_widget(name, Rect {
            x: name_layout[0].x,
            y: name_layout[0].bottom().saturating_sub(1),
            width: name_layout[0].width,
            height: 2,
        });
        self.project_name_info.render(frame, name_layout[1]);

        // location
        let location_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length(2), Constraint::Length(3)],
        ).split(layout[1]);
        let name = Paragraph::new("Location");
        frame.render_widget(name, Rect {
            x: location_layout[0].x,
            y: location_layout[0].bottom().saturating_sub(1),
            width: location_layout[0].width,
            height: 2,
        });
        self.location_info.render(frame, location_layout[1]);

        // fpga part
        let part_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Length(2), Constraint::Length(3)],
        ).split(layout[2]);
        let name = Paragraph::new("FPGA Part")
            .alignment(Alignment::Left);
        frame.render_widget(name, Rect {
            x: part_layout[0].x,
            y: part_layout[0].bottom().saturating_sub(1),
            width: part_layout[0].width,
            height: 2,
        });
        let part_info = Paragraph::new(self.project_data.part.clone())
            .block(Block::bordered());
        frame.render_widget(part_info, part_layout[1]);



        // let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        // frame.render_stateful_widget(
        //     scrollbar,
        //     inner.inner(Margin {
        //         vertical: 1,
        //         horizontal: 0,
        //     }),
        //     &mut self.vertical_scroll_state,
        // );
    }

    pub fn handle_event(&mut self, key_code: KeyEvent) {
        match key_code.code {
            KeyCode::Tab => {
                match self.active {
                    ActiveFormQuestion::PROJECT_NAME => self.project_name_info.toggle_active(),
                    ActiveFormQuestion::LOCATION => self.location_info.toggle_active(),
                    ActiveFormQuestion::FPGA_PART => {},
                }
                self.active = self.active.next();
                match self.active {
                    ActiveFormQuestion::PROJECT_NAME => self.project_name_info.toggle_active(),
                    ActiveFormQuestion::LOCATION => self.location_info.toggle_active(),
                    ActiveFormQuestion::FPGA_PART => {},
                }
            },
            KeyCode::Down => self.scroll_down(),
            KeyCode::Up => self.scroll_up(),
            KeyCode::Enter => {
                if self.active == ActiveFormQuestion::FPGA_PART {
                    self.fpga_part_info = true;
                }
            }
            _ => {},
        }

        match self.active {
            ActiveFormQuestion::PROJECT_NAME => self.project_name_info.handle_event(key_code.code),
            ActiveFormQuestion::LOCATION => self.location_info.handle_event(key_code.code),
            ActiveFormQuestion::FPGA_PART => self.fpga_part_info.handle_events(key_code.code),
        }
    }

    const fn scroll_down(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_add(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }

    const fn scroll_up(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
        self.vertical_scroll_state = self.vertical_scroll_state.position(self.vertical_scroll);
    }
}
