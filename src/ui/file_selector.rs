use ratatui::widgets::{ Block, StatefulWidget, Widget };

use crate::app::{ App };

pub struct FileSelector;

impl StatefulWidget for FileSelector {
    type State = App;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State
    )
        where Self: Sized
    {
        let block = Block::bordered().title("File Select");
        Widget::render(block, area, buf);
    }
}
