use super::Component;
use crate::{/* config::Config, */ tui::Frame};
use color_eyre::eyre::Result;
use ratatui::widgets::{Block, Borders, Wrap};
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
        let paragraph = Paragraph::new(self.content.clone())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::NONE))
            .style(
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::ITALIC),
            );
        f.render_widget(paragraph, area);
        Ok(())
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
