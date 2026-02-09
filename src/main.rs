mod key_derivation;
mod vault;
mod vault_entry;
mod vault_header;
mod app;
mod ui;

use crate::app::App;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(|terminal| App::new().expect("Error when launch App").run(terminal))?;
    Ok(())
}
