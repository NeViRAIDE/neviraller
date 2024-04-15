use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Rect;
use tokio::sync::mpsc::{self};

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::info::Info;
use crate::components::menu::Menu;
use crate::components::progress::ProgressBar;
use crate::components::Component;
use crate::neovim_nightly::scrap::scrap;
use crate::neovim_nightly::ver_compare::check_neovim_version;
use crate::{action::Action, config::Config, mode::Mode, tui};

pub struct App {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub components: Vec<Box<dyn Component>>,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let menu = Menu::new(vec![
            "Install Neovim".to_string(),
            "Install NEVIRAIDE".to_string(),
            "Check dependencies".to_string(),
            "Quit".to_string(),
        ]);
        let header = Header::new("NEVIRALLER");
        let footer = Footer::new("© 2024 RAprogramm");
        let info = Info::new(" Info component ");
        let progress_bar = ProgressBar::new();
        let config = Config::new()?;
        let mode = Mode::Home;

        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![
                Box::new(header),
                Box::new(menu),
                Box::new(progress_bar), // Добавление ProgressBar в приложение
                Box::new(info),
                Box::new(footer),
            ],
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            last_tick_key_events: Vec::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?;
        tui.enter()?;

        // Определение размеров и создание разделов экрана
        let size = tui.size()?;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(10),
                Constraint::Length(1),
            ])
            .split(size);

        // Разделение основного меню на две части
        let middle_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 50% для меню
                Constraint::Percentage(70), // 50% для другого контента
            ])
            .split(chunks[1]);

        let info_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // ProgressBar
                Constraint::Min(1),    // Info
            ])
            .split(middle_chunks[1]);

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
            component.register_config_handler(self.config.clone())?;
            // let area = if i == 1 { middle_chunks[0] } else { chunks[i] };
            // component.init(area)?;
        }
        self.components[0].init(chunks[0])?; // Header
        self.components[1].init(middle_chunks[0])?; // Menu
        self.components[2].init(info_chunks[0])?; // ProgressBar
        self.components[3].init(info_chunks[1])?; // Info
        self.components[4].init(chunks[2])?; // Fo

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(Action::Quit)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key) => {
                        if let Some(keymap) = self.config.keybindings.get(&self.mode) {
                            if let Some(action) = keymap.get(&vec![key]) {
                                log::info!("Got action: {action:?}");
                                action_tx.send(action.clone())?;
                            } else {
                                // If the key was not handled as a single key action,
                                // then consider it for multi-key combinations.
                                self.last_tick_key_events.push(key);

                                // Check for multi-key combinations
                                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                                    log::info!("Got action: {action:?}");
                                    action_tx.send(action.clone())?;
                                }
                            }
                        };
                    }
                    _ => {}
                }

                for component in self.components.iter_mut() {
                    if let Some(action) = component.handle_events(Some(e.clone()))? {
                        action_tx.send(action)?;
                    }
                }
            }

            // self.app_actions(action_rx, action_tx, tui, chunks, middle_chunks);

            while let Ok(action) = action_rx.try_recv() {
                if action != Action::Tick && action != Action::Render {
                    log::debug!("{action:?}");
                }
                match action {
                    Action::InstallNeovimNightly => {
                        self.update_info("Checking current Neovim version...");
                        let total_steps = 10.0;
                        for step in 0..=total_steps as usize {
                            let progress = step as f64 / total_steps;
                            self.components[2]
                                .as_any()
                                .downcast_mut::<ProgressBar>()
                                .unwrap()
                                .update_progress(progress);

                            // Обновление и перерисовка всего TUI, не только ProgressBar
                            tui.draw(|f| {
                                self.components.iter_mut().for_each(|comp| {
                                    let area = match comp.as_any().type_id() {
                                        _ if comp.as_any().is::<Header>() => chunks[0],
                                        _ if comp.as_any().is::<Menu>() => middle_chunks[0],
                                        _ if comp.as_any().is::<ProgressBar>() => info_chunks[0],
                                        _ if comp.as_any().is::<Info>() => info_chunks[1],
                                        _ if comp.as_any().is::<Footer>() => chunks[2],
                                        _ => unimplemented!(),
                                    };
                                    comp.draw(f, area).unwrap(); // Убедитесь, что каждый компонент корректно отрисовывается в своей области
                                });
                            })?;

                            // Имитация задержки для видимости изменений прогресса
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        }

                        match scrap().await {
                            Ok(ver) => match check_neovim_version(&ver).await {
                                Ok(_) => self.update_info("Neovim is up to date."),
                                Err(e) => {
                                    self.update_info(&format!("Error updating Neovim: {}", e))
                                }
                            },
                            Err(e) => self.update_info(&format!("Failed to check version: {}", e)),
                        }
                    }
                    Action::InstallNeviraide => {
                        self.update_info("Installing NEVIRAIDE...");
                    }
                    Action::CheckDependencies => {
                        self.update_info("Checking system dependencies...");
                        log::debug!("Starting dependencies check...");
                    }
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        let size = tui.size()?;
                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .margin(1)
                            .constraints([
                                Constraint::Length(3),
                                Constraint::Min(10),
                                Constraint::Length(3),
                            ])
                            .split(size);

                        let middle_chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Percentage(50), // 50% для меню
                                Constraint::Percentage(50), // 50% для Info
                            ])
                            .split(chunks[1]);

                        let info_chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3), // ProgressBar
                                Constraint::Min(1),    // Info
                            ])
                            .split(middle_chunks[1]);

                        // Переинициализация компонентов
                        self.components[0].init(chunks[0])?; // Header
                        self.components[1].init(middle_chunks[0])?; // Menu
                        self.components[2].init(info_chunks[0])?; // ProgressBar
                        self.components[3].init(info_chunks[1])?; // Info
                        self.components[4].init(chunks[2])?; // Fo
                    }
                    Action::Render => {
                        // Перерисовка интерфейса
                        // tui.draw(|f| {
                        //     self.components[0].draw(f, chunks[0]); // Header
                        //     self.components[1].draw(f, middle_chunks[0]); // Menu
                        //     self.components[2].draw(f, info_chunks[0]); // ProgressBar
                        //     self.components[3].draw(f, info_chunks[1]); // Info
                        //     self.components[4].draw(f, chunks[2]); // Fo
                        // })?;
                        self.update_ui(&mut tui);
                    }
                    _ => {}
                }

                for component in self.components.iter_mut() {
                    if let Some(action) = component.update(action.clone())? {
                        action_tx.send(action)?
                    };
                }
            }

            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                tui = tui::Tui::new()?;
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    fn update_info(&mut self, message: &str) {
        log::debug!("Updating info with message: {}", message);
        if let Some(info) = self
            .components
            .iter_mut()
            .find_map(|component| component.as_any().downcast_mut::<Info>())
        {
            info.update_content(message);
        }
    }

    fn update_ui(&mut self, tui: &mut tui::Tui) -> Result<()> {
        let size = tui.size()?;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(10),
                Constraint::Length(1),
            ])
            .split(size);

        let middle_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(chunks[1]);

        let info_chunks = if self.components[2]
            .as_any()
            .downcast_ref::<ProgressBar>()
            .unwrap()
            .visible
        {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1)])
                .split(middle_chunks[1])
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(0), Constraint::Min(1)])
                .split(middle_chunks[1])
        };

        tui.draw(|f| {
            self.components.iter_mut().for_each(|comp| {
                let area = if comp.as_any().is::<Header>() {
                    chunks[0]
                } else if comp.as_any().is::<Menu>() {
                    middle_chunks[0]
                } else if comp.as_any().is::<ProgressBar>() {
                    info_chunks[0]
                } else if comp.as_any().is::<Info>() {
                    info_chunks[1]
                } else if comp.as_any().is::<Footer>() {
                    chunks[2]
                } else {
                    unimplemented!()
                };
                comp.draw(f, area).unwrap(); // Убедитесь, что каждый компонент корректно отрисовывается в своей области
            });
        })?;

        Ok(())
    }
}
