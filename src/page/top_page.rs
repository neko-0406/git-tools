use ratatui::{layout::{Constraint, Direction, Layout}, prelude, style::Color, widgets::{Block, Borders, Paragraph, Widget}};

use crate::{App, FocusWidget};

impl App {
    pub fn render_top_page(&self, area: prelude::Rect, buf: &mut prelude::Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(self.side_width),
                Constraint::Percentage(100 - self.side_width),
            ])
            .split(area);

        Paragraph::new("side")
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_style(
                        match self.focus {
                            FocusWidget::SideMenu => Color::Cyan,
                            _ => Color::default()
                        }
                    )
            )
            .render(layout[0], buf);

        Paragraph::new("main")
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_style(
                        match self.focus {
                            FocusWidget::MainContent => Color::Cyan,
                            _ => Color::default()
                        }
                    )
            )
            .render(layout[1], buf);
    }
}