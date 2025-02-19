use crate::library::entry::BibEntry;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Serialize, Deserialize)]
pub struct Catalog(HashMap<String, BibEntry>);

impl Deref for Catalog {
    type Target = HashMap<String, BibEntry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Catalog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Catalog {
    pub fn new() -> Self {
        Catalog(HashMap::new())
    }
}
