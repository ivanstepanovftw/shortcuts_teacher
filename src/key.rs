use std::fmt;
use std::ops::Deref;
use rdev::Key;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct MyKey(pub Key);

impl fmt::Display for MyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            // Key::Alt => write!(f, "Alt"),
            // Key::AltGr => write!(f, "AltGr"),
            Key::Alt | Key::AltGr => write!(f, "Alt"),
            Key::Backspace => write!(f, "Backspace"),
            Key::CapsLock => write!(f, "CapsLock"),
            // Key::ControlLeft => write!(f, "ControlLeft"),
            // Key::ControlRight => write!(f, "ControlRight"),
            Key::ControlLeft | Key::ControlRight => write!(f, "Ctrl"),
            Key::Delete => write!(f, "Delete"),
            Key::DownArrow => write!(f, "DownArrow"),
            Key::End => write!(f, "End"),
            Key::Escape => write!(f, "Escape"),
            Key::F1 => write!(f, "F1"),
            Key::F10 => write!(f, "F10"),
            Key::F11 => write!(f, "F11"),
            Key::F12 => write!(f, "F12"),
            Key::F2 => write!(f, "F2"),
            Key::F3 => write!(f, "F3"),
            Key::F4 => write!(f, "F4"),
            Key::F5 => write!(f, "F5"),
            Key::F6 => write!(f, "F6"),
            Key::F7 => write!(f, "F7"),
            Key::F8 => write!(f, "F8"),
            Key::F9 => write!(f, "F9"),
            Key::Home => write!(f, "Home"),
            Key::LeftArrow => write!(f, "LeftArrow"),
            Key::MetaLeft => write!(f, "MetaLeft"),
            Key::MetaRight => write!(f, "MetaRight"),
            Key::PageDown => write!(f, "PageDown"),
            Key::PageUp => write!(f, "PageUp"),
            Key::Return => write!(f, "Return"),
            Key::RightArrow => write!(f, "RightArrow"),
            // Key::ShiftLeft => write!(f, "ShiftLeft"),
            // Key::ShiftRight => write!(f, "ShiftRight"),
            Key::ShiftLeft | Key::ShiftRight => write!(f, "Shift"),
            Key::Space => write!(f, "Space"),
            Key::Tab => write!(f, "Tab"),
            Key::UpArrow => write!(f, "UpArrow"),
            Key::PrintScreen => write!(f, "PrintScreen"),
            Key::ScrollLock => write!(f, "ScrollLock"),
            Key::Pause => write!(f, "Pause"),
            Key::NumLock => write!(f, "NumLock"),
            Key::BackQuote => write!(f, "BackQuote"),
            Key::Num1 => write!(f, "1"),
            Key::Num2 => write!(f, "2"),
            Key::Num3 => write!(f, "3"),
            Key::Num4 => write!(f, "4"),
            Key::Num5 => write!(f, "5"),
            Key::Num6 => write!(f, "6"),
            Key::Num7 => write!(f, "7"),
            Key::Num8 => write!(f, "8"),
            Key::Num9 => write!(f, "9"),
            Key::Num0 => write!(f, "0"),
            Key::Minus => write!(f, "Minus"),
            Key::Equal => write!(f, "Equal"),
            Key::KeyQ => write!(f, "Q"),
            Key::KeyW => write!(f, "W"),
            Key::KeyE => write!(f, "E"),
            Key::KeyR => write!(f, "R"),
            Key::KeyT => write!(f, "T"),
            Key::KeyY => write!(f, "Y"),
            Key::KeyU => write!(f, "U"),
            Key::KeyI => write!(f, "I"),
            Key::KeyO => write!(f, "O"),
            Key::KeyP => write!(f, "P"),
            Key::LeftBracket => write!(f, "LeftBracket"),
            Key::RightBracket => write!(f, "RightBracket"),
            Key::KeyA => write!(f, "A"),
            Key::KeyS => write!(f, "S"),
            Key::KeyD => write!(f, "D"),
            Key::KeyF => write!(f, "F"),
            Key::KeyG => write!(f, "G"),
            Key::KeyH => write!(f, "H"),
            Key::KeyJ => write!(f, "J"),
            Key::KeyK => write!(f, "K"),
            Key::KeyL => write!(f, "L"),
            Key::SemiColon => write!(f, "SemiColon"),
            Key::Quote => write!(f, "Quote"),
            Key::BackSlash => write!(f, "BackSlash"),
            Key::IntlBackslash => write!(f, "IntlBackslash"),
            Key::KeyZ => write!(f, "Z"),
            Key::KeyX => write!(f, "X"),
            Key::KeyC => write!(f, "C"),
            Key::KeyV => write!(f, "V"),
            Key::KeyB => write!(f, "B"),
            Key::KeyN => write!(f, "N"),
            Key::KeyM => write!(f, "M"),
            Key::Comma => write!(f, "Comma"),
            Key::Dot => write!(f, "Dot"),
            Key::Slash => write!(f, "Slash"),
            Key::Insert => write!(f, "Insert"),
            Key::KpReturn => write!(f, "KpReturn"),
            Key::KpMinus => write!(f, "KpMinus"),
            Key::KpPlus => write!(f, "KpPlus"),
            Key::KpMultiply => write!(f, "KpMultiply"),
            Key::KpDivide => write!(f, "KpDivide"),
            Key::Kp0 => write!(f, "Kp0"),
            Key::Kp1 => write!(f, "Kp1"),
            Key::Kp2 => write!(f, "Kp2"),
            Key::Kp3 => write!(f, "Kp3"),
            Key::Kp4 => write!(f, "Kp4"),
            Key::Kp5 => write!(f, "Kp5"),
            Key::Kp6 => write!(f, "Kp6"),
            Key::Kp7 => write!(f, "Kp7"),
            Key::Kp8 => write!(f, "Kp8"),
            Key::Kp9 => write!(f, "Kp9"),
            Key::KpDelete => write!(f, "KpDelete"),
            Key::Function => write!(f, "Function"),
            _ => write!(f, "{:?}", self.0),
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

impl Deref for MyKey {
    type Target = Key;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
