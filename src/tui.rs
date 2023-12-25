use std::{io, panic};

use color_eyre::eyre::Result;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{app::App, event::EventHandler, ui};

pub struct  Tui {
    // Interface of the teminal
    terminal : CrosstermTerminal,
    // Terminal event handler
    pub events: EventHandler,
}

impl Tui {
    // Contructs a new instance of [`Tui`]

    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events}
    }

    // Initialise the terminal interface
    // Enables the raw mode and sets terminal properties.

    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;

        // Define custom panic hook to reset the terminal properties
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    // Resets the terminal interface
    // This function is also used for the panic hook to revert
    // the terminal properties if unexpected errors occure.

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        Ok(())
    }

    // Exits the terminal interface
    // It disables the raw mode and reverts back the terminal properties
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    // [`Draw`] the terminal interface by [`rendering`] the widgets
    // [`Draw`] tui::Terminal::draw
    // [`rendering`] crate::ui::render

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

}
