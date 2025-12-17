use std::io;

use ratatui::{DefaultTerminal, Frame, prelude, widgets::Widget};

#[derive(Default)]
pub struct App {
    github_token: String,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // 環境変数の読み込み
        dotenv::dotenv().ok();
        // GitHubのアクセストークン取得・無ければアプリを落とす
        self.github_token =
            dotenv::var("GITHUB_TOKEN").expect("GitHubへのアクセストークンが見つかりませんでした");
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events();
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) {}
}

impl Widget for &App {
    fn render(self, area: prelude::Rect, buf: &mut prelude::Buffer) {}
}
