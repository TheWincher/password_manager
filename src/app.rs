use std::io;
use crossterm::event::{self, Event, KeyEventKind, KeyCode, KeyEvent};
use ratatui::{DefaultTerminal, Frame, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}};

#[derive(Debug, Default)]
pub struct App {
    exit: bool
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
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
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                self.handle_key_event(event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let title = Line::from(" Password Manager ".bold());

        let block = Block::bordered().title(title.centered()).border_set(border::THICK);

        let text = Text::from("Test");

        Paragraph::new(text).centered().block(block).render(area, buf);
    }
}