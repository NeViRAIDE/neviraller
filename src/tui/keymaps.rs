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

    pub fn format_bindings(&self) -> String {
        let mut result = String::new();
        for (key, action) in &self.bindings {
            let key_str = match key {
                KeyCode::Char(c) => c.to_string(),
                KeyCode::Down => "Down".to_string(),
                KeyCode::Up => "Up".to_string(),
                KeyCode::Enter => "Enter".to_string(),
                KeyCode::Esc => "Esc".to_string(),
                _ => format!("{:?}", key), // Для других не обработанных случаев
            };
            let action_str = format!("{:?}", action);
            result.push_str(&format!("{}: {}, ", key_str, action_str));
        }
        result.trim_end_matches(", ").to_string() // Удаляем последнюю запятую
    }
}
