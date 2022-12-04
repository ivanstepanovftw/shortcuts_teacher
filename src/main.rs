#![feature(string_leak)]
mod key;
mod key_buf;
mod shortcut;
mod shortcuts;
mod prelude;

use prelude::*;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::thread;
use notify_rust::Notification;
use rdev::Key;
use std::time::Duration;
use rdev::EventType::{KeyPress, KeyRelease};
use i18nx::t;
use sys_locale::get_locale;


fn main() {
    let current_locale = &get_locale().unwrap_or_default().leak()[..2];
    i18nx::locale!(current_locale);

    let shortcuts_to_learn = Shortcuts::new(vec![
        Shortcut::new(vec![Key::ControlLeft.into(), Key::KeyA.into()]),
        Shortcut::new(vec![Key::ControlLeft.into(), Key::KeyC.into()]),
        Shortcut::new(vec![Key::ControlLeft.into(), Key::KeyV.into()]),
    ]);
    let key_buf = Arc::new(Mutex::new(KeyBuf::new()));
    let mut clipboard = arboard::Clipboard::new().unwrap();

    let keyboard_cb = move |event: rdev::Event| {
        match event.event_type {
            KeyPress(key) => {
                key_buf.lock().unwrap().push(key.into());
            }
            KeyRelease(key) => {
                let shortcut = Shortcut::from(key_buf.lock().unwrap().deref());
                if let Some(valid_shortcut) = shortcuts_to_learn.find_unordered(&shortcut) {
                    if *valid_shortcut == shortcut {
                        if *valid_shortcut == Shortcut::new(vec![Key::ControlLeft.into(), Key::KeyC.into()]) {
                            // Sleep to allow clipboard to update
                            thread::sleep(Duration::from_millis(50));
                            Notification::new()
                                .summary(t!("You have copied!"))
                                .body(&t!("Clipboard: {}", clipboard.get_text().unwrap_or_default()))
                                .show()
                                .unwrap();
                        }
                    } else {
                        Notification::new()
                            .summary(t!("Invalid shortcut"))
                            .body(&t!("You pressed: {}.\nExpected: {}.", shortcut, valid_shortcut))
                            .show()
                            .unwrap();
                    }
                }

                key_buf.lock().unwrap().retain(|k| k.0 != key);
            }
            _ => (),
        }
    };

    if let Err(error) = listen(keyboard_cb) {
        println!("Error: {:?}", error)
    }
}
