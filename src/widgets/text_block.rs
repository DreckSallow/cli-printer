use crossterm::{execute, style::Print};

use crate::core::{
    interfaces::{Widget, WidgetChild},
    utils::{self, Action, RenderWidget},
};

type AfterCb<T> = Box<dyn FnMut(&mut TextBlockData, T) -> Action>;
type BeforeCb<T> = Box<dyn FnMut(&mut TextBlockData, T) -> RenderWidget>;
pub struct TextBlock<T> {
    cb_after: AfterCb<T>,
    cb_before: BeforeCb<T>,
    local_state: TextBlockData,
}

pub struct TextBlockData {
    pub text: String,
}

impl<T: Clone> Widget for TextBlock<T> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(stdout, Print(&self.local_state.text))?;
        Ok(())
    }
}

impl<T: Clone> WidgetChild<T> for TextBlock<T> {
    fn after_render(&mut self, global_state: T) -> utils::Action {
        (self.cb_after)(&mut self.local_state, global_state)
    }

    fn before_render(&mut self, global_state: T) -> RenderWidget {
        (self.cb_before)(&mut self.local_state, global_state)
    }
}

impl<T: Clone> TextBlock<T> {
    pub fn new(text: &str) -> Self {
        Self {
            cb_after: Box::new(|_, _| Action::Next),
            cb_before: Box::new(|_, _| RenderWidget::Yes),
            local_state: TextBlockData {
                text: text.to_owned(),
            },
        }
    }
    pub fn after(&mut self, cb: impl FnMut(&mut TextBlockData, T) -> Action + 'static) {
        self.cb_after = Box::new(cb)
    }
    pub fn before(&mut self, cb: impl FnMut(&mut TextBlockData, T) -> RenderWidget + 'static) {
        self.cb_before = Box::new(cb)
    }
}
