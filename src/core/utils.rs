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

pub struct IconAndLabel<'a>(pub &'a str, pub &'a str);
