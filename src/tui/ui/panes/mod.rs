use std::any::Any;
use ratatui::layout::Rect;
use ratatui::Frame;

use super::UI;

pub mod footer;
pub mod header;
pub mod menu;

pub trait Pane {
    fn render(&self, ui: &UI, frame: &mut Frame, area: Rect, extra: Option<&dyn Any>);
}
