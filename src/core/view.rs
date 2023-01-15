use std::{cell::RefCell, rc::Rc};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::{
    interfaces::{WidgetChild, WidgetRoot},
    utils::{Action, RenderWidget},
};

/// SectionsView is like a list, that render all widgets child, such as sections
pub struct SectionsView<'a, T: Clone> {
    sections: Vec<Box<dyn WidgetChild<Rc<RefCell<T>>> + 'a>>,
    action: Action,
    max: usize,
    pub global_state: Rc<RefCell<T>>,
}

impl<'a, T: Clone> WidgetRoot for SectionsView<'a, T> {
    fn render(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<()> {
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        execute!(stdout, EnterAlternateScreen)?;
        'outer: loop {
            execute!(stdout, MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;

            if self.action == Action::Exit {
                break;
            }

            if self.action == Action::Next {
                self.action = Action::KeepSection;
                if self.max + 1 > self.sections.len() {
                    break;
                }
                self.max += 1;
            }

            if self.max > self.sections.len() {
                break;
            }

            let mut count = 0;
            for section in &mut self.sections[0..self.max] {
                if count + 1 == self.max {
                    let render_widget = section.before_render(Rc::clone(&self.global_state));
                    if render_widget == RenderWidget::No {
                        // If not will render widget so, then break the loop
                        break 'outer;
                    }
                }

                section.render(stdout)?; // WidgetChild rendering

                if count + 1 == self.max {
                    self.action = section.after_render(Rc::clone(&self.global_state));
                }
                execute!(stdout, Print("\n"))?;
                count += 1;
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

impl<'a, T: Clone> SectionsView<'a, T> {
    pub fn new(global_state: T) -> Self {
        Self {
            sections: Vec::new(),
            action: Action::KeepSection,
            max: 0,
            global_state: Rc::new(RefCell::new(global_state)),
        }
    }
    pub fn child(&mut self, child: impl WidgetChild<Rc<RefCell<T>>> + 'a) {
        if self.max == 0 {
            self.max = 1;
        }
        self.sections.push(Box::new(child));
    }
}
