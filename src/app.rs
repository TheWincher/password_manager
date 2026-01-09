use std::io;
use crossterm::event::{ self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers };
use ratatui::{
    DefaultTerminal,
    Frame,
    layout::{ Alignment, Constraint, Direction, Layout, Rect },
    style::Stylize,
    symbols::border,
    text::{ Line, Text },
    widgets::{ Block, Borders, Clear, Padding, Paragraph, Widget },
};

use crate::vault::Vault;

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    show_master_password_popup: bool,
    input: Input,
    vault: Option<Vault>,
}

impl App {
    pub fn new() -> Self {
        App {
            exit: false,
            show_master_password_popup: true,
            input: Input::new(),
            vault: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        if self.show_master_password_popup {
            draw_popup(frame, centered_rect(80, 5, frame.area()), &self.input);
        }
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
            KeyCode::Esc => {
                self.show_master_password_popup = false;
            }
            KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.exit = true;
            }
            KeyCode::Char(c) if self.show_master_password_popup => {
                self.input.value.insert(self.input.cursor, c);
                self.input.cursor += 1;
            }
            KeyCode::Backspace if self.show_master_password_popup => {
                if self.input.cursor > 0 {
                    self.input.cursor -= 1;
                    self.input.value.remove(self.input.cursor);
                }
            }
            _ => {}
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[derive(Debug, Default)]
struct Input {
    value: String,
    cursor: usize,
}

impl Input {
    fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
        }
    }
}

fn draw_popup(f: &mut Frame, area: Rect, input: &Input) {
    // Important : Clear efface le fond
    f.render_widget(Clear, area);

    let block = Block::default()
        .title("Master password")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));
    let paragraph = Paragraph::new("*".repeat(input.value.len()))
        .block(block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
    // Position du curseur
    let x = area.x + (input.cursor as u16) + 2;
    let y = area.y + 1;

    f.set_cursor_position((x, y));
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where Self: Sized
    {
        let title = Line::from(" Password Manager ".bold());

        let block = Block::bordered().title(title.centered()).border_set(border::THICK);

        let text = Text::from("Test");

        Paragraph::new(text).centered().block(block).render(area, buf);
    }
}
