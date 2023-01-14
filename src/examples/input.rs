use std::io;

use crate::{
    core::{
        interfaces::WidgetRoot,
        utils::{Action, IconAndLabel},
        view::SectionsView,
    },
    styles::{ICON_CHECK, ICON_QUESTION},
    widgets::input::Input,
};

pub fn render_input() -> io::Result<()> {
    let mut stdout = io::stdout();

    let mut input = Input::new(
        IconAndLabel(ICON_QUESTION, "Type your name: "),
        IconAndLabel(ICON_CHECK, "Your name is: "),
    );
    input.add_fn(|this_input, _state| {
        if this_input.complete_input {
            return Action::Next;
        }
        Action::KeepSection
    });

    let mut section_list = SectionsView::new(Some(String::new()));
    section_list.child(input);
    section_list.render(&mut stdout)?;
    Ok(())
}
