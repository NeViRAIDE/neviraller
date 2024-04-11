use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{poll, read, Event, KeyCode};
use tokio::{sync::mpsc, time};

use ratatui::{
    widgets::{Block, Borders},
    Frame, Terminal,
};

use super::menu::{self, Menu, MenuAction};

// use self::neovim_nightly::{
//     scrap::scrap, update::update_neovim, ver_compare::check_neovim_version,
// };

pub struct App {
    menu: Menu,
    show_update_message: bool,
    update_message: String,
    should_quit: bool,
}

pub enum Action {
    Select,
    Next,
    Previous,
}

impl App {
    pub fn new() -> App {
        App {
            menu: Menu::new(menu::get_menu_items()),
            show_update_message: false,
            update_message: String::new(),
            should_quit: false,
        }
    }

    pub async fn run<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        mut rx: mpsc::Receiver<Action>,
    ) -> Result<()> {
        loop {
            terminal.draw(|f| {
                self.menu.render(f); // Использование метода render для отрисовки меню

                if self.show_update_message {
                    self.render_update_message(f, f.size()); // Теперь render_update_message - метод App
                }
            })?;

            if let Some(action) = rx.recv().await {
                match action {
                    Action::Select => {
                        let selected_action = self.menu.select(); // Получение выбранного действия из меню
                        self.handle_action(selected_action); // Обработка выбранного действия
                    }
                    Action::Next => self.menu.next(), // Переход к следующему элементу меню
                    Action::Previous => self.menu.previous(), // Переход к предыдущему элементу меню
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
                self.update_message = "установка обновлений Neovim...".to_string();
                self.show_update_message = true;
            }
            MenuAction::CheckForUpdates => {
                self.update_message = "Проверка доступных обновлений...".to_string();
                self.show_update_message = true;
            }
            MenuAction::CheckDependencies => {
                self.update_message = "Check dependencies...".to_string();
                self.show_update_message = true;
            }
            MenuAction::Quit => {
                self.should_quit = true;
            }
        }
    }

    fn render_update_message(&self, frame: &mut Frame, area: ratatui::layout::Rect) {
        if self.show_update_message {
            let message_block = Block::default()
                .title("Update Information")
                .borders(Borders::ALL)
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::LightCyan));
            let message_area =
                ratatui::layout::Rect::new(area.x + 40, area.y + 2, area.width / 2, 3);
            let paragraph = ratatui::widgets::Paragraph::new(self.update_message.clone())
                .block(message_block)
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(paragraph, message_area);
        }
    }
}

pub async fn event_handler(tx: mpsc::Sender<Action>) {
    let mut interval = time::interval(Duration::from_millis(100));
    loop {
        interval.tick().await;
        if let Ok(true) = poll(Duration::from_millis(0)) {
            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    KeyCode::Char('j' | 'n') | KeyCode::Down => {
                        tx.send(Action::Next).await.unwrap()
                    }
                    KeyCode::Char('k' | 'p') | KeyCode::Up => {
                        tx.send(Action::Previous).await.unwrap()
                    }
                    KeyCode::Enter => tx.send(Action::Select).await.unwrap(),
                    _ => {}
                }
            }
        }
    }
}
