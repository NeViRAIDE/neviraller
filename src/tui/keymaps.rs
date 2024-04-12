use super::app::Action;
use crossterm::event::KeyCode;
use std::collections::BTreeMap;

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
struct OrdKeyCode(KeyCode);

impl Ord for OrdKeyCode {
    fn cmp(&self, other: &Self) -> Ordering {
        format!("{:?}", self.0).cmp(&format!("{:?}", other.0))
    }
}

impl PartialOrd for OrdKeyCode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for OrdKeyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Clone)]
pub struct KeyBindings {
    bindings: BTreeMap<OrdKeyCode, Action>,
}

impl KeyBindings {
    pub fn new() -> Self {
        let mut bindings = BTreeMap::new();

        bindings.insert(OrdKeyCode(KeyCode::Char('j')), Action::Next);
        bindings.insert(OrdKeyCode(KeyCode::Char('n')), Action::Next);
        bindings.insert(OrdKeyCode(KeyCode::Down), Action::Next);
        bindings.insert(OrdKeyCode(KeyCode::Char('k')), Action::Previous);
        bindings.insert(OrdKeyCode(KeyCode::Char('p')), Action::Previous);
        bindings.insert(OrdKeyCode(KeyCode::Up), Action::Previous);
        bindings.insert(OrdKeyCode(KeyCode::Enter), Action::Select);
        bindings.insert(OrdKeyCode(KeyCode::Char('q')), Action::Quit);
        bindings.insert(OrdKeyCode(KeyCode::Esc), Action::Quit);

        Self { bindings }
    }

    pub fn get_action(&self, key: KeyCode) -> Option<&Action> {
        self.bindings.get(&OrdKeyCode(key))
    }

    pub fn format_bindings(&self) -> String {
        let mut action_to_keys: BTreeMap<&Action, Vec<String>> = BTreeMap::new();
        for (key, action) in &self.bindings {
            let key_str = match key.0 {
                KeyCode::Char(c) => c.to_string(),
                KeyCode::Down => "".to_string(),
                KeyCode::Up => "".to_string(),
                KeyCode::Enter => "Enter".to_string(),
                KeyCode::Esc => "Esc".to_string(),
                _ => format!("{:?}", key.0),
            };
            action_to_keys.entry(action).or_default().push(key_str);
        }

        let mut result = Vec::new();
        for (action, keys) in action_to_keys {
            let action_str = format!("{:?}", action);
            let keys_str = keys.join(" | ");
            result.push(format!("{}: {}", action_str, keys_str));
        }

        result.join("     ")
    }
}
