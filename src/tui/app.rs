use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{poll, read, Event, KeyCode};
use tokio::{sync::mpsc, time};

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    Frame, Terminal,
};

use super::{
    menu::{self, Menu, MenuAction},
    ui::{widgets::WidgetParams, UI},
};

// use self::neovim_nightly::{
//     scrap::scrap, update::update_neovim, ver_compare::check_neovim_version,
// };

pub struct App {
    menu: Menu,
    ui: UI,
    should_quit: bool,
}

pub enum Action {
    Select,
    Next,
    Previous,
    Quit,
}

impl App {
    pub fn new() -> App {
        App {
            menu: Menu::new(menu::get_menu_items()),
            ui: UI::new("Initial update message".to_string()), // Инициализация UI
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
                self.render_main_area(f, chunks[1]);
                self.ui
                    .render_additional_info(f, chunks[2], &self.ui.update_message);
                self.ui.render_footer(f, chunks[3]);

                // let info_area = Rect::new(0, size.height / 2, size.width, size.height / 4); // Пример области для дополнительной информации
                //
                // self.ui.render_additional_info(
                //     f,
                //     info_area,
                //     "Here is some additional information or logs.",
                // );
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

    fn render_main_area(&self, frame: &mut Frame, area: Rect) {
        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(area);

        self.menu.render(frame, columns[0]);
        // Пример использования UI для отрисовки основного содержимого
        let content_params =
            WidgetParams::new("Here is the main content of the application".to_string())
                .with_title("Content".to_string())
                .with_color(Color::White);
        self.ui.render_widget(frame, columns[1], &content_params);
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

pub async fn event_handler(tx: mpsc::UnboundedSender<Action>) {
    let mut interval = time::interval(Duration::from_millis(100));
    loop {
        interval.tick().await;
        if let Ok(true) = poll(Duration::from_millis(0)) {
            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    KeyCode::Char('j' | 'n') | KeyCode::Down => tx.send(Action::Next).unwrap(),
                    KeyCode::Char('k' | 'p') | KeyCode::Up => tx.send(Action::Previous).unwrap(),
                    KeyCode::Enter => tx.send(Action::Select).unwrap(),
                    KeyCode::Char('q') | KeyCode::Esc => tx.send(Action::Quit).unwrap(),
                    _ => {}
                }
            }
        }
    }
}
