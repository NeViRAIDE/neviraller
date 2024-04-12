use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

#[derive(Clone, Copy)]
pub enum MenuAction {
    InstallNeovimNightly,
    CheckForUpdates,
    CheckDependencies,
    Quit,
}

pub struct MenuItem {
    pub name: String,
    pub action: MenuAction,
}

pub fn get_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem {
            name: "Install Neovim Nightly".to_string(),
            action: MenuAction::InstallNeovimNightly,
        },
        MenuItem {
            name: "Check for updates".to_string(),
            action: MenuAction::CheckForUpdates,
        },
        MenuItem {
            name: "Dependencies".to_string(),
            action: MenuAction::CheckDependencies,
        },
        MenuItem {
            name: "Quit".to_string(),
            action: MenuAction::Quit,
        },
    ]
}

pub struct Menu {
    items: Vec<MenuItem>,
    state: ListState,
}

impl Menu {
    pub fn new(items: Vec<MenuItem>) -> Menu {
        let mut state = ListState::default();
        state.select(Some(0));
        Menu { items, state }
    }

    pub fn next(&mut self) {
        let next_index = match self.state.selected() {
            Some(selected) => {
                if selected >= self.items.len() - 1 {
                    0
                } else {
                    selected + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(next_index));
    }

    pub fn previous(&mut self) {
        let prev_index = match self.state.selected() {
            Some(selected) => {
                if selected == 0 {
                    self.items.len() - 1
                } else {
                    selected - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(prev_index));
    }

    pub fn select(&self) -> MenuAction {
        self.items[self.state.selected().unwrap_or(0)].action
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| ListItem::new(item.name.clone()))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Menu"))
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.state.clone());
    }
}
