use crate::app::AppComponent;
use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::style::{Style, Styled, Stylize};
use ratatui::widgets::{Block, BorderType, List, ListItem, ListState, Paragraph, StatefulWidget};

pub struct AppMenu {
    is_active: bool,
    menu_state: ListState,
    menu_items: Vec<String>,
}

impl AppComponent for AppMenu {
    fn handle_event(&mut self, event: ratatui::crossterm::event::Event) -> Option<Event> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Up => self.menu_state.select_previous(),
                KeyCode::Down => self.menu_state.select_next(),
                KeyCode::Enter => {
                    if let Some(e) = self.menu_state.selected() {
                        // call some function in AppBody
                        // if let Some(s) = self.body_items.get(e) {
                        //     self.string_logs.borrow_mut().push(s.clone());
                        // }
                    }
                }
                _ => { return Some(event) }
            }
        }

        None
    }

    fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut block = Block::bordered().title("List");
        let mut highlight_style = Style::default();
        if self.is_active {
            block = block.border_style(Style::new().green());
            highlight_style = highlight_style
                .bg(ratatui::style::Color::White)
                .fg(ratatui::style::Color::Black);
        }

        let list_items = self
            .menu_items
            .iter()
            .map(|f| ListItem::new(&f[..]))
            .collect::<Vec<_>>();

        let list = List::new(list_items)
            .block(block)
            .highlight_style(highlight_style);

        StatefulWidget::render(list, area, buf, &mut self.menu_state);
    }

    fn set_active(&mut self, b: bool) {
        self.is_active = b;
    }
}

impl AppMenu {
    pub fn new() -> Self {
        let menu = ["ADD", "DELETE", "MODIFY"].map(|f| f.to_owned());
        Self {
            is_active: false,
            menu_state: ListState::default(),
            menu_items: menu.to_vec(),
        }
    }
}
