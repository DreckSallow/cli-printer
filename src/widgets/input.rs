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
type Cb = dyn FnMut(&String, &bool) -> Action;

type Callback = Box<Cb>;

pub struct Input<'a> {
    text_init: IconAndLabel<'a>,
    text_final: IconAndLabel<'a>,
    input: String,
    is_hidden: bool,
    complete_input: bool,
    callback: Callback,
}

impl<'a> Widget for Input<'a> {
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

impl<'a> WidgetChild for Input<'a> {
    fn do_any(&mut self) -> utils::Action {
        (self.callback)(&self.input, &self.complete_input)
    }
}

impl<'a> Default for Input<'a> {
    fn default() -> Self {
        Self {
            text_init: IconAndLabel(ICON_QUESTION, "Write: "),
            text_final: IconAndLabel(ICON_CHECK, "Write: "),
            input: String::new(),
            is_hidden: false,
            complete_input: false,
            callback: Box::new(|content, is_selected| {
                if *is_selected && content.len() > 0 {
                    return Action::Next;
                }
                Action::KeepSection
            }),
        }
    }
}

impl<'a> Input<'a> {
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
    pub fn add_fn(&mut self, cb: impl FnMut(&String, &bool) -> Action + 'a + 'static) {
        self.callback = Box::new(cb);
    }
}
