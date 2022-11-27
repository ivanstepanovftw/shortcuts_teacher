use std::fmt;
use std::ops::Deref;
use rdev::Key;

#[derive(Clone)]
pub struct MyKey(pub Key);

impl fmt::Display for MyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Key::ControlLeft | Key::ControlRight => write!(f, "Ctrl"),
            Key::KeyC => write!(f, "C"),
            Key::KeyV => write!(f, "V"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl From<Key> for MyKey {
    fn from(key: Key) -> Self {
        Self(key)
    }
}

impl From<MyKey> for Key {
    fn from(key: MyKey) -> Self {
        key.0
    }
}

impl PartialEq for MyKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Deref for MyKey {
    type Target = Key;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
