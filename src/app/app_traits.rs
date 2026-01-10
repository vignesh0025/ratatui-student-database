use ratatui::crossterm::event::Event;

pub trait AppComponent {
    fn set_active(&mut self, b: bool);
    fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer);
    fn handle_event(&mut self, event: Event) -> Option<Event>;
}

#[derive(Default, PartialEq, Eq, Hash)]
pub enum ActiveAppWindow {
    Header,
    Footer,
    #[default]
    Body,
    Menu
}
