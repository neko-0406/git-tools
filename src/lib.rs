use std::io;

use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, layout::{Constraint, Direction, Layout}, prelude, style::{Color, Style}, text::Text, widgets::{Block, Borders, Paragraph, Widget}};

#[derive(Default)]
pub struct App {
    github_token: String,
    side_width: u16,
    focus: FocusWidget,
    exit: bool,
}

#[derive(Default)]
pub enum FocusWidget {
    #[default]
    SideMenu,
    MainContent
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // 環境変数の読み込み
        dotenv::dotenv().ok();
        // GitHubのアクセストークン取得・無ければアプリを落とす
        self.github_token =
            dotenv::var("GITHUB_TOKEN").expect("GitHubへのアクセストークンが見つかりませんでした");
        self.side_width = 20;
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Right => self.focus_main(),
                KeyCode::Left => self.focus_side(),
                _ => {}
            }
        }
    }
}

impl App {
    fn exit(&mut self) {
        self.exit = true;
    }
    fn focus_side(&mut self) {
        self.focus = FocusWidget::SideMenu;
    }
    fn focus_main(&mut self) {
        self.focus = FocusWidget::MainContent;
    }
}

impl Widget for &App {
    fn render(self, area: prelude::Rect, buf: &mut prelude::Buffer) {
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
