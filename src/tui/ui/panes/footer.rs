use std::any::Any;

use ratatui::layout::Alignment;
use ratatui::style::Color;
use ratatui::{layout::Rect, Frame};

use crate::tui::ui::widget_params::WidgetParams;
use crate::tui::ui::UI;

use super::Pane;

pub struct FooterPane {
    // Параметры и состояние подвала
}

impl Pane for FooterPane {
    fn render(&self, ui: &UI, frame: &mut Frame, area: Rect, _extra: Option<&dyn Any>) {
        let footer_params = WidgetParams::new("Press 'q' or 'esc' to quit.".to_string())
            .with_color(Color::Gray)
            .with_alignment(Alignment::Center);
        ui.render_widget(frame, area, &footer_params);
    }
}
