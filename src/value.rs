// GIVEN, do not edit. This file is part of the compiler you are handed, not the
// compiler you write. Read it anyway: what you write has to fit it.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    Str(String),
    /// A function is named by its index in `Program::functions`. It carries its own name
    /// as well, only so that `print f;` can say `<fn f>` without reaching back into the
    /// program; a `Value` that had to borrow from the tree would drag a lifetime through
    /// every scope, every assignment and every return in the compiler.
    Function {
        index: usize,
        name: String,
    },
}

impl Value {
    /// `false` and `nil` are false; everything else, `0` and `""` included, is true.
    pub fn is_truthy(&self) -> bool {
        !matches!(self, Value::Nil | Value::Bool(false))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            // Rust's f64 Display already prints the shortest round-tripping decimal
            // and drops a trailing `.0`, which is exactly what the spec asks for.
            Value::Number(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Function { name, .. } => write!(f, "<fn {}>", name),
        }
    }
}
