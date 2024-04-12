use super::app::Action;
use crossterm::event::KeyCode;
use std::collections::HashMap;

pub struct KeyBindings {
    bindings: HashMap<KeyCode, Action>,
}

impl KeyBindings {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();

        bindings.insert(KeyCode::Char('j'), Action::Next);
        bindings.insert(KeyCode::Char('n'), Action::Next);
        bindings.insert(KeyCode::Down, Action::Next);
        bindings.insert(KeyCode::Char('k'), Action::Previous);
        bindings.insert(KeyCode::Char('p'), Action::Previous);
        bindings.insert(KeyCode::Up, Action::Previous);
        bindings.insert(KeyCode::Enter, Action::Select);
        bindings.insert(KeyCode::Char('q'), Action::Quit);
        bindings.insert(KeyCode::Esc, Action::Quit);

        Self { bindings }
    }

    pub fn get_action(&self, key: KeyCode) -> Option<&Action> {
        self.bindings.get(&key)
    }
}
