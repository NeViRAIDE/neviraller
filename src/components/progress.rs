use crate::{components::Component, tui::Frame};
use color_eyre::eyre::Result;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Gauge},
};
use std::any::Any;

pub struct ProgressBar {
    pub progress: f64,
    pub visible: bool,
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

    pub fn update_progress(
        &mut self,
        progress: f64,
        action_tx: &tokio::sync::mpsc::UnboundedSender<crate::action::Action>,
    ) -> Result<()> {
        self.progress = progress / 100.0;
        self.visible = progress > 0.0 && progress < 100.0;

        action_tx.send(crate::action::Action::Render)?;

        Ok(())
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
                        .borders(ratatui::widgets::Borders::ALL)
                        .border_set(ratatui::symbols::border::ROUNDED),
                )
                .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
                .ratio(self.progress);

            f.render_widget(gauge, area);
        }
        Ok(())
    }
}
