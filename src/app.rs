use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::components::input::Input;
use crate::components::{
    footer::Footer, header::Header, info::Info, menu::Menu, progress::ProgressBar, Component,
};
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
        let config = Config::new()?;
        let mode = Mode::Home;

        Ok(Self {
            tick_rate,
            frame_rate,
            components: vec![
                Box::new(Header::new("NEVIRALLER")),
                Box::new(Menu::new(crate::components::menu::default_menu_items())),
                Box::new(ProgressBar::new()),
                Box::new(Info::new(" Info component ")),
                Box::new(Input::new(" Your choise ")),
                Box::new(Footer::new("© 2024 RAprogramm")),
            ],
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            last_tick_key_events: Vec::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?;
        tui.enter()?;

        for component in self.components.iter_mut() {
            component.register_action_handler(action_tx.clone())?;
            component.register_config_handler(self.config.clone())?;
        }

        self.update_ui(&mut tui).await?;

        self.event_loop(&mut tui, action_tx, action_rx).await?;

        tui.exit()?;
        Ok(())
    }

    async fn process_event(
        &mut self,
        event: tui::Event,
        action_tx: &UnboundedSender<Action>,
        tui: &mut tui::Tui, // Добавляем tui как аргумент
    ) -> Result<()> {
        match event {
            tui::Event::Quit => action_tx.send(Action::Quit)?,
            tui::Event::Render | tui::Event::Resize(_, _) => {
                self.update_ui(tui).await?; // Теперь мы обрабатываем UI обновления прямо здесь
            }
            tui::Event::Key(key) => self.handle_key_event(key, action_tx).await,
            _ => {}
        }
        Ok(())
    }

    async fn process_action(
        &mut self,
        action: Action,
        tui: &mut tui::Tui,
        action_tx: &UnboundedSender<Action>,
    ) -> Result<()> {
        match action {
            Action::Render => self.update_ui(tui).await?,
            Action::Resume => {
                // Пересоздаем экземпляр Tui
                let new_tui = tui::Tui::new()?;
                *tui = new_tui;
                tui.enter()?; // Убедитесь, что терминал правильно настроен
                self.update_ui(tui).await?; // Перерисовываем UI
            }
            // Обрабатывайте другие действия...
            _ => self.handle_specific_action(action, tui, action_tx).await?,
        }
        Ok(())
    }

    async fn event_loop(
        &mut self,
        tui: &mut tui::Tui,
        action_tx: UnboundedSender<Action>,
        mut action_rx: UnboundedReceiver<Action>,
    ) -> Result<()> {
        loop {
            if let Some(e) = tui.next().await {
                self.process_event(e, &action_tx, tui).await?;
            }

            while let Ok(action) = action_rx.try_recv() {
                self.process_action(action, tui, &action_tx).await?;
            }

            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                let tui = &mut tui::Tui::new()?;
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
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

    async fn update_ui(&mut self, tui: &mut tui::Tui) -> Result<()> {
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
                } else if comp.as_any().is::<Input>() {
                    info_chunks[1]
                } else if comp.as_any().is::<Footer>() {
                    chunks[2]
                } else {
                    unimplemented!()
                };
                comp.draw(f, area).unwrap();
            });
        })?;

        Ok(())
    }

    async fn handle_key_event(&mut self, key: KeyEvent, action_tx: &UnboundedSender<Action>) {
        log::info!("Key event received: {:?}", key);
        // self.last_tick_key_events.push(key);

        if let Some(keymap) = self.config.keybindings.get(&self.mode) {
            if let Some(action) = keymap.get(&vec![key]) {
                log::info!("Action found for sequence: {action:?}");
                action_tx
                    .send(action.clone())
                    .expect("Failed to send action");
            } else {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    log::info!("Got action: {action:?}");
                    if let Err(r) = action_tx.send(action.clone()) {
                        log::error!("Unable to got action: {}", r);
                    }
                }
            }
        } else {
            log::warn!("No keybindings found for current mode: {:?}", self.mode);
            self.last_tick_key_events.clear();
        }
    }

    async fn handle_specific_action(
        &mut self,
        action: Action,
        tui: &mut tui::Tui,
        action_tx: &UnboundedSender<Action>,
    ) -> Result<()> {
        if action != Action::Tick && action != Action::Render {
            log::debug!("{action:?}");
        }

        match action {
            Action::InstallNeovimNightly => {
                log::info!("Starting installation of Neovim Nightly");
                self.update_info("Installing Neovim Nightly...");
                let available_ver = scrap().await.unwrap();
                let check_ver = check_neovim_version(&available_ver).await.unwrap();

                let total_steps = 10;
                for step in 0..=total_steps {
                    if let Err(r) = self.update_ui(tui).await {
                        log::error!("Unable to update UI: {}", r)
                    }
                    if let Some(progress_bar) = self
                        .components
                        .iter_mut()
                        .find_map(|component| component.as_any().downcast_mut::<ProgressBar>())
                    {
                        let progress = (step as f64 / total_steps as f64) * 100.0;
                        progress_bar.update_progress(progress, action_tx)?;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
                }

                self.update_info(format!("{:#?}", check_ver).as_str());
                self.update_info("Neovim Nightly installed successfully.");
            }
            Action::InstallNeviraide => {
                self.update_info("NEVIRAIDE");
            }
            Action::CheckDeps => {
                self.update_info("All dependencies are up to date");
            }
            Action::Test => {
                self.update_info("Test for test");
                if let Some(input) = self
                    .components
                    .iter_mut()
                    .find_map(|component| component.as_any().downcast_mut::<Input>())
                {
                    input.show();
                }
            }
            Action::Quit => self.should_quit = true,
            Action::Suspend => self.should_suspend = true,
            Action::Resume => self.should_suspend = false,
            Action::Render => self.update_ui(tui).await?,
            _ => log::debug!("Unhandled action: {:?}", action),
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.update(action.clone())? {
                action_tx.send(action)?
            };
        }
        Ok(())
    }
}
