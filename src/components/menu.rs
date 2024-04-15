use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Menu {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    menu_items: Vec<String>,
    selected_index: usize,
    list_state: ListState,
}

impl Menu {
    pub fn new(menu_items: Vec<String>) -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            menu_items,
            selected_index: 0,
            list_state: state,
            ..Self::default()
        }
    }
}

impl Component for Menu {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_key_events(&mut self, key: crossterm::event::KeyEvent) -> Result<Option<Action>> {
        match key.code {
            crossterm::event::KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    self.list_state.select(Some(self.selected_index));
                }
                Ok(None)
            }
            crossterm::event::KeyCode::Down => {
                if self.selected_index < self.menu_items.len() - 1 {
                    self.selected_index += 1;
                    self.list_state.select(Some(self.selected_index));
                }
                Ok(None)
            }
            crossterm::event::KeyCode::Enter => {
                Ok(Some(Action::Select)) // Инициировать действие Select
            }
            _ => Ok(None),
        }
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Prev => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    self.list_state.select(Some(self.selected_index));
                }
            }
            Action::Next => {
                if self.selected_index < self.menu_items.len() - 1 {
                    self.selected_index += 1;
                    self.list_state.select(Some(self.selected_index));
                }
            }
            Action::Select => {
                let action = match self.menu_items[self.selected_index].as_str() {
                    "Install Neovim" => Action::InstallNeovimNightly,
                    "Install NEVIRAIDE" => Action::InstallNeviraide,
                    "Check Dependencies" => Action::CheckDependencies,
                    "Quit" => Action::Quit,
                    _ => {
                        log::warn!(
                            "Unknown menu item selected: {}",
                            self.menu_items[self.selected_index]
                        );
                        Action::Error(format!(
                            "Unknown menu item: {}",
                            self.menu_items[self.selected_index]
                        ))
                    }
                };
                if let Some(tx) = &self.command_tx {
                    tx.send(action)?;
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let items: Vec<ListItem> = self
            .menu_items
            .iter()
            .map(|item| ListItem::new(item.clone()))
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .title("Menu"),
            )
            .highlight_symbol(">> ")
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_stateful_widget(list, area, &mut self.list_state);
        Ok(())
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
