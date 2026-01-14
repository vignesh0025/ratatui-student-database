use ratatui::{
    style::{Style, Styled, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::AppComponent;

use std::sync::{Arc, Mutex};
use ratatui::crossterm::event::{Event, KeyCode};

pub struct AppFooter {
    is_active: bool,
    string_logs: Arc<Mutex<Vec<String>>>,
}

impl AppFooter {
    pub fn new(is_active: bool, string_logs: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            is_active,
            string_logs,
        }
    }
}

impl AppComponent for AppFooter {
    fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut block = Block::bordered().title("List");

        if self.is_active {
            block = block.set_style(Style::new().green());
        }

        let vec_str = self.string_logs.lock().unwrap();
        // let vec_str = &*self.string_logs.borrow();
        let lines: Vec<Line> = vec_str.iter().map(|s| Line::from(&s[..])).rev().collect();
        let p = Paragraph::new(Text::from(lines)).block(block);

        p.render(area, buf);
    }

    fn set_active(&mut self, b: bool) {
        self.is_active = b;
    }

    fn handle_event(&mut self, event: Event) -> Option<Event> {
        // Footer doesn't handle any events currently
        if let Event::Key(k) = event {
            match k.code {
                KeyCode::Char('c') => {
                    let mut string_logs = self.string_logs.lock().unwrap();
                    string_logs.clear()
                },
                _ => { return Some(event) }
            }
        }

        None
    }
}
