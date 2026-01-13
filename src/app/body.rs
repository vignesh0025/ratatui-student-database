use ratatui::crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style, Styled, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph, StatefulWidget, Widget},
};

use crate::app::app_structs::{Class, StudentListItem, student_list_vec};
use crate::app::data_model::DataModel;
use crate::app::{AppComponent, data_model};

use std::cell::RefCell;
use std::rc::Rc;

use std::path::Path;
use tui_textarea::{Input, Key, TextArea};

#[derive(Default)]
pub struct AppBody<'a> {
    body_state: ListState,
    is_active: bool,
    string_logs: Rc<RefCell<Vec<String>>>,
    show_popup: bool,
    textarea: TextArea<'a>,
    data_model: DataModel,
    dialog_type: Option<DialogType>,
}

enum DialogType {
    FilterDialog,
    Dialog1,
}

impl<'a> AppComponent for AppBody<'a> {
    fn set_active(&mut self, b: bool) {
        self.is_active = b;
    }

    fn render(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        self.render_body(area, buf);
    }

    fn handle_event(&mut self, event: Event) -> Option<Event> {
        let handled_event: Option<Event> = self.handle_popup_event(event);

        if let Some(event) = handled_event {
            if let Event::Key(key) = event {
                match key.code {
                    KeyCode::Up => self.body_state.select_previous(),
                    KeyCode::Down => self.body_state.select_next(),
                    KeyCode::Enter => {
                        if let Some(e) = self.body_state.selected() {
                            if let Some(s) = self.data_model.data_items.get(e) {
                                // self.string_logs.borrow_mut().push(s.clone());
                            }
                        }
                    }
                    KeyCode::Char('H') => self.body_state.select_first(),
                    KeyCode::Char('T') => self.body_state.select_last(),
                    KeyCode::Char('c') => self.data_model.clear_filter(),
                    KeyCode::Char('s') => self.dialog_type = Some(DialogType::FilterDialog),
                    _ => return Some(event),
                }
            } else {
                return Some(event);
            }
        }

        None
    }
}

impl<'a> AppBody<'a> {
    pub fn new(is_active: bool, string_logs: Rc<RefCell<Vec<String>>>) -> Self {
        let app = AppBody::default();

        // let body_items = student_list_vec(&Path::new("./student-dataset.csv")).unwrap();
        let data_model = DataModel::new(
            &Path::new("./student-dataset.csv"),
            data_model::DataType::CsvBin,
        )
        .unwrap();
        Self {
            body_state: ListState::default().with_selected(Some(0)),
            is_active,
            string_logs,
            data_model,
            ..app
        }
    }

    fn render_body(&mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut block = Block::bordered().title("List");

        if self.is_active {
            block = block.border_style(Style::new().green());
        }

        let items = self
            .data_model
            .filter()
            .into_iter()
            .enumerate()
            .map(|(i, val)| ListItem::from(val))
            .collect::<Vec<_>>();

        let list = List::new(items).block(block).highlight_style(
            Style::default()
                .bg(ratatui::style::Color::White)
                .fg(ratatui::style::Color::Black),
        );

        StatefulWidget::render(list, area, buf, &mut self.body_state);

        match &self.dialog_type {
            Some(DialogType::FilterDialog) => {
                let popup_area = popup_area(area, 60, 20);

                let c = Clear::default();
                c.render(popup_area, buf);

                self.textarea.set_cursor_line_style(Style::default());
                self.textarea.set_placeholder_text("Enter a search here: ");
                self.textarea.set_block(
                    Block::bordered()
                        .border_type(ratatui::widgets::BorderType::Rounded)
                        .title("Enter name: "),
                );

                self.textarea.render(popup_area, buf);
            }
            _ => {}
        }
    }

    fn handle_popup_event(&mut self, event: Event) -> Option<Event> {
        match &self.dialog_type {
            Some(DialogType::FilterDialog) => {
                if let Event::Key(key) = &event {
                    match key.code {
                        KeyCode::Char(_) | KeyCode::Backspace => {
                            self.textarea.input(event);
                            return None;
                        }
                        KeyCode::Enter => {
                            let lines = self.textarea.lines();
                            if lines[0].is_empty() {
                                self.data_model.clear_filter();
                            } else {
                                self.data_model.set_filter(&lines[0]);
                            }
                            self.dialog_type = None;
                            // self.body_items.iter().for_each(|i| i.name.find(lines[0]));
                        }
                        KeyCode::Esc => self.dialog_type = None,
                        _ => {}
                    }
                }
            },
            _ => {}
        }

        Some(event)
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
