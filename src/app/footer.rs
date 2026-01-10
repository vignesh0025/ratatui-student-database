use ratatui::{
    style::{Style, Styled, Stylize},
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::AppComponent;

use std::cell::RefCell;
use std::rc::Rc;

use ratatui::crossterm::event::{Event, KeyCode};

pub struct AppFooter {
    is_active: bool,
    string_logs: Rc<RefCell<Vec<String>>>,
}

impl AppFooter {
    pub fn new(is_active: bool, string_logs: Rc<RefCell<Vec<String>>>) -> Self {
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

        let vec_str = &*self.string_logs.borrow();
        let lines: Vec<Line> = vec_str.iter().map(|s| Line::from(&s[..])).collect();
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
                    self.string_logs.borrow_mut().clear()
                },
                _ => { return Some(event) }
            }
        }

        None
    }
}
