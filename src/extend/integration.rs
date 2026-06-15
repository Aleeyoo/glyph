//! Bridge between miniLisp evaluation and editor commands.
//!
//! Provides `editor-call`, a built-in function that looks up and invokes any
//! registered editor command by name. Arguments are ignored -- the command is
//! called with neutral Flags and no numeric argument.
//!
//! # Thread-locals
//!
//! The editor reference is passed via a thread-local cell so it can be made
//! available to the plain function pointer stored inside `LispVal::Func`.
//! The caller (`Editor::eval_expression`, config-file loading) is responsible
//! for setting the pointer before calling into the evaluator.

use std::cell::RefCell;

use crate::command::Flags;
use crate::extend::engine::LispVal;
use crate::Editor;

thread_local! {
    static EDITOR_PTR: RefCell<Option<*mut Editor>> = RefCell::new(None);
}

/// Run `f` with `editor` accessible to `editor-call`.
pub fn with_editor<T>(editor: &mut Editor, f: impl FnOnce() -> T) -> T {
    EDITOR_PTR.with(|cell| {
        *cell.borrow_mut() = Some(editor as *mut Editor);
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        *cell.borrow_mut() = None;
        r
    })
    .unwrap_or_else(|e| std::panic::resume_unwind(e))
}

/// Look up `name` in the command registry and call it.
///
/// Returns `LispVal::Nil` on success. Returns an error string if the command
/// is not found, or if the command function itself fails.
pub fn editor_call(_args: &[LispVal]) -> Result<LispVal, String> {
    if _args.is_empty() {
        return Err("editor-call: need a command name".into());
    }

    let name = match &_args[0] {
        LispVal::Symbol(s) | LispVal::String(s) => s.clone(),
        _ => return Err("editor-call: first argument must be a symbol or string (command name)".into()),
    };

    EDITOR_PTR.with(|cell| {
        let editor_ptr = cell.borrow();
        let editor_ptr = editor_ptr.ok_or_else(|| "editor-call: no editor context (BUG)".to_string())?;
        let editor = unsafe { &mut *editor_ptr };

        let cmd = editor.command_registry.lookup(&name)
            .ok_or_else(|| format!("editor-call: unknown command: {}", name))?;

        cmd(editor, Flags::default(), 1)
            .map_err(|e| format!("editor-call {}: {}", name, e))?;

        Ok(LispVal::Nil)
    })
}

/// Store `val` under `sym` in the editor's variable table.
/// Overwrites any existing value for that symbol.
pub fn lisp_setq(ed: &mut Editor, sym: &str, val: &LispVal) -> Result<(), String> {
    ed.variables.insert(sym.to_string(), val.clone());
    Ok(())
}

/// Store `val` under `sym` only if `sym` is not already defined.
pub fn lisp_setq_default(ed: &mut Editor, sym: &str, val: &LispVal) -> Result<(), String> {
    ed.variables.entry(sym.to_string()).or_insert_with(|| val.clone());
    Ok(())
}

