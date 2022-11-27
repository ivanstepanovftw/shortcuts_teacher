mod prelude;
mod key_log;
mod shortcut;
mod shortcuts;
mod key_buf;

use std::ops::Deref;
use prelude::*;
use std::sync::{Arc, Mutex};
use notify_rust::Notification;
use device_query::{DeviceEvents, DeviceState, Keycode};
use std::time::Instant;

fn main() {
    let shortcuts_to_learn = Shortcuts::new(vec![
        Shortcut::new(vec![Keycode::LControl, Keycode::C]),
        Shortcut::new(vec![Keycode::LControl, Keycode::V]),
    ]);
    let key_buf = Arc::new(Mutex::new(KeyBuf::new()));
    let key_log = Arc::new(Mutex::new(KeyLog::new()));

    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down({
        let key_buf = key_buf.clone();
        let key_log = key_log.clone();
        move |key| {
            let mut key_buf = key_buf.lock().unwrap();
            let mut key_log = key_log.lock().unwrap();

            key_buf.push(*key);
            // key_log.push((*key, Instant::now()));
        }
    });
    let _guard = device_state.on_key_up({
        let key_buf = key_buf.clone();
        let key_log = key_log.clone();
        move |key| {
            let mut key_buf = key_buf.lock().unwrap();
            let mut key_log = key_log.lock().unwrap();

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

            key_buf.retain(|&k| k != *key);
            // key_log.remove(key); // TODO: remove after timeout
        }
    });

    loop {
        let sleep_time = std::time::Duration::from_millis(1000);
        std::thread::sleep(sleep_time);
        dbg!("sleeping");
    }
}
