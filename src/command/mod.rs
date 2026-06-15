//! Command types and dispatch registry.

pub mod edit;
pub mod prefix;
pub mod file;
pub mod buffer;
pub mod window;
pub mod meta;
pub mod extended;
pub mod search;
pub mod shell;
pub mod advanced;
pub mod pairs;
pub mod async_io;

use crate::Editor;

/// Result type for commands.
pub type CmdResult = Result<(), Box<dyn std::error::Error>>;

/// A command function.
pub type CommandFn = fn(&mut Editor, Flags, i32) -> CmdResult;

/// Flags passed to every command.
#[derive(Debug, Clone, Copy, Default)]
pub struct Flags {
    pub has_arg: bool,     // FFARG — C-u or M-digit prefix
    pub neg_arg: bool,     // FFNEGARG — negative argument
    pub called_by_fn: bool,// FFRAND — called programmatically
}

impl Flags {
    pub fn new(f: u8) -> Self {
        Self {
            has_arg: (f & 1) != 0,
            neg_arg: (f & 2) != 0,
            called_by_fn: (f & 4) != 0,
        }
    }
}

/// A registered command.
pub struct Command {
    pub name: &'static str,
    pub func: CommandFn,
    pub doc: &'static str,
}

/// Registry of all editor commands.
#[derive(Default)]
pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn register(&mut self, name: &'static str, func: CommandFn, doc: &'static str) {
        self.commands.push(Command { name, func, doc });
    }

    pub fn lookup(&self, name: &str) -> Option<CommandFn> {
        self.commands.iter().find(|c| c.name == name).map(|c| c.func)
    }

    pub fn names(&self) -> Vec<&str> {
        self.commands.iter().map(|c| c.name).collect()
    }

    pub fn find_prefix(&self, prefix: &str) -> Vec<&str> {
        self.commands.iter()
            .filter(|c| c.name.starts_with(prefix))
            .map(|c| c.name)
            .collect()
    }
}
