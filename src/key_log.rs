use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use notify_rust::Notification;
use device_query::{DeviceQuery, DeviceEvents, DeviceState, Keycode};
use crate::prelude::*;


pub struct KeyLog(Vec<(Keycode, Instant)>);

impl KeyLog {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Deref for KeyLog {
    type Target = Vec<(Keycode, Instant)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
