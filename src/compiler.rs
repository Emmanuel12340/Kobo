use crate::ast::{Expr, Program, Resolved, Stmt};
use crate::chunk::{Bytecode, Chunk, Op};
use crate::token::TokenType;
use crate::value::Value;

/// AST → bytecode. One pass, no intermediate graph: the tree is already the structure a
/// code generator wants, so walking it and emitting as we go is enough for a language
/// this size.
///
/// **Name resolution is not done here any more.** `resolver.rs` runs first and writes
/// every local's stack slot into the tree, so this file emits `GET_LOCAL 3` by reading a
/// number rather than by keeping its own scope stack and searching it. That used to be
/// two implementations of one rule, the resolver's and this one's, which had to agree
/// and had nothing checking that they did.
pub fn compile(program: &Program) -> Bytecode {
    let functions = program
        .functions
        .iter()
        .map(|f| {
            // Parameters are the frame's first locals, in order, so argument `i` is
            // already in slot `i` when the body starts; the resolver gave them those
            // slots and this compiler never has to know it.
            let mut c = Compiler::new(Chunk::new(f.name.clone(), f.params.len()), program);
            c.in_function = true;
            c.statements(&f.body);
            c.finish(f.line)
        })
        .collect();

    let mut script = Compiler::new(Chunk::new("script".to_string(), 0), program);
    script.statements(&program.stmts);
    let line = script.chunk.last_line();
    Bytecode {
        script: script.finish(line),
        functions,
    }
}

struct Compiler<'a> {
    chunk: Chunk,
    program: &'a Program,
    /// A function body's declarations are locals even at its outermost level; the
    /// script's are globals. That is the only thing this file still needs to know about
    /// scope, and the resolver decides everything else.
    in_function: bool,
}

impl<'a> Compiler<'a> {
    fn new(chunk: Chunk, program: &'a Program) -> Self {
        Compiler {
            chunk,
            program,
            in_function: false,
        }
    }

    /// Every chunk ends by returning `nil`, so falling off the end of a function body
    /// returns `nil` (spec section 4.3) without the VM needing a special case.
    fn finish(mut self, line: usize) -> Chunk {
        self.chunk.write(Op::Nil, line);
        self.chunk.write(Op::Return, line);
        self.chunk
    }

    fn statements(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.statement(stmt);
        }
    }

    // --- statements -------------------------------------------------------------------

    fn statement(&mut self, stmt: &Stmt) {
        // TODO(you): emit code for one statement.
        todo!("statement")
    }

    // --- expressions ------------------------------------------------------------------

    /// Emits the code for `expr` and returns the line the caller should attribute its own
    /// follow-up instruction to.
    fn expression(&mut self, expr: &Expr) -> usize {
        // TODO(you): emit code for one expression, and return the line it came from.
        todo!("expression")
    }

    // --- names ------------------------------------------------------------------------

    /// At the top level a name is global and stays a name. Inside any scope it is a local,
    /// and the value already sitting on the stack *is* the variable; declaring it is
    /// nothing more than agreeing which slot it occupies.
    /// A global is written into the globals map; a local is already on the stack in the
    /// slot the resolver gave it, so declaring one emits nothing. `slot` is the
    /// resolver's answer and `in_function` is the one case it cannot express on its own:
    /// the script's outermost declarations are globals and a function body's are not.
    fn declare(&mut self, name: &str, slot: Option<usize>, line: usize) {
        // TODO(you): a global is written into the globals map; a local is already on the stack
        //            in the slot resolver.rs gave it, so it emits nothing.
        todo!("declare")
    }

    fn name_constant(&mut self, name: &str) -> usize {
        self.chunk.constant(Value::Str(name.to_string()))
    }

    /// Leaving a scope pops its locals off the stack, one `Pop` each. How many is the
    /// resolver's count, carried on the `Block`; this file no longer tracks scopes to
    /// work it out a second time. The interpreter dropped a whole `HashMap` here; the VM
    /// adjusts a stack pointer.
    fn end_scope(&mut self, locals: usize) {
        // TODO(you): leave a scope: one Pop per local that dies here, and the resolver already
        //            counted them onto the Block.
        todo!("end_scope")
    }

    fn patch(&mut self, at: usize) {
        // TODO(you): fill in a forward jump, whose target was not known when it was emitted.
        todo!("patch")
    }
}
