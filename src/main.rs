use color_eyre::Result;

use self::tui::{
    errors::install_hooks,
    neviraller::Neviraller,
    tui::{init_term, restore_term},
};

mod tui;

fn main() -> Result<()> {
    install_hooks()?;
    let mut terminal = init_term()?;
    let app_result = Neviraller::default().run(&mut terminal);
    restore_term()?;
    app_result
}
