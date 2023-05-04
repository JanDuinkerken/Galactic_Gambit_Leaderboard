use crate::domain::entities::{Entry, EntryId, EntryUsername, EntryLevel, EntrySeconds};

pub enum Insert {
    Ok(EntryId),
    Conflict,
    Error
}

pub trait Repository {
    fn insert (&mut self, id: EntryId, user: EntryUsername, level:EntryLevel, seconds: EntrySeconds) -> Insert;
}

impl Repository for InMemoryRepository{
    fn insert (&mut self, id: EntryId, user: EntryUsername, level:EntryLevel, seconds: EntrySeconds) -> Insert {
        if self.error {
            return Insert::Error;
        }

        if self.entries.iter().any(|entry| entry.id == id) {
            return Insert::Conflict;
        }

        let id_clone = id.clone();
        self.entries.push(Entry::new(id_clone, user, level, seconds));
        Insert::Ok(id)
    }
}

pub struct InMemoryRepository {
    error: bool,
    entries: Vec<Entry>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let entries: Vec<Entry> = vec![];
        Self {
            error: false, 
            entries, 
        }
    }

    pub fn with_error(self) -> Self {
        Self {
            error: true,
            ..self
        }
    }
}