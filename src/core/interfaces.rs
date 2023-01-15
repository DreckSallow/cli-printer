use std::io::{self, Stdout};

use super::utils::{Action, RenderWidget};

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

pub trait WidgetChild<T: Clone>: Widget {
    /// Method called by Widget parent, before rendering.
    ///
    /// Returns a `RenderWidget`, that indicate if it will render
    /// ## Arguments:
    ///
    /// * `global_state` - Some struct or any type that hold a global state
    ///
    /// If the returned value is `RenderWidget::Yes`, then the widget is rendered
    ///
    /// Otherwise,not is rendered, and exit the render of the all widgets
    fn before_render(&mut self, global_state: T) -> RenderWidget;

    /// Method called by Widget parent, after rendering.
    ///
    /// Returns a `Action`, that indicate if `continue` with the next widget,
    ///
    /// `keep` render with the current widget, and `exit` the render by widget parent.
    ///
    /// ## Arguments:
    ///
    /// * `global_state` - Some struct or any type that hold a global state
    ///
    ///
    fn after_render(&mut self, global_state: T) -> Action;
}
