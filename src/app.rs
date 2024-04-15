use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::layout::{Constraint, Direction, Layout};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::components::{
    footer::Footer, header::Header, info::Info, menu::Menu, progress::ProgressBar, Component,
};
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
            "Test".to_string(),
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
                Box::new(progress_bar),
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
    ) -> Result<()> {
        match event {
            tui::Event::Quit => action_tx.send(Action::Quit)?,
            tui::Event::Render | tui::Event::Resize(_, _) => action_tx.send(Action::Render)?,
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
            Action::Render | Action::Resize(_, _) => self.update_ui(tui).await?,
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
                self.process_event(e, &action_tx).await?;
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
        self.last_tick_key_events.push(key);

        if let Some(keymap) = self.config.keybindings.get(&self.mode) {
            if let Some(action) = keymap.get(&self.last_tick_key_events) {
                log::info!("Action found for sequence: {action:?}");
                action_tx
                    .send(action.clone())
                    .expect("Failed to send action");
                self.last_tick_key_events.clear();
            } else {
                log::info!("No action found for sequence, clearing events.");
                self.last_tick_key_events.clear();
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
                self.update_info("Neovim Nightly ");
            }
            Action::InstallNeviraide => {
                self.update_info("NEVIRAIDE");
            }
            Action::CheckDeps => {
                self.update_info("All dependencies are up to date");
            }
            Action::Test => {
                self.update_info("Test for test");
            }
            Action::Quit => {
                self.should_quit = true;
            }
            Action::Suspend => {
                self.should_suspend = true;
            }
            Action::Resume => {
                self.should_suspend = false;
                let tui = &mut tui::Tui::new()?;
                tui.enter()?;
            }
            Action::Tick => {
                self.last_tick_key_events.drain(..);
            }
            Action::Render => {
                self.update_ui(tui).await?;
            }
            _ => {
                log::debug!("Unhandled action: {:?}", action);
            }
        }
        for component in self.components.iter_mut() {
            if let Some(action) = component.update(action.clone())? {
                action_tx.send(action)?
            };
        }
        Ok(())
    }
}
