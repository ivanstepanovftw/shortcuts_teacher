use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use notify_rust::Notification;
use device_query::{DeviceQuery, DeviceEvents, DeviceState, Keycode};
use crate::prelude::*;

#[derive(PartialEq)]
pub struct Shortcut(pub Vec<Keycode>);

impl Shortcut {
    pub fn new(keys: Vec<Keycode>) -> Self {
        Self(keys)
    }
}

impl Deref for Shortcut {
    type Target = Vec<Keycode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.iter().map(|key| key.to_string()).collect::<Vec<String>>().join(" + "))
    }
}
