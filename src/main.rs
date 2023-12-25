
pub mod app;

pub mod event;

pub mod ui;

pub mod tui;

pub mod update;

use color_eyre::eyre::Result;
use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal,events);
    tui.enter()?;

    while !app.should_quit {
        // render app
        tui.draw(&mut app)?;

        // handel events
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_,_) => {}
        };

    }
    //Exit
    tui.exit()?;
    Ok(())
}
