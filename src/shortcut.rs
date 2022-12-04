use std::fmt;
use std::ops::{Deref};
use crate::key::MyKey;


#[derive(Debug, PartialEq)]
pub struct Shortcut(pub Vec<MyKey>);

impl Shortcut {
    pub fn new(keys: Vec<MyKey>) -> Self {
        Self(keys)
    }
}

impl Deref for Shortcut {
    type Target = Vec<MyKey>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.iter().map(|key| key.to_string()).collect::<Vec<String>>().join(" + "))
    }
}
