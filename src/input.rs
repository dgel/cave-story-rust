use sdl2::keyboard::Keycode;
use std::collections::BTreeMap;

struct KeyState {
    held: bool,
    pressed: bool,
    released: bool,
}

impl KeyState {
    fn new() -> Self {
        KeyState {
            held: false,
            pressed: false,
            released: false,
        }
    }
}

pub struct Input {
    input_keys: BTreeMap<i32, KeyState>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            input_keys: BTreeMap::new(),
        }
    }
    pub fn begin_new_frame(&mut self) {
        for (_, state) in self.input_keys.iter_mut() {
            state.pressed = false;
            state.released = false;
        }
    }

    pub fn on_key_down(&mut self, key: Keycode) {
        let state = self.input_keys.entry(key as i32).or_insert(KeyState::new());
        if !state.held {
            state.pressed = true;
            state.held = true;
        }
    }

    pub fn on_key_up(&mut self, key: Keycode) {
        let state = self.input_keys.entry(key as i32).or_insert(KeyState::new());
        state.released = true;
        state.held = false;
    }

    pub fn key_held(&self, key: Keycode) -> bool {
        self.input_keys
            .get(&(key as i32))
            .map_or(false, |state| state.held)
    }

    pub fn key_pressed(&self, key: Keycode) -> bool {
        self.input_keys
            .get(&(key as i32))
            .map_or(false, |state| state.pressed)
    }

    pub fn key_released(&self, key: Keycode) -> bool {
        self.input_keys
            .get(&(key as i32))
            .map_or(false, |state| state.released)
    }
}
