use std::error::Error;
use crossterm::event;

pub mod app;

use app::App;
use ratatui::widgets::ListState;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    let mut terminal = ratatui::init();
    while !app.quit {
        terminal.draw(|f| f.render_widget(&mut app, f.area()))?;
        app.handle_events(event::read()?);
    }
    ratatui::restore();

    Ok(())
}
