// GIVEN, do not edit. This file is part of the compiler you are handed, not the
// compiler you write. Read it anyway: what you write has to fit it.

use crate::value::Value;

/// One instruction of the Kobo virtual machine.
///
/// A production VM packs these into bytes; that is what "bytecode" means, and it is why
/// a real `OP_CONSTANT` is followed by a one-byte operand rather than carrying a `usize`.
/// Keeping the operand inside the variant costs memory and buys legibility: the
/// disassembler, the compiler and the VM all read the same shape, and nothing in this
/// course turns on the encoding. Week 12's Module A is where the packing is discussed.
#[derive(Debug, Clone, Copy)]
pub enum Op {
    /// Push `constants[i]`.
    Constant(usize),
    Nil,
    True,
    False,
    /// Discard the top of the stack. Every expression statement ends with one.
    Pop,

    /// The operand is the index of a constant holding the variable's name. A global has
    /// to be findable by name at run time, and the name has to live somewhere the VM can
    /// reach, so it goes in the constant table with everything else.
    DefineGlobal(usize),
    GetGlobal(usize),
    SetGlobal(usize),
    /// A slot in the current call frame's window of the stack. Resolved at compile time,
    /// which is the whole reason locals are faster than globals.
    GetLocal(usize),
    SetLocal(usize),

    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Negate,
    Print,

    /// Absolute index into `code`, not a relative offset. A real VM uses an offset so the
    /// operand fits in two bytes; an absolute target is readable in a disassembly.
    Jump(usize),
    /// Jumps if the top of the stack is falsey, and **leaves it there**. `and` and `or`
    /// evaluate to one of their operands (spec section 4.1), so the value has to survive the
    /// test; `if` and `while` emit an explicit `Pop` afterwards instead.
    JumpIfFalse(usize),
    /// The operand is the argument count. The callee sits under the arguments.
    Call(usize),
    Return,
}

/// A compiled body: the script, or one function.
pub struct Chunk {
    pub name: String,
    pub arity: usize,
    pub code: Vec<Op>,
    /// `lines[i]` is the source line of `code[i]`. This is the only reason a runtime error
    /// from the VM can say `[line N]` at all, and it is why the two engines can agree.
    pub lines: Vec<usize>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new(name: String, arity: usize) -> Self {
        Chunk {
            name,
            arity,
            code: Vec::new(),
            lines: Vec::new(),
            constants: Vec::new(),
        }
    }

    /// Append an instruction; returns its offset, which is what jump patching needs.
    pub fn write(&mut self, op: Op, line: usize) -> usize {
        self.code.push(op);
        self.lines.push(line);
        self.code.len() - 1
    }

    /// Add a constant, reusing an equal one. Reuse is not required for correctness; it
    /// keeps the table small enough to read in a disassembly.
    pub fn constant(&mut self, value: Value) -> usize {
        if let Some(i) = self.constants.iter().position(|c| *c == value) {
            return i;
        }
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn last_line(&self) -> usize {
        *self.lines.last().unwrap_or(&1)
    }
}

/// Everything a program compiles to. `functions[i]` is the chunk for
/// `Program::functions[i]`, so a `Value::Function` means the same index in both engines
/// and the tree-walker and the VM can be swapped without touching a value.
pub struct Bytecode {
    pub script: Chunk,
    pub functions: Vec<Chunk>,
}
