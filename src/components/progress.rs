use crate::{components::Component, tui::Frame};
use color_eyre::eyre::Result;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Gauge},
};
use std::any::Any;

pub struct ProgressBar {
    pub progress: f64, // Прогресс от 0.0 до 1.0
    pub visible: bool, // Управление видимостью ProgressBar
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgressBar {
    pub fn new() -> Self {
        ProgressBar {
            progress: 0.0,
            visible: false,
        }
    }

    /// Обновляет прогресс бар до нового значения.
    pub fn update_progress(&mut self, progress: f64) {
        self.progress = progress;
        self.visible = progress > 0.0 && progress < 1.0; // Видим только если прогресс не на 0% и не на 100%
    }
}

impl Component for ProgressBar {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        if self.visible {
            let gauge = Gauge::default()
                .block(
                    Block::default()
                        .title(" Progress ")
                        .borders(ratatui::widgets::Borders::ALL),
                )
                .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
                .ratio(self.progress);

            f.render_widget(gauge, area);
        }
        Ok(())
    }
}
