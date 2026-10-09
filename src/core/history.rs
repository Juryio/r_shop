use crate::core::document::Document;

pub trait Command {
    fn execute(&mut self, document: &mut Document);
    fn undo(&mut self, document: &mut Document);
}

pub struct History {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn execute_command(&mut self, mut command: Box<dyn Command>, document: &mut Document) {
        command.execute(document);
        self.undo_stack.push(command);
        self.redo_stack.clear(); // Clear redo stack on new action
    }

    pub fn undo(&mut self, document: &mut Document) {
        if let Some(mut command) = self.undo_stack.pop() {
            command.undo(document);
            self.redo_stack.push(command);
        }
    }

    pub fn redo(&mut self, document: &mut Document) {
        if let Some(mut command) = self.redo_stack.pop() {
            command.execute(document);
            self.undo_stack.push(command);
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
