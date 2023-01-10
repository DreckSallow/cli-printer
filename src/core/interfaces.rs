use std::io::{self, Stdout};

use super::utils::{Action, WidgetState};

///Define a Widget to render in each turn of the loop
pub trait Widget {
    fn render(&mut self, stdout: &mut Stdout) -> io::Result<()>;
}

pub trait WidgetRoot {
    fn render(&mut self, _stdout: &mut Stdout) -> io::Result<()> {
        loop {
            todo!()
        }
    }
}

pub trait WidgetChild: Widget {
    fn action(&self) -> Action;
    fn get_state(&self) -> &WidgetState<String>;
}
