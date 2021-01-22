pub enum KeyType {
    Digit(u8),
    Character(char),
    LeftShift,
    RightShift,
    LeftShiftReleased,
    RightShiftReleased,
    Backspace,
    Enter,
    Space,
    Tick,
}
pub struct Keyboard {
    left_shift_pressed: bool,
    right_shift_pressed: bool
}

pub static mut KEYBOARD: Keyboard =  Keyboard {
    left_shift_pressed: false,
    right_shift_pressed: false
};

pub struct Key {
    pub key_type: KeyType,
}
impl Key {
    fn new(key_type: KeyType) -> Self {
        match key_type {
            KeyType::LeftShift => unsafe {KEYBOARD.left_shift_pressed = true},
            KeyType::RightShift => unsafe {KEYBOARD.right_shift_pressed = true},
            KeyType::LeftShiftReleased => unsafe {KEYBOARD.left_shift_pressed = false},
            KeyType::RightShiftReleased => unsafe {KEYBOARD.right_shift_pressed = false},
            _ => {}
        }
        Key {key_type: key_type}
    }
    pub fn to_char(self) -> Option<char> {
        let c = match self.key_type {
            KeyType::Digit(d) => ('0' as u8 + d) as char,
            KeyType::Character(c) => unsafe {
                if KEYBOARD.left_shift_pressed || KEYBOARD.right_shift_pressed {
                    match c {
                        '-' => '_',
                        _ => (c as u8 + 'A' as u8 - 'a' as u8) as char,
                    }
                }
                else {
                    c
                }
            },
            KeyType::Enter => '\n',
            KeyType::Space => ' ',
            KeyType::Backspace => 0x08 as char,
            KeyType::Tick => unsafe {
                if KEYBOARD.left_shift_pressed || KEYBOARD.right_shift_pressed {
                    '\"'
                }
                else {
                    '\''
                }
            },
            _ => return None 
        };
        Some(c)
    }
}

pub fn get_key(scancode: u8) -> Option<Key> {
    let result = match scancode {
        0x02 ..= 0x0A => Key::new(KeyType::Digit(scancode - 0x01)),
        0x0B => Key::new(KeyType::Digit(0)),
        0x0E => Key::new(KeyType::Backspace),
        0x10 => Key::new(KeyType::Character('q')),
        0x11 => Key::new(KeyType::Character('w')),
        0x12 => Key::new(KeyType::Character('e')),
        0x13 => Key::new(KeyType::Character('r')),
        0x14 => Key::new(KeyType::Character('t')),
        0x15 => Key::new(KeyType::Character('y')),
        0x16 => Key::new(KeyType::Character('u')),
        0x17 => Key::new(KeyType::Character('i')),
        0x18 => Key::new(KeyType::Character('o')),
        0x19 => Key::new(KeyType::Character('p')),
        0x1E => Key::new(KeyType::Character('a')),
        0x1F => Key::new(KeyType::Character('s')),
        0x20 => Key::new(KeyType::Character('d')),
        0x21 => Key::new(KeyType::Character('f')),
        0x22 => Key::new(KeyType::Character('g')),
        0x23 => Key::new(KeyType::Character('h')),
        0x24 => Key::new(KeyType::Character('j')),
        0x25 => Key::new(KeyType::Character('k')),
        0x26 => Key::new(KeyType::Character('l')),
        0x28 => Key::new(KeyType::Tick),
        0x2A => Key::new(KeyType::LeftShift),
        0x2B => Key::new(KeyType::Character('\\')),
        0x2C => Key::new(KeyType::Character('z')),
        0x2D => Key::new(KeyType::Character('x')),
        0x2E => Key::new(KeyType::Character('c')),
        0x2F => Key::new(KeyType::Character('v')),
        0x30 => Key::new(KeyType::Character('b')),
        0x31 => Key::new(KeyType::Character('n')),
        0x32 => Key::new(KeyType::Character('m')),
        0x33 => Key::new(KeyType::Character(',')),
        0x34 => Key::new(KeyType::Character('.')),
        0x35 => Key::new(KeyType::Character('/')),
        0x0C => Key::new(KeyType::Character('-')),
        0x36 => Key::new(KeyType::RightShift),
        0xAA => Key::new(KeyType::LeftShiftReleased),
        0xB6 => Key::new(KeyType::RightShiftReleased),
        0x1C => Key::new(KeyType::Enter),
        0x39 => Key::new(KeyType::Space),
        _ => return None,
    };
    Some(result)
}
