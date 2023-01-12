use crossterm::execute;
use crossterm::style::{Print, Stylize};

use crate::core::interfaces::Widget;

pub struct ListItem<'b>(pub &'b str);

#[derive(Default)]
pub struct ListState {
    pub offset: usize,
}

pub struct List<'b> {
    items: Vec<ListItem<'b>>,
    pub length: usize,
    pub state: ListState,
    icon_selected: &'b str,
    icon_space: String,
}

impl<'b> Widget for List<'b> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        let mut count = 0;
        for option in &self.items {
            if count == self.state.offset {
                execute!(
                    stdout,
                    Print(self.icon_selected.cyan()),
                    Print(option.0.cyan()),
                    Print("\n")
                )?;
            } else {
                execute!(
                    stdout,
                    Print(&self.icon_space),
                    Print(option.0),
                    Print("\n")
                )?;
            }
            count += 1;
        }
        Ok(())
    }
}

impl<'b> List<'b> {
    pub fn new(options: Vec<&'b str>) -> Self {
        let options_list: Vec<ListItem<'b>> = options.iter().map(|s| ListItem(s)).collect();
        let length = options_list.len();
        Self {
            items: options_list,
            state: ListState::default(),
            icon_selected: "> ",
            icon_space: "  ".to_string(),
            length,
        }
    }
    pub fn add_icon_selected(&mut self, icon: &'b str) {
        let chars: Vec<&str> = icon.chars().map(|_| " ").collect();
        self.icon_selected = icon;
        self.icon_space = chars.join("");
    }
    pub fn next(&mut self) {
        let offset = &self.state.offset;
        if offset + 1 >= self.items.len() {
            self.state.offset = 0;
        } else {
            self.state.offset += 1;
        }
    }
    pub fn prev(&mut self) {
        let offset = &self.state.offset;
        if *offset == 0 {
            self.state.offset = self.items.len() - 1;
        } else {
            self.state.offset -= 1;
        }
    }
    pub fn get_current_index(&self) -> (usize, Option<&ListItem<'b>>) {
        (self.state.offset.clone(), self.items.get(self.state.offset))
    }
}
