use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::{
    interfaces::{WidgetChild, WidgetRoot},
    utils::Action,
};

/// SectionsView is like a list, that render all widgets child, such as sections
pub struct SectionsView<'a> {
    sections: Vec<Box<dyn WidgetChild + 'a>>,
    action: Action,
    max: usize,
}

impl<'a> WidgetRoot for SectionsView<'a> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        execute!(stdout, EnterAlternateScreen)?;
        loop {
            execute!(stdout, Clear(ClearType::FromCursorDown), MoveTo(0, 0))?;
            if let Action::Exit = self.action {
                break;
            }
            execute!(stdout, Clear(ClearType::FromCursorDown), MoveTo(0, 0))?;

            if let Action::Next = self.action {
                self.action = Action::KeepSection;
                if self.max + 1 > self.sections.len() {
                    self.action = Action::Exit;
                    break;
                }
                self.max += 1;
            }

            if self.max > self.sections.len() {
                self.action = Action::Exit;
                break;
            }

            for section in &mut self.sections[0..self.max] {
                section.render(stdout)?; // WidgetChild rendering
                execute!(stdout, Print("\n"))?;
                self.action = section.do_any()
                // Handle the previus state and return the new Action State
            }
        }
        execute!(stdout, LeaveAlternateScreen)?;
        let mut count = 0;
        for section in &mut self.sections[0..self.max] {
            section.render(stdout)?; // WidgetChild rendering
            if count + 1 != self.max {
                execute!(stdout, Print("\n"))?;
            }
            count += 1;
        }

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
    pub fn child(&mut self, child: impl WidgetChild + 'a) {
        if self.max == 0 {
            self.max = 1;
        }
        self.sections.push(Box::new(child));
    }
}
