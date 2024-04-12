use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier};
use ratatui::widgets::Borders;
use ratatui::{layout::Rect, Frame};
use std::any::Any;

use crate::tui::keymaps::KeyBindings;
use crate::tui::ui::widget_params::WidgetParams;
use crate::tui::ui::UI;

use super::Pane;

pub struct FooterPane {}

impl Pane for FooterPane {
    fn render(&self, ui: &UI, frame: &mut Frame, area: Rect, extra: Option<&dyn Any>) {
        let bindings = extra
            .and_then(|any| any.downcast_ref::<KeyBindings>())
            .expect("KeyBindings expected");

        let formatted_bindings = bindings.format_bindings();

        let footer_params = WidgetParams::new(formatted_bindings.to_string())
            .with_borders(Borders::NONE)
            .with_color(Color::DarkGray)
            .with_modifier(Modifier::ITALIC)
            .with_alignment(Alignment::Center);

        ui.render_widget(frame, area, &footer_params);
    }
}
