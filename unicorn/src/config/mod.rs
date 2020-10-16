pub mod keys;
pub mod scancode;

use self::keys::PX8Key;
use self::scancode::{Scancode, Mod};

use std::collections::HashMap;

pub struct Mouse {
    pub x: i32,
    pub y: i32,
    pub state: u32,
    pub state_quick: u32,
    pub delay: f64,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            x: 0,
            y: 0,
            state: 0,
            state_quick: 0,
            delay: 0.,
        }
    }
}
pub struct PlayerKeys {
    frames: HashMap<PX8Key, f64>,
    keys: HashMap<PX8Key, bool>,
    keys_quick: HashMap<PX8Key, bool>,
}

impl PlayerKeys {
    pub fn new() -> PlayerKeys {
        let mut keys = HashMap::new();
        let mut keys_quick = HashMap::new();

        keys.insert(PX8Key::Right, false);
        keys.insert(PX8Key::Left, false);
        keys.insert(PX8Key::Up, false);
        keys.insert(PX8Key::Down, false);
        keys.insert(PX8Key::A, false);
        keys.insert(PX8Key::B, false);
        keys.insert(PX8Key::Pause, false);
        keys.insert(PX8Key::Enter, false);

        keys_quick.insert(PX8Key::Right, false);
        keys_quick.insert(PX8Key::Left, false);
        keys_quick.insert(PX8Key::Up, false);
        keys_quick.insert(PX8Key::Down, false);
        keys_quick.insert(PX8Key::A, false);
        keys_quick.insert(PX8Key::B, false);
        keys_quick.insert(PX8Key::Pause, false);
        keys_quick.insert(PX8Key::Enter, false);

        PlayerKeys {
            frames: HashMap::new(),
            keys: keys,
            keys_quick: keys_quick,
        }
    }

    pub fn update() {

    }
}

pub struct Players {
    pub mouse: Mouse,
    pub text: String,
    pub delta: f64,

    // keys mapped to players, available for consumption by cartridges
    pub pkeys: HashMap<u8, PlayerKeys>,
    // keys down by scancode
    pub akeys: HashMap<Scancode, bool>,
    // keys pressed by scancode (only true for the first frame it's pressed)
    pub akeys_quick: HashMap<Scancode, bool>,
}

impl Players {
    pub fn new() -> Players {
        let mut keys = HashMap::new();
        keys.insert(0, PlayerKeys::new());
        keys.insert(1, PlayerKeys::new());

        Players {
            pkeys: keys,
            mouse: Mouse::new(),
            akeys: HashMap::new(),
            akeys_quick: HashMap::new(),
            text: "".to_string(),
            delta: 0.1,
        }
    }

    pub fn clear_text(&mut self) {
        self.text = "".to_string();
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn get_text(&mut self) -> String {
        self.text.clone()
    }

    pub fn set_mouse_x(&mut self, x: i32) {
        self.mouse.x = x;
    }

    pub fn set_mouse_y(&mut self, y: i32) {
        self.mouse.y = y;
    }

    pub fn mouse_button_down(&mut self, left: bool, right: bool, middle: bool, elapsed: f64) {
        self.mouse.state = 0;

        if left {
            self.mouse.state = 1;
        } else if right {
            self.mouse.state = 2;
        } else if middle {
            self.mouse.state = 4;
        }

        self.mouse.state_quick = self.mouse.state;
        self.mouse.delay = elapsed;
    }

    pub fn mouse_button_up(&mut self) {
        self.mouse.state = 0;
        self.mouse.state_quick = 0;
    }

    pub fn update(&mut self, elapsed: f64) {
        if elapsed - self.mouse.delay > self.delta {
            self.mouse.state = 0;
        }

        // Flip off all the keys in akeys_quick so they're not on for more than one frame
        // This is probably susceptible to a race condition where the key gets pressed in
        // the middle of a frame
        for (_, value) in self.akeys_quick.iter_mut() {
            if *value {
                *value = false;
            }
        }
        // same for players.keys_quick
        for (_, keys) in self.pkeys.iter_mut() {
            let pkeys_quick = &mut keys.keys_quick;
            for (_, value) in pkeys_quick.iter_mut() {
                if *value {
                    *value = false;
                }
            }
        }
    }

    pub fn key_down(&mut self, keymod: Mod, scancode: Scancode) {
        debug!("SCANCODE {:?} {:?} -> DOWN",
               keymod,
               scancode);

        let mut scancode = scancode;

        if keymod == Mod::LCTRLMOD || keymod == Mod::RCTRLMOD || keymod == Mod::LGUIMOD ||
           keymod == Mod::RGUIMOD {
            if scancode == Scancode::C {
                scancode = Scancode::Copy;
            } else if scancode == Scancode::V {
                scancode = Scancode::Paste;
            } else if scancode == Scancode::X {
                scancode = Scancode::Cut;
            }
        }

        self.akeys.insert(scancode, true);
        self.akeys_quick.insert(scancode, true);

        if let (Some(key), player) = self::keys::map_keycode(scancode) {
            self.key_down_direct(player, key);
        }
    }

    pub fn key_down_direct(&mut self, player: u8, key: PX8Key) {
        debug!("KEY {:?} Player {:?} -> DOWN",
               key,
               player);

        match self.pkeys.get_mut(&player) {
            Some(keys) => {
                keys.keys.insert(key, true);
                keys.keys_quick.insert(key, true);
            }
            None => (),
        }
    }

    pub fn key_direc_hor_up(&mut self, player: u8) {
        match self.pkeys.get_mut(&player) {
            Some(keys) => {
                keys.keys.insert(PX8Key::Right, false);
                keys.keys.insert(PX8Key::Left, false);
            }
            None => (),
        }
    }

    pub fn key_direc_ver_up(&mut self, player: u8) {
        match self.pkeys.get_mut(&player) {
            Some(keys) => {
                keys.keys.insert(PX8Key::Up, false);
                keys.keys.insert(PX8Key::Down, false);
            }
            None => (),
        }
    }

    pub fn key_up(&mut self, keymod: Mod, scancode: Scancode) {
        debug!("SCANCODE {:?} UP", scancode);

        let mut scancode = scancode;

        if keymod == Mod::LCTRLMOD || keymod == Mod::RCTRLMOD || keymod == Mod::LGUIMOD ||
           keymod == Mod::RGUIMOD {
            if scancode == Scancode::C {
                scancode = Scancode::Copy;
            } else if scancode == Scancode::V {
                scancode = Scancode::Paste;
            } else if scancode == Scancode::X {
                scancode = Scancode::Cut;
            }
        }

        self.akeys.insert(scancode, false);
        self.akeys_quick.insert(scancode, false);

        if let (Some(key), player) = self::keys::map_keycode(scancode) {
            self.key_up_direct(player, key);
        }
    }

    pub fn key_up_direct(&mut self, player: u8, key: PX8Key) {
        debug!("KEY {:?} Player {:?} -> UP", key, player);

        match self.pkeys.get_mut(&player) {
            Some(keys) => {
                keys.keys.insert(key, false);
                keys.keys_quick.insert(key, false);
            }
            None => (),
        }
    }

    pub fn get_value(&self, player: u8, index: u8) -> bool {
        match self.pkeys.get(&player) {
            Some(keys) => {
                match index {
                    0 if keys.keys[&PX8Key::Left] => true,
                    1 if keys.keys[&PX8Key::Right] => true,
                    2 if keys.keys[&PX8Key::Up] => true,
                    3 if keys.keys[&PX8Key::Down] => true,
                    4 if keys.keys[&PX8Key::A] => true,
                    5 if keys.keys[&PX8Key::B] => true,
                    6 if keys.keys[&PX8Key::Enter] => true,
                    7 if keys.keys[&PX8Key::Pause] => true,
                    _ => false,
                }
            }
            None => false,
        }
    }


    pub fn get_value_quick(&self, player: u8, index: u8) -> bool {
        match self.pkeys.get(&player) {
            Some(keys) => {
                match index {
                    0 if keys.keys_quick[&PX8Key::Left] => true,
                    1 if keys.keys_quick[&PX8Key::Right] => true,
                    2 if keys.keys_quick[&PX8Key::Up] => true,
                    3 if keys.keys_quick[&PX8Key::Down] => true,
                    4 if keys.keys_quick[&PX8Key::A] => true,
                    5 if keys.keys_quick[&PX8Key::B] => true,
                    6 if keys.keys_quick[&PX8Key::Enter] => true,
                    7 if keys.keys_quick[&PX8Key::Pause] => true,
                    _ => false,
                }
            }
            None => false,
        }
    }

    pub fn btn(&self, player: u8, index: u8) -> bool {
        self.get_value(player, index)
    }

    pub fn btnp(&self, player: u8, index: u8) -> bool {
        self.get_value_quick(player, index)
    }

    pub fn btn_raw(&self, scancode: Scancode) -> bool {
        match self.akeys.get(&scancode) {
            Some(value) => *value,
            None => false,
        }
    }

    pub fn btnp_raw(&self, scancode: Scancode) -> bool {
        match self.akeys_quick.get(&scancode) {
            Some(value) => *value,
            None => false,
        }
    }

    pub fn mouse_coordinate(&self, index: u8) -> i32 {
        match index {
            0 => self.mouse.x,
            1 => self.mouse.y,
            _ => 0,
        }
    }

    pub fn mouse_state(&self) -> u32 {
        self.mouse.state
    }

    pub fn mouse_state_quick(&self) -> u32 {
        self.mouse.state_quick
    }
}