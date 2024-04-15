use super::Component;
use color_eyre::eyre::Result;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

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
        // Явное указание типа для `AsRef<str>`, чтобы избежать неоднозначности
        let title_str: &str = self.title.as_ref();

        let paragraph = Paragraph::new(title_str)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::NONE))
            .style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_widget(paragraph, area);
        Ok(())
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
