use std::ops::{Deref, DerefMut};
use device_query::Keycode;
use crate::prelude::*;


pub struct KeyBuf(pub Vec<Keycode>);

impl KeyBuf {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Deref for KeyBuf {
    type Target = Vec<Keycode>;

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
