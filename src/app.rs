use std::collections::HashMap;

use crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, ListState, Paragraph, StatefulWidget, Widget};

mod body;

use body::AppBody;

#[derive(Default, PartialEq, Eq, Hash)]
enum ActiveAppWindow {
    Header,
    Footer,
    #[default]
    Body,
}

pub struct App {
    pub quit: bool,
    dir: Direction,
    app_components: HashMap<ActiveAppWindow, AppBody>,
    active_window: ActiveAppWindow,
}

impl App {
    pub fn new() -> Self {
        let app_components = HashMap::from([
            (
                ActiveAppWindow::Header,
                AppBody::new("HeaderText".to_owned(), false),
            ),
            (
                ActiveAppWindow::Body,
                AppBody::new("BodyText".to_owned(), true),
            ),
            (
                ActiveAppWindow::Footer,
                AppBody::new("FooterText".to_owned(), false),
            ),
        ]);
        Self {
            quit: false,
            dir: Direction::Vertical,
            app_components,
            active_window: ActiveAppWindow::Body,
        }
    }

    fn set_window_active(&mut self, window: ActiveAppWindow) {
        if self.active_window == window {
            return;
        }

        for (k, v) in &mut self.app_components {
            v.set_active(*k == window);
        }

        self.active_window = window;
    }

    pub fn handle_events(&mut self, event: Event) {
        match event {
            Event::Key(k) => match k.code {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char('v') => self.dir = Direction::Vertical,
                KeyCode::Char('h') => self.dir = Direction::Horizontal,
                KeyCode::Char('1') => self.set_window_active(ActiveAppWindow::Header),
                KeyCode::Char('2') => self.set_window_active(ActiveAppWindow::Body),
                KeyCode::Char('3') => self.set_window_active(ActiveAppWindow::Footer),
                _ => {}
            },
            _ => {}
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let layout = [
            Constraint::Fill(1),
            Constraint::Fill(9),
            Constraint::Fill(1),
        ];
        let l = Layout::vertical(layout).direction(self.dir);
        let [top, middle, bottom] = l.areas(area);
        
        for (k,v) in self.app_components.iter_mut() {
            match k {
                ActiveAppWindow::Header => v.render(top, buf),   
                ActiveAppWindow::Body => v.render(middle, buf),   
                ActiveAppWindow::Footer => v.render(bottom, buf),   
            }
        }
    }
}
