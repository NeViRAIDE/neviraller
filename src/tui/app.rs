use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{poll, read, Event};
use tokio::{sync::mpsc, time};

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use super::{
    keymaps::KeyBindings,
    menu::{self, Menu, MenuAction},
    ui::UI,
};

// use self::neovim_nightly::{
//     scrap::scrap, update::update_neovim, ver_compare::check_neovim_version,
// };

pub struct App {
    menu: Menu,
    ui: UI,
    should_quit: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Action {
    Select,
    Next,
    Previous,
    Quit,
}

impl App {
    pub fn new(keys: KeyBindings) -> App {
        App {
            menu: Menu::new(menu::get_menu_items()),
            ui: UI::new("Initial update message".to_string(), keys), 
            should_quit: false,
        }
    }

    pub async fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        mut rx: mpsc::UnboundedReceiver<Action>,
    ) -> Result<()> {
        loop {
            terminal.draw(|f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                        Constraint::Length(3),
                    ])
                    .split(size);

                self.ui.render_header(f, chunks[0]);
                self.ui.render_menu(f, chunks[1], &self.menu);
                self.ui
                    .render_additional_info(f, chunks[2], &self.ui.update_message);
                self.ui.render_footer(f, chunks[3]);
            })?;

            if let Some(action) = rx.recv().await {
                match action {
                    Action::Select => {
                        let selected_action = self.menu.select();
                        self.handle_action(selected_action);
                    }
                    Action::Next => self.menu.next(),
                    Action::Previous => self.menu.previous(),
                    Action::Quit => self.should_quit = true,
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_action(&mut self, action: MenuAction) {
        match action {
            MenuAction::InstallNeovimNightly => {
                self.ui
                    .set_update_message("установка обновлений Neovim...".to_string());
            }
            MenuAction::CheckForUpdates => {
                self.ui
                    .set_update_message("Проверка доступных обновлений...".to_string());
            }
            MenuAction::CheckDependencies => {
                self.ui
                    .set_update_message("Check dependencies...".to_string());
            }
            MenuAction::Quit => {
                self.should_quit = true;
            }
        }
    }
}

pub async fn event_handler(tx: mpsc::UnboundedSender<Action>, key_bindings: KeyBindings) {
    let mut interval = time::interval(Duration::from_millis(100));
    loop {
        interval.tick().await;
        if let Ok(true) = poll(Duration::from_millis(0)) {
            match read() {
                Ok(Event::Key(key)) => {
                    if let Some(action) = key_bindings.get_action(key.code) {
                        if let Err(e) = tx.send(action.clone()) {
                            eprintln!("Error sending action: {:?}", e);
                            break;
                        }
                    }
                }
                Ok(Event::Mouse(_)) => {
                    // Обработка событий мыши
                }
                Ok(Event::Resize(_, _)) => {
                    // Обработка изменения размера терминала
                }
                Ok(Event::FocusGained) | Ok(Event::FocusLost) => {
                    // Обработка событий фокуса
                }
                Ok(Event::Paste(_)) => {
                    // Обработка вставки из буфера обмена
                }
                Err(e) => {
                    eprintln!("Error reading event: {:?}", e);
                    continue;
                } // _ => {} // Обработка других неучтенных событий
            }
        }
    }
}
