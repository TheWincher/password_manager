use std::path::PathBuf;

use crossterm::event::{ KeyCode, KeyModifiers };
use ratatui::{ style::Style, widgets::{ Block, List, ListState, StatefulWidget, Tabs, Widget } };

use crate::{ app::{ App, AppState, FocusedWidget, Message }, ui::file_selector::{ FileSelector } };

#[derive(Debug)]
pub struct CenterLeftWidgetState {
    list_state: ListState,
    pub current_dir: PathBuf,
    pub selected_file: Option<PathBuf>,
}

impl CenterLeftWidgetState {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)),
            current_dir: PathBuf::from("/"),
            selected_file: None,
        }
    }
}

#[derive(Debug)]
pub struct CenterLeftWidget;

impl CenterLeftWidget {
    pub fn handle_key_event(
        key_event: crossterm::event::KeyEvent,
        app_state: &App
    ) -> Option<Message> {
        match key_event.code {
            KeyCode::Right if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Message::SetFocusedWidget(FocusedWidget::CenterRight))
            }
            KeyCode::Up if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                Some(Message::SetFocusedWidget(FocusedWidget::Top))
            }
            KeyCode::Enter if app_state.state == AppState::NoConfigFound => {
                Some(Message::UpdateState(AppState::CreateNewVault))
            }
            KeyCode::Up if app_state.state == AppState::NoConfigFound => {
                Some(Message::SetFocusedWidget(FocusedWidget::Top))
            }
            // KeyCode::Enter if app_state.state == AppState::CreateNewVault => {
            //     Some(Message::EnterDir(app_state.center_left_state.current_dir.clone().join(path)))
            // }
            _ => None,
        }
    }
}

impl StatefulWidget for CenterLeftWidget {
    type State = App;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State
    )
        where Self: Sized
    {
        let style = if state.focused_widget == FocusedWidget::CenterLeft {
            Style::new().blue()
        } else {
            Style::new()
        };

        let center_left_block = Block::bordered().title("Vault").border_style(style);

        match state.state {
            AppState::NoConfigFound => {
                let list = List::new(["Create new vault", "Open existing vault"])
                    .block(center_left_block)
                    .highlight_style(Style::new().white());

                StatefulWidget::render(list, area, buf, &mut state.center_left_state.list_state);
            }
            AppState::CreateNewVault => {
                let file_selector = FileSelector;
                file_selector.render(area, buf, state);
            }
            _ => {}
        }
    }
}
