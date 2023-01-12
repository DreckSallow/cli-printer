use std::io;

use crate::{
    core::{interfaces::WidgetRoot, utils::Action, view::SectionsView},
    widgets,
};

pub fn render_list_view() -> io::Result<()> {
    let mut stdout = io::stdout();
    let options = vec!["Option1", "Option2", "Option3", "Nothing"];

    let mut list_selected = widgets::list_selected::ListSelected::new(options);
    list_selected.add_text_init("? ", "Choose an option: ");
    list_selected.add_text_final("√ ", "Option selected: ");

    list_selected.add_fn(|_list_state, is_selected| {
        if *is_selected {
            return Action::Next;
        }
        Action::KeepSection
    });

    let mut section_list = SectionsView::new();
    section_list.child(list_selected);
    section_list.render(&mut stdout)?;
    Ok(())
}
