// use crate::keyreceiver::{run_receiver, ReceiverEvent};
use crate::keysender::run_sender;
// use crate::systemfont::get_fontfamily_list;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystroke {
    keycode: u32,
    symbol: String,
}
impl Keystroke {
    pub fn new(keycode: u32, symbol: String) -> Self {
        Keystroke { keycode, symbol }
    }
}

impl fmt::Display for Keystroke {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key[{:#x} '{}']", self.keycode, self.symbol)
    }
}
// pub struct Keyway {
//     keys: Vec<Keystroke>,
//     windows: HashMap<window::Id, KeywayWindows>,
//     config_window: ConfigWindow,
//     key_window: KeyWindow,
//     timeout: Arc<Mutex<u16>>,
//     visible_mouse: bool,
//     is_shutdown: Arc<Mutex<bool>>,
//     theme: Theme,
//     close_icon: svg::Handle,
//     minimize_icon: svg::Handle,
//     fontsize: u16,
//     fontfamily: String,
// }
// enum KeywayWindows {
//     Configure,
//     Keydisplay,
// }
