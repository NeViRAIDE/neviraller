use super::Component;
use crate::{/* config::Config, */ tui::Frame};
use color_eyre::eyre::Result;
use ratatui::widgets::{Block, Borders};
use ratatui::{prelude::*, widgets::Paragraph};

pub struct Footer {
    content: String,
}

impl Footer {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

impl Component for Footer {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let paragraph =
            Paragraph::new(self.content.clone()).block(Block::default().borders(Borders::ALL));
        f.render_widget(paragraph, area);
        Ok(())
    }
}
