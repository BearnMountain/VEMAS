
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::{Alignment, Constraint, Direction, Layout, Rect}, widgets::{Block, Borders, Table}};

use crate::util::search::SearchTree;

pub struct SearchWidget {
    table: SearchTree,
    search_word: String,
}

impl SearchWidget {
    pub fn new(search_items: Vec<String>) -> Self {
        return Self {
            table: SearchTree::new(search_items),
            search_word: String::new(),
        };
    }

    pub fn render(
        &mut self, 
        frame: &mut Frame, 
        area: Rect,
    ) {
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



        // top part
        // let header = Row::new(["Name", "TOP", "VHDL", "SV", "TESTBENCH", "PART"])
        //     .style(Style::new().bold())
        //     .bottom_margin(1);
        //
        // let rows = self.projects
        //     .iter()
        //     .map(|obj| Row::new(obj.borrow().to_string()))
        //     .collect::<Vec<Row>>();
        //
        // let widths = [
        //     Constraint::Min(12),
        //     Constraint::Min(11),
        //     Constraint::Length(6),
        //     Constraint::Length(5),
        //     Constraint::Length(9),
        //     Constraint::Min(18),
        // ];
        //
        // let table = Table::new(rows, widths)
        //     .header(header)
        //     .column_spacing(1)
        //     .block(
        //         Block::default()
        //             .title("Projects")
        //             .borders(Borders::ALL),
        //     )
        //     .style(Color::White)
        //     .row_highlight_style(Style::new().on_black().bold())
        //     .column_highlight_style(Color::Gray)
        //     // .cell_highlight_style(Style::new().reversed().yellow())bro[name_area, location_area, fpga_area]
        //     .highlight_symbol("> ");
        //
        // frame.render_stateful_widget(table, chunks[0], &mut self.table_state);
        //
        // // bottom 
        // let rows = vec![
        //     Row::new([
        //         "j/k - Nav",
        //         "o - Open",
        //         "n - New",
        //         "d - Delete",
        //         "q - Quit",
        //     ]),
        // ];
        //
        // let widths = [
        //     Constraint::Percentage(20),
        //     Constraint::Percentage(20),
        //     Constraint::Percentage(20),
        //     Constraint::Percentage(20),
        //     Constraint::Percentage(20),
        // ];
        //
        // let help = Table::new(rows, widths)
        //     .block(
        //         Block::default()
        //             .title("Help")
        //             .borders(Borders::ALL),
        //     )
        //     .column_spacing(1);
        //
        // frame.render_widget(help, layout[1]);    
    }

    pub fn handle_events(&mut self, event: &KeyCode) {

    }

}
