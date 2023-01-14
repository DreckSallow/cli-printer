use core::time;

use crossterm::{
    cursor::MoveTo,
    event::{self, poll, KeyCode},
    execute,
    style::{Print, Stylize},
    terminal::Clear,
};

use crate::{
    core::{
        interfaces::{Widget, WidgetChild},
        utils::{self, Action, IconAndLabel},
    },
    styles::{ICON_CHECK, ICON_QUESTION},
};
type Cb<T> = dyn FnMut(InputData, T) -> Action;

type Callback<T> = Box<Cb<T>>;

pub struct Input<'a, T> {
    text_init: IconAndLabel<'a>,
    text_final: IconAndLabel<'a>,
    pub input: String,
    pub is_hidden: bool,
    pub complete_input: bool,
    callback: Callback<T>,
}

pub struct InputData {
    pub input: String,
    pub is_hidden: bool,
    pub complete_input: bool,
}

impl<'a, T: Clone> Widget for Input<'a, T> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(
            stdout,
            Clear(crossterm::terminal::ClearType::CurrentLine),
            MoveTo(0, 0),
        )?;
        if !self.complete_input {
            execute!(
                stdout,
                Print(self.text_init.0.cyan()),
                Print(self.text_init.1),
            )?;
            if !self.is_hidden {
                execute!(stdout, Print(self.input.as_str().dark_grey()))?;
            }
            if poll(time::Duration::from_millis(80))? {
                match event::read()? {
                    event::Event::Key(k) => {
                        let code = k.code;
                        if code == KeyCode::Enter {
                            if self.input.len() != 0 {
                                self.complete_input = true;
                            }
                        } else if code == KeyCode::Backspace {
                            self.input.pop();
                        }
                        if let KeyCode::Char(c) = code {
                            self.input.push(c);
                        }
                    }
                    _ => {}
                }
            }
            return Ok(());
        }
        execute!(
            stdout,
            Print(self.text_final.0.green()),
            Print(self.text_final.1),
            Print(self.input.as_str().dark_grey())
        )?;
        Ok(())
    }
}

impl<'a, T: Clone> WidgetChild<T> for Input<'a, T> {
    fn do_any(&mut self, global_state: T) -> utils::Action {
        let data = self.get_internal_data();
        (self.callback)(data, global_state)
    }
}

impl<'a, T: Clone> Default for Input<'a, T> {
    fn default() -> Self {
        Self {
            text_init: IconAndLabel(ICON_QUESTION, "Write: "),
            text_final: IconAndLabel(ICON_CHECK, "Write: "),
            input: String::new(),
            is_hidden: false,
            complete_input: false,
            callback: Box::new(|input, _gloabl_state| {
                if input.complete_input && input.input.len() > 0 {
                    return Action::Next;
                }
                Action::KeepSection
            }),
        }
    }
}

impl<'a, T: Clone> Input<'a, T> {
    pub fn new(text_init: IconAndLabel<'a>, text_final: IconAndLabel<'a>) -> Self {
        Self {
            text_init,
            text_final,
            input: String::new(),
            is_hidden: false,
            complete_input: false,
            callback: Box::new(|_, _| Action::KeepSection),
        }
    }
    pub fn hidden(&mut self, hidden: bool) {
        self.is_hidden = hidden
    }
    pub fn add_fn(&mut self, cb: impl FnMut(InputData, T) -> Action + 'a + 'static) {
        self.callback = Box::new(cb);
    }

    fn get_internal_data(&self) -> InputData {
        InputData {
            input: self.input.to_owned(),
            is_hidden: self.is_hidden,
            complete_input: self.complete_input,
        }
    }
}
