use std::io;

use crate::{
    core::{interfaces::WidgetRoot, utils::Action, view::SectionsView},
    widgets::ListSelected,
};

pub fn render_list_view() -> io::Result<()> {
    let mut stdout = io::stdout();

    let options = vec!["Option1", "Option2", "Option3", "Nothing"];

    let mut list_selected = ListSelected::new(options);
    list_selected.add_text_init("? ", "Choose an option: ");
    list_selected.add_text_final("√ ", "Option selected: ");

    list_selected.add_fn(|this_list, _state| {
        if this_list.is_selected {
            return Action::Next;
        }
        Action::KeepSection
    });

    let mut section_list = SectionsView::new(Some("".to_string()));
    section_list.child(list_selected);
    section_list.render(&mut stdout)?;
    Ok(())
}
