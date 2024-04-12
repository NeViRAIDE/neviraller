use ratatui::{
    layout::Alignment,
    style::{Color, Modifier},
    widgets::Borders,
};

pub struct WidgetParams {
    pub text: String,
    pub alignment: Alignment,
    pub borders: Borders,
    pub title: Option<String>,
    pub color: Color,
    pub modifier: Modifier,
}

impl WidgetParams {
    pub fn new(text: String) -> Self {
        Self {
            text,
            alignment: Alignment::Left,
            borders: Borders::ALL,
            title: None,
            color: Color::White,
            modifier: Modifier::empty(),
        }
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn _with_modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = modifier;
        self
    }
}
