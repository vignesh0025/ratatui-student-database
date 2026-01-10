use std::error::Error;
use ratatui::crossterm::event;

pub mod app;

use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::default();
    let mut terminal = ratatui::init();
    while !app.quit {
        terminal.draw(|f| f.render_widget(&mut app, f.area()))?;
        app.handle_events(event::read()?.into());
    }
    ratatui::restore();

    Ok(())
}
