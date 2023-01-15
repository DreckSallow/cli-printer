/// Define a Action into SectionsView:
///  - `Next`: Continue with the next section
///  - `Exit`: Finish the render
///  - `KeepSection`: keep in the same section
///
#[derive(PartialEq, Clone, Debug)]
pub enum Action {
    Next,
    Exit,
    KeepSection,
}

#[derive(Debug, PartialEq)]
pub enum RenderWidget {
    Yes,
    No,
}

pub struct IconAndLabel<'a>(pub &'a str, pub &'a str);
