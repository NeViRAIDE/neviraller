use super::Component;
use crate::tui::Frame;
use color_eyre::eyre::Result;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::{prelude::*, widgets::Block, widgets::Borders};

pub struct Info {
    title: String,
    content: String, // Добавляем поле для хранения дополнительного контента
}

impl Info {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: String::new(), // Инициализируем пустым содержимым
        }
    }

    // Метод для обновления содержимого
    pub fn update_content(&mut self, new_content: &str) {
        self.content.push_str(new_content);
        self.content.push('\n');
    }
}

impl Component for Info {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let title_str: &str = &self.title;
        let content_str: &str = &self.content;

        let block = Block::default()
            .title(title_str)
            .borders(Borders::ALL)
            .border_set(ratatui::symbols::border::ROUNDED);
        let scroll_offset = self
            .content
            .matches('\n')
            .count()
            .saturating_sub(area.height as usize); // Автоматическая прокрутка к последнему сообщению

        let paragraph = Paragraph::new(content_str)
            .block(block)
            .wrap(Wrap { trim: true })
            .scroll((scroll_offset as u16, 0)); // Прокрутка для показа новых сообщений

        f.render_widget(paragraph, area);
        Ok(())
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
