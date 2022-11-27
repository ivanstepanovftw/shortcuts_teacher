use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use notify_rust::Notification;
use device_query::{DeviceQuery, DeviceEvents, DeviceState, Keycode};
use crate::prelude::*;

pub struct Shortcuts(pub Vec<Shortcut>);

impl Shortcuts {
    pub fn new(shortcuts: Vec<Shortcut>) -> Self {
        Self(shortcuts)
    }

    /// Find a shortcut in the list of shortcuts.
    // pub fn contains_subset(&self, shortcut: &Shortcut) -> bool {
    //     shortcut.iter().all(|key| self.0.contains(key))
    // }

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
