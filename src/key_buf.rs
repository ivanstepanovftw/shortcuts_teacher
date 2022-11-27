use std::ops::{Deref, DerefMut};
use crate::key::MyKey;
use crate::prelude::*;

pub struct KeyBuf(pub Vec<MyKey>);

impl KeyBuf {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Deref for KeyBuf {
    type Target = Vec<MyKey>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KeyBuf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// impl to convert to Shortcut
impl From<&KeyBuf> for Shortcut {
    fn from(key_buf: &KeyBuf) -> Self {
        Self(key_buf.0.clone())
    }
}
