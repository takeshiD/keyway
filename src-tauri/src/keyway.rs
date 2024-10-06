use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystroke {
    virtkey: u32,
    symbol: String, 
}

impl Keystroke {
    pub fn new(virtkey: u32, symbol: String) -> Self {
        Keystroke {
            virtkey,
            symbol,
        }
    }
    pub fn is_mod(&self) -> bool {
        unimplemented!()
    }
}

impl fmt::Display for Keystroke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key[{:#x} '{}']", self.virtkey, self.symbol)
    }
}

