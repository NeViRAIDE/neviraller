use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use self::widgets::WidgetParams;

pub mod widgets;

pub struct UI {
    pub update_message: String,
    pub show_update_message: bool,
}

impl UI {
    pub fn new(update_message: String) -> Self {
        Self {
            update_message,
            show_update_message: false,
        }
    }

    pub fn set_update_message(&mut self, message: String) {
        self.update_message = message;
        self.show_update_message = true;
    }

    pub fn clear_update_message(&mut self) {
        self.show_update_message = false;
    }

    pub fn render_widget(&self, frame: &mut Frame, area: Rect, params: &WidgetParams) {
        let block = Block::default()
            .borders(params.borders)
            .title(params.title.clone().unwrap_or_default())
            .style(
                Style::default()
                    .fg(params.color)
                    .add_modifier(params.modifier),
            );

        let paragraph = Paragraph::new(params.text.clone())
            .block(block)
            .alignment(params.alignment)
            .style(Style::default().fg(params.color));

        frame.render_widget(paragraph, area);
    }

    // Определение методов для рендеринга header, footer и других общих элементов
    pub fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_params = WidgetParams::new("NEVIRAIDE".to_string())
            .with_title("Main Header".to_string())
            .with_color(Color::Yellow)
            .with_alignment(Alignment::Center);
        self.render_widget(frame, area, &header_params);
    }

    pub fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let footer_params = WidgetParams::new("Press 'q' or 'esc' to quit.".to_string())
            .with_title("Footer".to_string())
            .with_color(Color::Gray)
            .with_alignment(Alignment::Center);
        self.render_widget(frame, area, &footer_params);
    }

    pub fn render_additional_info(&self, frame: &mut Frame, area: Rect, info_text: &str) {
        let info_params = WidgetParams::new(info_text.to_string())
            .with_borders(Borders::ALL)
            .with_title("Additional Info".to_string())
            .with_color(Color::White)
            .with_alignment(Alignment::Left);

        self.render_widget(frame, area, &info_params);
    }
}
