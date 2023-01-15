use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Print, Stylize},
};

use crate::{
    core::{
        interfaces::{Widget, WidgetChild},
        utils::{Action, IconAndLabel, RenderWidget},
    },
    styles::{ICON_CHECK, ICON_QUESTION},
};

use super::list;

type AfterCb<T> = dyn FnMut(&mut ListSelectedData, T) -> Action;

type BeforeCb<T> = dyn FnMut(&mut ListSelectedData, T) -> RenderWidget;

pub struct ListSelected<'a, T> {
    pub list: list::List<'a>,
    cb_before: Box<BeforeCb<T>>,
    cb_after: Box<AfterCb<T>>,
    text_init: IconAndLabel<'a>,
    text_final: IconAndLabel<'a>,
    local_state: ListSelectedData,
}

pub struct ListSelectedData {
    pub is_selected: bool,
    pub offset: usize,
    pub current_option: Option<String>,
    pub length: usize,
}

impl<'a, T: Clone> Widget for ListSelected<'a, T> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        if !self.local_state.is_selected {
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
                        self.local_state.is_selected = true
                    }
                    self.local_state.offset = self.list.state.offset.clone();
                    self.local_state.current_option = match self.list.get_current_index().1 {
                        Some(l) => Some(l.0.to_string()),
                        None => None,
                    };
                }
                _ => {}
            }
            return Ok(());
        }

        let option_selected = self.list.get_current_index();
        let text_selected = match option_selected.1 {
            Some(t) => t.0,
            None => return Ok(()),
        };

        execute!(
            stdout,
            Print(self.text_final.0.green()),
            Print(self.text_final.1),
            Print(text_selected.dark_grey()),
        )?;

        Ok(())
    }
}

impl<'a, T: Clone> WidgetChild<T> for ListSelected<'a, T> {
    fn after_render(&mut self, global_state: T) -> Action {
        (self.cb_after)(&mut self.local_state, global_state)
    }

    fn before_render(&mut self, global_state: T) -> RenderWidget {
        (self.cb_before)(&mut self.local_state, global_state)
    }
}

impl<'a, T: Clone> ListSelected<'a, T> {
    pub fn new(options: Vec<&'a str>) -> Self {
        let length = options.len();
        Self {
            list: list::List::new(options),
            cb_after: Box::new(|_, _| Action::Next),
            cb_before: Box::new(|_, _| RenderWidget::Yes),
            text_init: IconAndLabel(ICON_QUESTION, "Choose an option: "),
            text_final: IconAndLabel(ICON_CHECK, "Option selected: "),
            local_state: ListSelectedData {
                is_selected: false,
                offset: 0,
                current_option: None,
                length,
            },
        }
    }
    pub fn after(&mut self, cb: impl FnMut(&mut ListSelectedData, T) -> Action + 'static) {
        self.cb_after = Box::new(cb);
    }
    pub fn before(&mut self, cb: impl FnMut(&mut ListSelectedData, T) -> RenderWidget + 'static) {
        self.cb_before = Box::new(cb);
    }
    pub fn add_text_init(&mut self, icon: &'a str, label: &'a str) {
        self.text_init = IconAndLabel(icon, label);
    }
    pub fn add_text_final(&mut self, icon: &'a str, label: &'a str) {
        self.text_final = IconAndLabel(icon, label);
    }
}
