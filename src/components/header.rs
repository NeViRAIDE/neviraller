use super::Component;
use crate::{/* config::Config, */ tui::Frame};
use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::Block, widgets::Borders};

pub struct Header {
    title: String,
}

impl Header {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

impl Component for Header {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL);
        f.render_widget(block, area);
        Ok(())
    }
}
