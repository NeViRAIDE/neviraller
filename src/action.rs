use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Display, Deserialize)]
pub enum Action {
    Select,
    Next,
    Prev,
    InstallNeovimNightly,
    InstallNeviraide,
    CheckDeps,
    Test,
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    LogMessage(String),
    Help,
}
