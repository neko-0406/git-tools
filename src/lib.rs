use std::io;

use ratatui::{DefaultTerminal, Frame, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers}, layout::{Constraint, Direction, Layout}, prelude, style::{Color, Style}, text::Text, widgets::{Block, Borders, Paragraph, Widget}};

mod page;

#[derive(Default)]
pub struct App {
    pub github_token: String,
    pub side_width: u16,
    pub focus: FocusWidget,
    pub display_page: DisplayPage,
    pub exit: bool,
}

#[derive(Default)]
pub enum FocusWidget {
    #[default]
    SideMenu,
    MainContent
}

#[derive(Default)]
pub enum DisplayPage {
    #[default]
    Top
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
        match self.display_page {
            DisplayPage::Top => self.render_top_page(area, buf),
        }
    }
}
