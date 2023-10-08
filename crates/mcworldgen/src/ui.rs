use std::collections::HashMap;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Widget, Table, Row, Cell},
    Frame, buffer::Buffer, style::{Style, Color}, text::{Spans, Span},
};

fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());

    let block = Block::default()
         .title("Block")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let block = Block::default()
         .title("Block 2")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}

// UI "Components"
pub struct UiDimensionSelector {
    pub dimensions: Vec<String>,
}

pub struct UiBlockDistribution {
    pub counts: HashMap<String, u64>,
}

impl UiBlockDistribution {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    fn build_table(self) -> Table<'static> {
        Table::new(vec![
            // Row can be created from simple strings.
            Row::new(vec!["Row11", "Row12", "Row13"]),
            // You can style the entire row.
            Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
            // If you need more control over the styling you may need to create Cells directly
            Row::new(vec![
                Cell::from("Row31"),
                Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
                Cell::from(Spans::from(vec![
                    Span::raw("Row"),
                    Span::styled("33", Style::default().fg(Color::Green))
                ])),
            ]),
            // If a Row need to display some content over multiple lines, you just have to change
            // its height.
            Row::new(vec![
                Cell::from("Row\n41"),
                Cell::from("Row\n42"),
                Cell::from("Row\n43"),
            ]).height(2),
        ])
    }
}

impl Widget for UiBlockDistribution {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.build_table().render(area, buf);
    }
}
