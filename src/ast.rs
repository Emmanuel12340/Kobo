// GIVEN, do not edit. This file is part of the compiler you are handed, not the
// compiler you write. Read it anyway: what you write has to fit it.

use crate::token::Token;
use crate::value::Value;

/// A parsed source file: the top-level statements, and every function in the file
/// hoisted into one flat table.
///
/// Functions are lifted out of the tree and referred to by index. That costs a little
/// indirection here and buys a great deal later: a `Value` can name a function with a
/// plain `usize` instead of owning a piece of the tree, so no reference counting and no
/// lifetimes reach `value.rs`. The bytecode compiler wants exactly the same table, one
/// chunk per entry, which is not a coincidence; numbering the functions is what makes
/// a call a jump.
#[derive(Debug, Default)]
pub struct Program {
    pub stmts: Vec<Stmt>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    /// Where `fun` was written. The bytecode compiler needs a line to attribute the
    /// implicit `return nil` at the end of the body to.
    pub line: usize,
}

/// Where a name was resolved to, filled in by `resolver.rs` before either engine runs.
///
/// `Global` is both the answer for a real global and the parser's placeholder, and those
/// are the same thing on purpose: an unresolved tree behaves exactly as this compiler did
/// before the pass existed, because a name that resolves to no local *is* a global.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Resolved {
    #[default]
    Global,
    Local {
        /// Scopes out from the innermost. The interpreter indexes with it.
        depth: usize,
        /// Position in the call frame's run of stack values. The VM indexes with it.
        slot: usize,
    },
}

/// `Box` and nothing else. The tree owns its children outright, so no node ever borrows
/// from another and no lifetime ever appears in this file. That is a deliberate choice;
/// see docs/language-spec.md section 4.3 for what it costs us later.
#[derive(Debug)]
pub enum Expr {
    /// The line matters even though a literal can never fail: the bytecode compiler
    /// attributes every instruction to a source line, and `CONSTANT` is an instruction.
    Literal {
        value: Value,
        line: usize,
    },
    Grouping(Box<Expr>),
    /// The token, not just the name: a runtime error needs the line it was written on.
    Variable {
        name: Token,
        at: Resolved,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
        at: Resolved,
    },
    Call {
        callee: Box<Expr>,
        /// The closing `)`. Errors about the call (wrong arity, calling a number)
        /// belong to the call site, not to whatever expression produced the callee.
        paren: Token,
        args: Vec<Expr>,
    },
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: Token,
        /// Absent means `nil` (spec section 3.1), but the tree records which was written.
        initialiser: Option<Expr>,
        /// The stack slot the resolver gave it, or `None` for a global.
        slot: Option<usize>,
    },
    Block {
        body: Vec<Stmt>,
        /// How many locals the block declares, so the bytecode compiler knows how many
        /// `Pop`s leaving it costs without tracking scopes a second time.
        locals: usize,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        /// The dangling `else` binds here, to the nearest unmatched `if`, because
        /// recursive descent consumes it as soon as it sees it (spec section 3.1).
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    /// An index into `Program::functions`. Executing it binds the name in the current
    /// scope; the body lives in the table.
    Function {
        index: usize,
        /// The declaration's own name, so the resolver can bind it without reaching into
        /// `Program::functions` while that table is being walked.
        name: String,
        slot: Option<usize>,
    },
    Return(Option<Expr>),
}
