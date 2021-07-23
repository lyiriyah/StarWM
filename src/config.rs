// Config.rs - Handles configuration of the editor
use crate::key::Key;
use crate::StarMan;
use std::collections::HashMap;

// This is a function or closure that is run on a key press event
pub type Handler = fn(&mut StarMan) -> ();

// Configuration that holds the key bindings within the window manager
pub struct Config {
    pub key_bindings: HashMap<Key, Handler>,
}

impl Config {
    pub fn new() -> Self {
        // Start a fresh configuration struct
        Self {
            key_bindings: HashMap::new(),
        }
    }

    pub fn bind_handler(&mut self, key: Key, handler: Handler) {
        // Add a key binding and a handler function to the configuration
        self.key_bindings.insert(key, handler);
    }

    pub fn key(&self, key: &Key) -> Option<&Handler> {
        // Get a handler function when a key binding occurs
        self.key_bindings.get(key)
    }
}
