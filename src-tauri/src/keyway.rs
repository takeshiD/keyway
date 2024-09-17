// use crate::systemfont::get_fontfamily_list;
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystroke {
    physcode: u32,
    keycode: u32,
    symbol: String,
}

impl Keystroke {
    pub fn new(physcode: u32, keycode: u32, symbol: String) -> Self {
        Keystroke { physcode, keycode, symbol }
    }
    pub fn is_mod(&self) -> bool {
        false
    }
}

impl fmt::Display for Keystroke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key[{:#x} '{}']", self.keycode, self.symbol)
    }
}

enum InputEvent {
    KEYDOWN(u32),
    KEYUP(u32),
}
