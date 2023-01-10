use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use super::{
    interfaces::{WidgetChild, WidgetRoot},
    utils::{Action, WidgetState},
};

type Intersection<'a> = Box<dyn FnMut(&WidgetState<String>) -> Action + 'a>;

type Section<'a> = (Box<dyn WidgetChild + 'a>, Intersection<'a>);

/// SectionsView is like a list, that render all widgets child, such as sections
pub struct SectionsView<'a> {
    sections: Vec<Section<'a>>,
    action: Action,
    max: usize,
}

impl<'a> WidgetRoot for SectionsView<'a> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(stdout, EnterAlternateScreen)?;
        loop {
            if self.max > self.sections.len() {
                break;
            }
            if let Action::Exit = self.action {
                break;
            }
            if let Action::Next = self.action {
                self.action = Action::KeepSection;
                self.max += 1;
            }

            for section in &mut self.sections[0..self.max] {
                section.0.render(stdout)?; // WidgetChild rendering

                // Handle the previus state and return the new Action State
                self.action = (section.1)(section.0.get_state());
            }
        }

        execute!(stdout, LeaveAlternateScreen)?;
        Ok(())
    }
}

impl<'a> SectionsView<'a> {
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            action: Action::KeepSection,
            max: 0,
        }
    }
    pub fn child(&mut self, child: impl WidgetChild + 'a, cb: Intersection<'a>) {
        if self.max != 0 {
            self.max = 1;
        }
        self.sections.push((Box::new(child), Box::new(cb)));
    }
}
