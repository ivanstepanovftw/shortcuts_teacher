use std::ops::{Deref};
use crate::prelude::*;

pub struct Shortcuts(pub Vec<Shortcut>);

impl Shortcuts {
    pub fn new(shortcuts: Vec<Shortcut>) -> Self {
        Self(shortcuts)
    }

    pub fn find_unordered(&self, shortcut: &Shortcut) -> Option<&Shortcut> {
        self.iter().find(|valid_shortcut| valid_shortcut.iter().all(|key| shortcut.contains(key)))
    }
}

impl Deref for Shortcuts {
    type Target = Vec<Shortcut>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
