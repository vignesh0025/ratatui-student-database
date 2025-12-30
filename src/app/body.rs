use ratatui::{style::{Style, Styled, Stylize}, widgets::{self, Block, List, ListItem, ListState, Paragraph, StatefulWidget, Widget}};

#[derive(Default)]
pub struct AppBody {
    text: String,
    items: Vec<String>,
    state: ListState,
    is_active: bool
}

impl AppBody {
    pub fn new(text: String, is_active: bool) -> Self {
        let i = ["ABCD", "EFGH"].map(|f| f.to_owned());
        Self {
            text: text,
            items: i.to_vec(),
            state: ListState::default(),
            is_active: is_active
        }
    }

    pub fn set_active(&mut self, b: bool) {
        self.is_active = b;
    }

    fn add_items(&mut self) {
        self.items.push(String::from("ABCD"));
        self.items.push(String::from("EFGH"));
        self.items.push(String::from("IJKL"));
    }
}

impl Widget for &mut AppBody {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut block = Block::bordered().title("List");

        if self.is_active {
            block = block.set_style(Style::new().green());
        }

        let list_items = self.items.iter().map(|f| ListItem::new(f.to_string()));
        let list = List::new(list_items)
            .block(block)
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
