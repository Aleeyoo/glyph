//! Config file loading: ~/.config/glyph/config.mg
//! Evaluates the config file as miniLisp if it exists.

use std::fs;
use crate::Editor;

/// Load and evaluate the user config file.
pub fn load_config(ed: &mut Editor) {
    let config_path = dirs_config_path();
    match fs::read_to_string(&config_path) {
        Ok(content) => {
            match super::engine::read(&content) {
                Ok(_val) => {
                    ed.echo_line = format!("Loaded config: {}", config_path);
                }
                Err(e) => {
                    ed.echo_line = format!("Config error: {}", e);
                }
            }
        }
        Err(_) => {
            // Config file doesn't exist — that's fine
        }
    }
}

fn dirs_config_path() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    format!("{}/.config/glyph/config.mg", home)
}
