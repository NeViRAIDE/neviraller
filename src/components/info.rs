use super::Component;
use crate::{/* config::Config, */ tui::Frame};
use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::Block, widgets::Borders};

pub struct Info {
    title: String,
}

impl Info {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

impl Component for Info {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL);
        f.render_widget(block, area);
        Ok(())
    }
}
