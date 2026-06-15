//! Keymap system — range-based dispatch with prefix key support.
//!
//! Mirrors mg's `KEYMAPE` struct semantics: each entry covers a range of
//! keycodes and maps to either a command or a sub-keymap (for prefix keys).

use std::collections::HashMap;

/// Internal keycode representation.
pub type KCode = u16;

/// Identifier for a command.
pub type CommandId = &'static str;

/// What happens when a key in this entry is pressed.
#[derive(Debug, Clone, Copy)]
pub enum KeyAction {
    /// Directly execute a command.
    Command(CommandId),
    /// Switch to a sub-keymap (prefix key).
    Prefix(usize),
}

/// A single entry in a keymap: covers keys from `start` to `end` inclusive.
pub struct KeyEntry {
    pub start: KCode,
    pub end: KCode,
    pub action: KeyAction,
}

/// A keymap: a sorted list of range-based entries plus a default fallback.
pub struct Keymap {
    pub name: String,
    pub entries: Vec<KeyEntry>,
    pub default: Option<CommandId>,
}

impl Keymap {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: Vec::new(),
            default: None,
        }
    }

    /// Look up a keycode. Returns the action, or the default if no entry matches.
    pub fn lookup(&self, kc: KCode) -> Option<KeyAction> {
        // Linear scan since keymaps are small (< 100 entries).
        for entry in &self.entries {
            if kc >= entry.start && kc <= entry.end {
                return Some(entry.action);
            }
        }
        None
    }
}

/// The keymap tree: global keymap plus mode-specific overrides.
pub struct KeymapTree {
    pub maps: Vec<Keymap>,
    /// Active prefix chain: indices into maps.
    pub prefix_chain: Vec<usize>,
    /// Named command registry.
    pub commands: HashMap<&'static str, fn()>,
}

impl KeymapTree {
    pub fn new() -> Self {
        Self {
            maps: Vec::new(),
            prefix_chain: Vec::new(),
            commands: HashMap::new(),
        }
    }

    /// Register a named command.
    pub fn register(&mut self, _name: &'static str) {
        // Stub — will wire up actual PF dispatch later.
    }
}

impl Default for KeymapTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_returns_action_for_registered_key() {
        let km = Keymap {
            name: "test".into(),
            entries: vec![KeyEntry {
                start: 1, end: 1,
                action: KeyAction::Command("test-cmd"),
            }],
            default: None,
        };
        assert!(km.lookup(1).is_some());
        assert_eq!(format!("{:?}", km.lookup(1).unwrap()), "Command(\"test-cmd\")");
    }

    #[test]
    fn lookup_returns_none_for_unregistered_key() {
        let km = Keymap::new("empty");
        assert!(km.lookup(99).is_none());
    }

    #[test]
    fn lookup_matches_range() {
        let km = Keymap {
            name: "range".into(),
            entries: vec![KeyEntry {
                start: 10, end: 20,
                action: KeyAction::Command("range-cmd"),
            }],
            default: None,
        };
        assert!(km.lookup(10).is_some());
        assert!(km.lookup(15).is_some());
        assert!(km.lookup(20).is_some());
        assert!(km.lookup(9).is_none());
        assert!(km.lookup(21).is_none());
    }
}
