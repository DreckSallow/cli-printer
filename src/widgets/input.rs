use core::time;

use crossterm::{
    event::{self, poll, KeyCode},
    execute,
    style::{Print, Stylize},
};

use crate::{
    core::{
        interfaces::{Widget, WidgetChild},
        utils::{self, Action, IconAndLabel, RenderWidget},
    },
    styles::{ICON_CHECK, ICON_QUESTION},
};

type AfterCb<T> = dyn FnMut(&mut InputData, T) -> Action;
type BeforeCb<T> = dyn FnMut(&mut InputData, T) -> RenderWidget;

pub struct Input<'a, T> {
    text_init: IconAndLabel<'a>,
    text_final: IconAndLabel<'a>,
    cb_after: Box<AfterCb<T>>,
    cb_before: Box<BeforeCb<T>>,
    local_state: InputData,
}

#[derive(Clone)]
pub struct InputData {
    pub input: String,
    pub is_hidden: bool,
    pub complete_input: bool,
}

impl<'a, T: Clone> Widget for Input<'a, T> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        if !self.local_state.complete_input {
            execute!(
                stdout,
                Print(self.text_init.0.cyan()),
                Print(self.text_init.1),
            )?;
            if !self.local_state.is_hidden {
                execute!(stdout, Print(self.local_state.input.as_str().dark_grey()))?;
            }
            if poll(time::Duration::from_millis(80))? {
                match event::read()? {
                    event::Event::Key(k) => {
                        let code = k.code;
                        if code == KeyCode::Enter {
                            if self.local_state.input.len() != 0 {
                                self.local_state.complete_input = true;
                            }
                        } else if code == KeyCode::Backspace {
                            self.local_state.input.pop();
                        }
                        if let KeyCode::Char(c) = code {
                            self.local_state.input.push(c);
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
            Print(self.local_state.input.as_str().dark_grey())
        )?;
        Ok(())
    }
}

impl<'a, T: Clone> WidgetChild<T> for Input<'a, T> {
    fn before_render(&mut self, global_state: T) -> RenderWidget {
        (self.cb_before)(&mut self.local_state, global_state)
    }
    fn after_render(&mut self, global_state: T) -> utils::Action {
        (self.cb_after)(&mut self.local_state, global_state)
    }
}

impl<'a, T: Clone> Default for Input<'a, T> {
    fn default() -> Self {
        Self {
            text_init: IconAndLabel(ICON_QUESTION, "Write: "),
            text_final: IconAndLabel(ICON_CHECK, "Write: "),
            cb_after: Box::new(|input, _gloabl_state| {
                if input.complete_input && input.input.len() > 0 {
                    return Action::Next;
                }
                Action::KeepSection
            }),

            local_state: InputData {
                input: String::new(),
                is_hidden: false,
                complete_input: false,
            },
            cb_before: Box::new(|_, _| RenderWidget::Yes),
        }
    }
}

impl<'a, T: Clone> Input<'a, T> {
    pub fn new(text_init: IconAndLabel<'a>, text_final: IconAndLabel<'a>) -> Self {
        Self {
            text_init,
            text_final,
            cb_after: Box::new(|input, _gloabl_state| {
                if input.complete_input && input.input.len() > 0 {
                    return Action::Next;
                }
                Action::KeepSection
            }),
            local_state: InputData {
                input: String::new(),
                is_hidden: false,
                complete_input: false,
            },
            cb_before: Box::new(|_, _| RenderWidget::Yes),
        }
    }
    pub fn hidden(&mut self, hidden: bool) {
        self.local_state.is_hidden = hidden
    }
    pub fn after(&mut self, cb: impl FnMut(&mut InputData, T) -> Action + 'static) {
        self.cb_after = Box::new(cb);
    }
    pub fn before(&mut self, cb: impl FnMut(&mut InputData, T) -> RenderWidget + 'static) {
        self.cb_before = Box::new(cb);
    }
}
