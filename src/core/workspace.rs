use crate::core::document::Document;

pub struct Workspace {
    pub documents: Vec<Document>,
    pub active_document_index: Option<usize>,
}

impl Workspace {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
            active_document_index: None,
        }
    }

    pub fn add_document(&mut self, document: Document) {
        self.documents.push(document);
        self.active_document_index = Some(self.documents.len() - 1);
    }

    pub fn close_document(&mut self, index: usize) {
        if index < self.documents.len() {
            self.documents.remove(index);

            if self.documents.is_empty() {
                self.active_document_index = None;
            } else if let Some(active) = self.active_document_index {
                if active == index {
                    // Closed active document, set to previous or 0
                    self.active_document_index = Some(if index > 0 { index - 1 } else { 0 });
                } else if active > index {
                    // Active document is after the closed one, shift index
                    self.active_document_index = Some(active - 1);
                }
            }
        }
    }

    pub fn active_document(&self) -> Option<&Document> {
        self.active_document_index.and_then(|idx| self.documents.get(idx))
    }

    pub fn active_document_mut(&mut self) -> Option<&mut Document> {
        if let Some(idx) = self.active_document_index {
            self.documents.get_mut(idx)
        } else {
            None
        }
    }

    pub fn set_active_document(&mut self, index: usize) {
        if index < self.documents.len() {
            self.active_document_index = Some(index);
        }
    }
}
