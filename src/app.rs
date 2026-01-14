use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Widget;

mod app_traits;
mod app_structs;
mod body;
mod footer;
mod menu;
mod data_model;

use body::AppBody;
use footer::AppFooter;
use menu::AppMenu;
use app_traits::*;

pub struct App {
    pub quit: bool,
    dir: Direction,
    app_components: HashMap<ActiveAppWindow, Box<dyn AppComponent>>,
    active_window: ActiveAppWindow,
    // #[allow(dead_code)] // Kept alive to maintain Rc reference count for shared state
    // string_logs: Rc<Mutex<Vec<String>>>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        let ref_string_logs = Arc::new(Mutex::new(Vec::new()));
        Self {
            quit: false,
            dir: Direction::Vertical,
            app_components: HashMap::from([
                (
                    ActiveAppWindow::Header,
                    Box::new(AppBody::new(false, Arc::clone(&ref_string_logs)))
                        as Box<dyn AppComponent>,
                ),
                (
                    ActiveAppWindow::Menu,
                    Box::new(AppMenu::new()) as Box<dyn AppComponent>
                ),
                (
                    ActiveAppWindow::Body,
                    Box::new(AppBody::new(true, Arc::clone(&ref_string_logs)))
                        as Box<dyn AppComponent>,
                ),
                (
                    ActiveAppWindow::Footer,
                    Box::new(AppFooter::new(false, Arc::clone(&ref_string_logs)))
                        as Box<dyn AppComponent>,
                ),
            ]),
            active_window: ActiveAppWindow::Body,
            // string_logs: ref_string_logs,
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

        let mut event_handled: Option<Event> = None;

        /* Forward events to Active Window and process them only if they are not handled inside */
        if let Some(component) = self.app_components.get_mut(&self.active_window) {
            event_handled = component.handle_event(event);
        }

        if let Some(event) = event_handled {
            if let Event::Key(k) = event {
                match k.code {
                    KeyCode::Char('q') => self.quit = true,
                    KeyCode::Char('v') => self.dir = Direction::Vertical,
                    KeyCode::Char('h') => self.dir = Direction::Horizontal,
                    KeyCode::Char('1') => self.set_window_active(ActiveAppWindow::Header),
                    KeyCode::Char('2') => self.set_window_active(ActiveAppWindow::Body),
                    KeyCode::Char('3') => self.set_window_active(ActiveAppWindow::Footer),
                    KeyCode::Char('4') => self.set_window_active(ActiveAppWindow::Menu),
                    _ => {

                    }
                }
            }
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

        let layout_body_menu =  [
            Constraint::Fill(1),
            Constraint::Fill(9)
        ];

        let [menu, body] = Layout::horizontal(layout_body_menu).areas(middle);

        for (k, v) in self.app_components.iter_mut() {
            match k {
                ActiveAppWindow::Header => v.render(top, buf),
                ActiveAppWindow::Body => v.render(body, buf),
                ActiveAppWindow::Menu => v.render(menu, buf),
                ActiveAppWindow::Footer => v.render(bottom, buf),
            }
        }
    }
}
