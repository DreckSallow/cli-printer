use std::io;

use crate::{
    core::{
        interfaces::WidgetRoot,
        utils::{Action, IconAndLabel},
        view::SectionsView,
    },
    styles::{ICON_CHECK, ICON_QUESTION},
    widgets::{self, input::Input},
};

pub fn combine_widgets() -> io::Result<()> {
    let mut stdout = io::stdout();
    let options = vec!["Option1", "Option2", "Option3", "Nothing"];

    let mut list_selected = widgets::list_selected::ListSelected::new(options);
    list_selected.add_text_init("? ", "Choose an option: ");
    list_selected.add_text_final("√ ", "Option selected: ");

    list_selected.add_fn(|_list_state, is_selected| {
        //Do something
        if *is_selected {
            return Action::Next;
        }
        Action::KeepSection
    });

    let mut input_user = Input::new(
        IconAndLabel(ICON_QUESTION, "Write your name: "),
        IconAndLabel(ICON_CHECK, "Your name is: "),
    );
    input_user.add_fn(|content, is_complete| {
        if *is_complete && content.len() > 0 {
            return Action::Next;
        }
        Action::KeepSection
    });
    let mut section_list = SectionsView::new();
    section_list.child(input_user);
    section_list.child(list_selected);
    section_list.render(&mut stdout)?;
    Ok(())
}
