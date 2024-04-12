use std::any::Any;

use ratatui::layout::Alignment;
use ratatui::style::Color;
use ratatui::{layout::Rect, Frame};

use crate::tui::ui::widget_params::WidgetParams;
use crate::tui::ui::UI;

use super::Pane;

pub struct HeaderPane {}

impl Pane for HeaderPane {
    fn render(&self, ui: &UI, frame: &mut Frame, area: Rect, _extra: Option<&dyn Any>) {
        let header_params = WidgetParams::new("NEVIRALLER".to_string())
            .with_color(Color::Yellow)
            .with_alignment(Alignment::Center);
        ui.render_widget(frame, area, &header_params);
    }
}
