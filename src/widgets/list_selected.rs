use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Print, Stylize},
};

use crate::core::{
    interfaces::{Widget, WidgetChild},
    utils::Action,
};

use super::list::{self, List, ListState};

type Cb = dyn FnMut(&ListState, &bool) -> Action;

type Callback = Box<Cb>;
pub struct TextInit<'a>(pub &'a str, pub &'a str);

pub struct ListSelected<'a> {
    pub list: list::List<'a>,
    callback: Callback,
    is_selected: bool,
    text_init: TextInit<'a>,
    text_final: TextInit<'a>,
}

impl<'a> Widget for ListSelected<'a> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        if !self.is_selected {
            execute!(
                stdout,
                Print(self.text_init.0.cyan()),
                Print(self.text_init.1),
                Print("\n")
            )?;
            self.list.render(stdout)?;
            match event::read()? {
                Event::Key(k) => {
                    let code = k.code;
                    if code == KeyCode::Down {
                        self.list.next()
                    } else if code == KeyCode::Up {
                        self.list.prev()
                    } else if code == KeyCode::Enter {
                        self.is_selected = true
                    }
                }
                _ => {}
            }
            return Ok(());
        }

        let option_selected = self.list.get_current_index();
        if option_selected.0 == self.list.length - 1 {
            return Ok(()); // Selected las option (Usually is exit options)
        }
        let text_selected = match option_selected.1 {
            Some(t) => t.0,
            None => return Ok(()),
        };

        execute!(
            stdout,
            Print(self.text_final.0.cyan()),
            Print(self.text_final.1),
            Print(text_selected.dark_grey()),
            Print("\n")
        )?;

        Ok(())
    }
}

impl<'a> WidgetChild for ListSelected<'a> {
    fn do_any(&mut self) -> Action {
        (self.callback)(&self.list.state, &self.is_selected)
    }
}

impl<'a> ListSelected<'a> {
    pub fn new(options: Vec<&'a str>) -> Self {
        Self {
            list: List::new(options),
            callback: Box::new(|_, _| Action::KeepSection),
            is_selected: false,
            text_init: TextInit("? ", "Choose an option: "),
            text_final: TextInit("√ ", "Option selected: "),
        }
    }
    pub fn add_fn(&mut self, cb: impl FnMut(&ListState, &bool) -> Action + 'a + 'static) {
        self.callback = Box::new(cb);
    }
    pub fn add_text_init(&mut self, icon: &'a str, label: &'a str) {
        self.text_init = TextInit(icon, label);
    }
    pub fn add_text_final(&mut self, icon: &'a str, label: &'a str) {
        self.text_final = TextInit(icon, label);
    }
}
