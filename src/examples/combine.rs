use std::{cell::RefCell, io, rc::Rc};

use crate::{
    core::{
        interfaces::WidgetRoot,
        utils::{Action, IconAndLabel},
        view::SectionsView,
    },
    styles::{ICON_CHECK, ICON_QUESTION},
    widgets::{Input, ListSelected},
};

#[derive(Clone, Debug)]
struct GlobalState {
    pub option_list: Option<String>,
    pub input_user: Option<String>,
}

pub fn combine_widgets() -> io::Result<()> {
    let mut stdout = io::stdout();
    let options = vec!["Option1", "Option2", "Option3", "Nothing"];

    let mut list_selected: ListSelected<Rc<RefCell<GlobalState>>> = ListSelected::new(options);
    list_selected.add_text_init("? ", "Choose an option: ");
    list_selected.add_text_final("√ ", "Option selected: ");

    list_selected.add_fn(|list_state, context_state| {
        if list_state.is_selected {
            if list_state.offset == list_state.length - 1 {
                context_state.borrow_mut().option_list = None;
            } else {
                context_state.borrow_mut().option_list = list_state.current_option;
            }
            return Action::Next;
        }
        Action::KeepSection
    });

    let mut input_user: Input<Rc<RefCell<GlobalState>>> = Input::new(
        IconAndLabel(ICON_QUESTION, "Write your name: "),
        IconAndLabel(ICON_CHECK, "Your name is: "),
    );

    input_user.add_fn(|this_input, context_state| {
        if this_input.complete_input && this_input.input.len() > 0 {
            context_state.borrow_mut().input_user = Some(this_input.input);
            return Action::Next;
        }
        Action::KeepSection
    });

    let mut section_list = SectionsView::new(GlobalState {
        option_list: None,
        input_user: None,
    });

    section_list.child(input_user);
    section_list.child(list_selected);
    section_list.render(&mut stdout)?;
    Ok(())
}
