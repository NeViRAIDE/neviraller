use std::any::Any;

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Color;
use ratatui::{layout::Rect, Frame};

use crate::tui::menu::Menu;
use crate::tui::ui::widget_params::WidgetParams;
use crate::tui::ui::UI;

use super::Pane;

pub struct MenuPane {}

impl Pane for MenuPane {
    fn render(&self, ui: &UI, frame: &mut Frame, area: Rect, extra: Option<&dyn Any>) {
        let menu = extra
            .and_then(|any| any.downcast_ref::<Menu>())
            .expect("MenuPane requires a Menu reference");
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        menu.render(frame, columns[0]);
        let content_params =
            WidgetParams::new("Here is the main content of the application".to_string())
                .with_title("Content".to_string())
                .with_color(Color::White);
        ui.render_widget(frame, columns[1], &content_params);
    }
}
