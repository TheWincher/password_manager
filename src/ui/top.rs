use crossterm::event::{ KeyCode, KeyModifiers };
use ratatui::{ style::Style, widgets::{ Block, StatefulWidget, Tabs, Widget } };

use crate::app::{ App, FocusedWidget, Message };

#[derive(Debug)]
pub struct TopWidgetState {
    pub selected_tab: usize,
}

impl TopWidgetState {
    pub fn new() -> Self {
        Self { selected_tab: 0 }
    }
}

#[derive(Debug)]
pub struct TopWidget;

impl TopWidget {
    pub fn handle_key_event(key_event: crossterm::event::KeyEvent) -> Option<Message> {
        match key_event.code {
            KeyCode::Right => { Some(Message::NextTab) }
            KeyCode::Left => { Some(Message::PreviousTab) }
            KeyCode::Down if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Message::SetFocusedWidget(FocusedWidget::CenterLeft))
            }
            _ => None,
        }
    }
}

impl StatefulWidget for TopWidget {
    type State = App;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State
    )
        where Self: Sized
    {
        let style = if state.focused_widget == FocusedWidget::Top {
            Style::new().blue()
        } else {
            Style::new()
        };

        let top_block = Block::bordered().title("Menu").border_style(style);

        Tabs::new(["Vault", "Config"])
            .select(state.top_state.selected_tab)
            .block(top_block)
            .render(area, buf);
    }
}
