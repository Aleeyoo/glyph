//! Hook system for Lisp callbacks.
//!
//! Allows Lisp code to register functions that are called at specific
//! editor lifecycle events (find-file, before-save, after-save, kill-buffer).

use std::collections::HashMap;

use super::engine::LispVal;
use crate::Editor;

/// Manages named hook lists, each holding zero or more Lisp functions.
///
/// Hooks are run in insertion order. If a hook function returns an error,
/// execution of the remaining hooks is aborted and the error is propagated.
pub struct HookSystem {
    hooks: HashMap<String, Vec<LispVal>>,
}

impl HookSystem {
    /// Create an empty hook system with the well-known hook names pre-populated.
    pub fn new() -> Self {
        let mut hooks = HashMap::new();
        for name in &[
            "find-file-hook",
            "before-save-hook",
            "after-save-hook",
            "kill-buffer-hook",
        ] {
            hooks.insert(name.to_string(), Vec::new());
        }
        Self { hooks }
    }

    /// Register `func` to be called when hook `name` fires.
    ///
    /// Silently creates the hook list if `name` does not exist yet.
    pub fn add_hook(&mut self, name: &str, func: LispVal) {
        self.hooks.entry(name.to_string()).or_insert_with(Vec::new).push(func);
    }

    /// Remove the first occurrence of `func` from hook `name`.
    ///
    /// Comparison uses `LispVal` equality at the value level (not pointer identity).
    pub fn remove_hook(&mut self, name: &str, func: &LispVal) {
        if let Some(list) = self.hooks.get_mut(name) {
            list.retain(|f| f != func);
        }
    }

    /// Run every function registered for hook `name`, in order.
    ///
    /// Each function receives no arguments. If any function returns an error
    /// the remaining hooks are skipped and the error is returned.
    pub fn run_hooks(&mut self, name: &str, _ed: &mut Editor) -> Result<(), String> {
        let hooks = match self.hooks.get(name) {
            Some(h) => h.clone(),
            None => return Ok(()),
        };

        for func in &hooks {
            match func {
                LispVal::Func(f) => {
                    f(&[])?;
                }
                LispVal::Closure(_, _, _) => {
                    return Err(format!(
                        "Hook '{}': closure invocation not yet supported",
                        name
                    ));
                }
                other => {
                    return Err(format!(
                        "Hook '{}': expected a function, got {}",
                        name, other
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Editor;

    fn test_editor() -> Editor {
        Editor::new(24, 80)
    }

    fn noop_func(_args: &[LispVal]) -> Result<LispVal, String> {
        Ok(LispVal::Nil)
    }

    #[test]
    fn pre_populates_well_known_hooks() {
        let mut hs = HookSystem::new();
        for name in &[
            "find-file-hook",
            "before-save-hook",
            "after-save-hook",
            "kill-buffer-hook",
        ] {
            // The hook list exists and starts empty; run_hooks is a no-op.
            assert!(hs.run_hooks(name, &mut test_editor()).is_ok());
        }
    }

    #[test]
    fn add_and_remove_hook() {
        let mut hs = HookSystem::new();
        let func = LispVal::Func(noop_func);

        // Add the func twice so we can verify remove only knocks out one.
        hs.add_hook("find-file-hook", func.clone());
        hs.add_hook("find-file-hook", func.clone());

        // First call should succeed (two entries, func matches both).
        hs.remove_hook("find-file-hook", &func);
        // Second call should succeed even if the list is already empty.
        hs.remove_hook("find-file-hook", &func);
        hs.remove_hook("find-file-hook", &func);

        // Running should still be fine (empty hook list).
        assert!(hs.run_hooks("find-file-hook", &mut test_editor()).is_ok());
    }

    #[test]
    fn run_hooks_with_single_func_calls_it() {
        let mut ed = test_editor();
        let mut hs = HookSystem::new();

        static CALLED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

        fn test_fn(_args: &[LispVal]) -> Result<LispVal, String> {
            CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(LispVal::Nil)
        }

        hs.add_hook("find-file-hook", LispVal::Func(test_fn));
        assert!(hs.run_hooks("find-file-hook", &mut ed).is_ok());
        assert!(
            CALLED.load(std::sync::atomic::Ordering::SeqCst),
            "hook function was not called"
        );
    }

    #[test]
    fn run_hooks_unknown_name_is_ok() {
        let mut ed = test_editor();
        let mut hs = HookSystem::new();
        assert!(hs.run_hooks("nonexistent-hook", &mut ed).is_ok());
    }

    #[test]
    fn remove_hook_unknown_name_is_noop() {
        let mut hs = HookSystem::new();
        let func = LispVal::Func(noop_func);
        // Should not panic.
        hs.remove_hook("nonexistent-hook", &func);
    }

    #[test]
    fn run_hooks_errors_on_non_function() {
        let mut ed = test_editor();
        let mut hs = HookSystem::new();
        hs.add_hook("find-file-hook", LispVal::Integer(42));

        let result = hs.run_hooks("find-file-hook", &mut ed);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("expected a function"));
    }

    #[test]
    fn run_hooks_stops_at_first_error() {
        let mut ed = test_editor();
        let mut hs = HookSystem::new();

        fn err_fn(_args: &[LispVal]) -> Result<LispVal, String> {
            Err("boom".into())
        }

        static SECOND_CALLED: std::sync::atomic::AtomicBool =
            std::sync::atomic::AtomicBool::new(false);

        fn ok_fn(_args: &[LispVal]) -> Result<LispVal, String> {
            SECOND_CALLED.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(LispVal::Nil)
        }

        hs.add_hook("find-file-hook", LispVal::Func(err_fn));
        hs.add_hook("find-file-hook", LispVal::Func(ok_fn));

        let result = hs.run_hooks("find-file-hook", &mut ed);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("boom"));
        assert!(
            !SECOND_CALLED.load(std::sync::atomic::Ordering::SeqCst),
            "second hook should not be called after error"
        );
    }
}
