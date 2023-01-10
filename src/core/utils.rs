use std::fmt::Display;

/// Define a Action into SectionsView:
///  - `Next`: Continue with the next section
///  - `Exit`: Finish the render
///  - `KeepSection`: keep in the same section
///
pub enum Action {
    Next,
    Exit,
    KeepSection,
}

pub struct WidgetState<T: Display>(pub T);
