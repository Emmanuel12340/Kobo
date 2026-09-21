// GIVEN, do not edit. This file is part of the compiler you are handed, not the
// compiler you write. Read it anyway: what you write has to fit it.

use std::collections::HashMap;

use crate::value::Value;

/// Kobo's scope stack.
///
/// **This file is complete. Do not edit it.** It is the one piece of the compiler whose
/// design is handed to you rather than asked of you, because getting ownership right in
/// Rust is a different problem from getting scope right, and this course is about the
/// second one.
///
/// Every scope is a map from name to value, and the scopes are a plain `Vec`. Entering a
/// block pushes a map; leaving pops it. A name is looked up from the innermost scope
/// outwards, which is what makes shadowing work: the inner binding is found first and the
/// outer one is untouched underneath, waiting for the block to end.
///
/// A production language cannot do this. A closure outlives the block it was written in,
/// so its scopes have to survive the pop, and that forces every environment onto the heap
/// with reference counting to keep it alive. Kobo has no closures (spec section 4.3), and this
/// `Vec` is what that decision buys.
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
    /// Where each active call's scopes begin. A function body sees its own scopes and the
    /// globals, and nothing in between (spec section 4.3). The caller's locals are still sitting
    /// on this stack; the frame base is what makes them invisible.
    frames: Vec<usize>,
}

impl Environment {
    /// A fresh environment holds one scope, the global one, and it is never popped.
    pub fn new() -> Self {
        Environment {
            scopes: vec![HashMap::new()],
            frames: Vec::new(),
        }
    }

    /// Enter a call. The scopes below this point belong to the caller and stop being
    /// visible until `pop_frame`.
    pub fn push_frame(&mut self) {
        self.frames.push(self.scopes.len());
        self.scopes.push(HashMap::new());
    }

    /// Leave a call, discarding every scope it pushed, including any left behind by a
    /// `return` out of the middle of a block.
    pub fn pop_frame(&mut self) {
        if let Some(base) = self.frames.pop() {
            self.scopes.truncate(base);
        }
    }

    /// How many calls are active. The interpreter caps it: Kobo's call stack is the Rust
    /// call stack, and running off the end of that aborts the process.
    pub fn frame_depth(&self) -> usize {
        self.frames.len()
    }

    /// The lowest scope the running code may see. Zero at the top level, where everything
    /// on the stack is in scope.
    fn frame_base(&self) -> usize {
        *self.frames.last().unwrap_or(&0)
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Bind `name` in the innermost scope. An existing binding in that same scope is
    /// replaced (spec section 4.2 allows a global to be redeclared) and a binding of the same
    /// name in an enclosing scope is left alone, shadowed until this scope is popped.
    pub fn declare(&mut self, name: &str, value: Value) {
        self.innermost().insert(name.to_string(), value);
    }

    /// Read a local the resolver already placed. `depth` is scopes out from the
    /// innermost, so there is no search and no frame check: `resolver.rs` walked the
    /// same lexical structure and cannot have named a scope this frame does not own.
    pub fn get_at(&self, depth: usize, name: &str) -> Option<&Value> {
        let i = self.scopes.len().checked_sub(depth + 1)?;
        self.scopes[i].get(name)
    }

    /// Assign to a local the resolver already placed. `false` only if the binding has
    /// gone, which resolution makes impossible.
    pub fn assign_at(&mut self, depth: usize, name: &str, value: Value) -> bool {
        match self.scopes.len().checked_sub(depth + 1) {
            Some(i) => match self.scopes[i].get_mut(name) {
                Some(slot) => {
                    *slot = value;
                    true
                }
                None => false,
            },
            None => false,
        }
    }

    /// Read a global. Globals stay dynamic: a name may be used before its declaration
    /// runs, which is what makes mutual recursion work, so this is still a lookup.
    pub fn get_global(&self, name: &str) -> Option<&Value> {
        self.scopes[0].get(name)
    }

    /// Assign to a global. `false` if it was never declared: assignment does not create
    /// a variable in Kobo, `var` does.
    pub fn assign_global(&mut self, name: &str, value: Value) -> bool {
        match self.scopes[0].get_mut(name) {
            Some(slot) => {
                *slot = value;
                true
            }
            None => false,
        }
    }

    /// Innermost outwards, so the nearest declaration wins: the current frame's scopes
    /// from the top down, then the globals. Nothing between; that gap is section 4.3.
    ///
    /// Unused once `resolver.rs` is written, because resolution reaches for `get_at`
    /// instead. It is here because `evaluate` written before the resolver works has
    /// nothing else to call.
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&Value> {
        let base = self.frame_base();
        for scope in self.scopes[base..].iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        if base > 0 {
            return self.scopes[0].get(name);
        }
        None
    }

    /// Assign to an existing binding. Returns `false` if the name was never declared:
    /// assignment does not create a variable in Kobo, `var` does.
    #[allow(dead_code)]
    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        let base = self.frame_base();
        for scope in self.scopes[base..].iter_mut().rev() {
            if let Some(slot) = scope.get_mut(name) {
                *slot = value;
                return true;
            }
        }
        if base > 0 {
            if let Some(slot) = self.scopes[0].get_mut(name) {
                *slot = value;
                return true;
            }
        }
        false
    }

    fn innermost(&mut self) -> &mut HashMap<String, Value> {
        self.scopes
            .last_mut()
            .expect("the global scope is never popped")
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::new()
    }
}
