use std::any::Any;

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use self::{
    panes::{footer::FooterPane, header::HeaderPane, menu::MenuPane, Pane},
    widget_params::WidgetParams,
};

use super::{keymaps::KeyBindings, menu::Menu};

pub mod panes;
pub mod widget_params;

#[derive(Clone)]
pub struct UI {
    pub update_message: String,
    pub show_update_message: bool,
    pub key_bindings: KeyBindings,
}

impl UI {
    pub fn new(update_message: String, key_bindings: KeyBindings) -> Self {
        Self {
            update_message,
            show_update_message: false,
            key_bindings,
        }
    }

    // pub fn set_update_message(&mut self, message: String) {
    //     self.update_message = message;
    //     self.show_update_message = true;
    // }
    pub fn set_update_message(&mut self, message: String) {
        // Добавляем новое сообщение, разделяя их переносом строки, если уже есть какой-то текст
        self.update_message = if self.update_message.is_empty() {
            message
        } else {
            format!("{}\n{}", self.update_message, message)
        };
        self.show_update_message = true;
    }

    pub fn _clear_update_message(&mut self) {
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

    pub fn render_header(&self, frame: &mut Frame, area: Rect) {
        let header_pane = HeaderPane {};
        header_pane.render(self, frame, area, None);
    }

    pub fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let footer_pane = FooterPane {};
        footer_pane.render(self, frame, area, Some(&self.key_bindings as &dyn Any));
    }

    pub fn render_menu(&self, frame: &mut Frame, area: Rect, menu: &Menu) {
        let menu_pane = MenuPane {};
        menu_pane.render(self, frame, area, Some(menu as &dyn Any));
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
