mod key;
mod key_buf;
mod key_log;
mod shortcut;
mod shortcuts;
mod prelude;

use prelude::*;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use notify_rust::Notification;
use rdev::{listen, Event, Key};
use std::time::Instant;
use rdev::EventType::{KeyPress, KeyRelease};


fn main() {
    let shortcuts_to_learn = Shortcuts::new(vec![
        Shortcut::new(vec![Key::ControlLeft.into(), Key::KeyC.into()]),
        Shortcut::new(vec![Key::ControlLeft.into(), Key::KeyV.into()]),
    ]);
    let key_buf = Arc::new(Mutex::new(KeyBuf::new()));
    let key_log = Arc::new(Mutex::new(KeyLog::new()));

    let callback = {
        let key_buf = key_buf.clone();
        let key_log = key_log.clone();
        move |event: Event| {
            let mut key_buf = key_buf.lock().unwrap();
            let mut key_log = key_log.lock().unwrap();

            match event.event_type {
                KeyPress(key) => {
                    key_buf.push(key.into());
                    // key_log.push((*key, Instant::now()));
                },
                KeyRelease(key) => {
                    let shortcut = Shortcut::from(key_buf.deref());
                    if let Some(valid_shortcut) = shortcuts_to_learn.find_unordered(&shortcut){
                        if *valid_shortcut != shortcut {
                            Notification::new()
                                .summary("Invalid shortcut")
                                .body(&format!("You typed: {}\nExpected: {}", shortcut, valid_shortcut))
                                .show()
                                .unwrap();
                        }
                    }

                    key_buf.retain(|k| k.0 != key);
                    // key_log.remove(key); // TODO: remove after timeout
                },
                _ => (),
            }
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}
