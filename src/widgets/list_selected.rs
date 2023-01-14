use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Print, Stylize},
};

use crate::{
    core::{
        interfaces::{Widget, WidgetChild},
        utils::{Action, IconAndLabel},
    },
    styles::{ICON_CHECK, ICON_QUESTION},
};

use super::list;

type Cb<T> = dyn FnMut(ListSelectedData, T) -> Action;

type Callback<T> = Box<Cb<T>>;

pub struct ListSelected<'a, T> {
    pub list: list::List<'a>,
    callback: Callback<T>,
    pub is_selected: bool,
    text_init: IconAndLabel<'a>,
    text_final: IconAndLabel<'a>,
}

pub struct ListSelectedData {
    pub is_selected: bool,
    pub offset: usize,
    pub current_option: Option<String>,
    pub length: usize,
}

impl<'a, T: Clone> Widget for ListSelected<'a, T> {
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

impl<'a, T: Clone> WidgetChild<T> for ListSelected<'a, T> {
    fn do_any(&mut self, global_state: T) -> Action {
        let data = self.get_internal_data();
        (self.callback)(data, global_state)
    }
}

impl<'a, T: Clone> ListSelected<'a, T> {
    pub fn new(options: Vec<&'a str>) -> Self {
        Self {
            list: list::List::new(options),
            callback: Box::new(|_, _| Action::Next),
            is_selected: false,
            text_init: IconAndLabel(ICON_QUESTION, "Choose an option: "),
            text_final: IconAndLabel(ICON_CHECK, "Option selected: "),
        }
    }
    pub fn add_fn(&mut self, cb: impl FnMut(ListSelectedData, T) -> Action + 'a + 'static) {
        self.callback = Box::new(cb);
    }
    pub fn add_text_init(&mut self, icon: &'a str, label: &'a str) {
        self.text_init = IconAndLabel(icon, label);
    }
    pub fn add_text_final(&mut self, icon: &'a str, label: &'a str) {
        self.text_final = IconAndLabel(icon, label);
    }
    fn get_internal_data(&self) -> ListSelectedData {
        let current_option = match self.list.get_current_index().1 {
            Some(opt) => Some(opt.0.to_string()),
            None => None,
        };
        ListSelectedData {
            is_selected: self.is_selected,
            offset: self.list.state.offset,
            current_option,
            length: self.list.length,
        }
    }
}
