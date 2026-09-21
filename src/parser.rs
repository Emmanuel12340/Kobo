use crate::ast::{Expr, Function, Program, Resolved, Stmt};
use crate::token::{one_line, Token, TokenType};
use crate::value::Value;

/// Recursive descent. Every rule in the grammar is one function, and the call order of
/// those functions *is* the precedence table.
pub fn parse(tokens: Vec<Token>) -> (Program, Vec<String>) {
    let mut p = Parser {
        tokens,
        current: 0,
        depth: 0,
        program: Program::default(),
        errors: Vec::new(),
    };
    while !p.at_end() {
        match p.declaration() {
            Ok(s) => p.program.stmts.push(s),
            Err(()) => p.synchronise(),
        }
    }
    (p.program, p.errors)
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    /// How many grammar rules are currently on the Rust call stack. Recursive descent
    /// borrows the host language's stack, so a deeply nested source file would overflow
    /// it and abort the process. A table-driven parser has no such limit; this is the
    /// price of the technique, and the reason for `MAX_NESTING`.
    depth: usize,
    /// Built as the parse runs: `funDecl` appends to `program.functions` and hands back
    /// the index. Only `stmts` is filled in at the end, by `parse`.
    program: Program,
    errors: Vec<String>,
}

/// Deep enough that no honest program reaches it, shallow enough that the Rust stack
/// does not: measured overflow is past a thousand levels on this toolchain.
const MAX_NESTING: usize = 250;

type Parsed<T> = Result<T, ()>;

impl Parser {
    // --- declarations and statements ------------------------------------------------

    fn declaration(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: a declaration is a fun, a var, or a statement. This is also
        //            where an error is caught and synchronise() runs.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        self.statement()
    }

    fn fun_declaration(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: a function declaration, its parameter list, and its body.
        //            Functions are hoisted into Program::functions.
        todo!("fun_declaration")
    }

    fn var_declaration(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: 'var' name ( '=' expression )? ';'
        todo!("var_declaration")
    }

    fn statement(&mut self) -> Parsed<Stmt> {
        self.depth += 1;
        let result = self.statement_inner();
        self.depth -= 1;
        result
    }

    fn statement_inner(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: pick the statement by its leading token.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        // Named rather than misreported, so an `if` says what it is instead of failing
        // as a broken expression.
        if self.check(TokenType::LBrace) || self.check(TokenType::If)
            || self.check(TokenType::While) || self.check(TokenType::Return)
        {
            self.error("This statement form is not written yet.");
            return Err(());
        }
        if self.matches(&[TokenType::Print]) {
            let value = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
            Ok(Stmt::Print(value))
        } else {
            let expr = self.expression()?;
            self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
            Ok(Stmt::Expression(expr))
        }
    }

    fn return_statement(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: 'return' expression? ';'
        todo!("return_statement")
    }

    fn if_statement(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1. The dangling else binds to the nearest unmatched if , the
        //            recursion decides that, not the grammar.
        todo!("if_statement")
    }

    fn while_statement(&mut self) -> Parsed<Stmt> {
        // TODO(you): spec 3.1: 'while' '(' expression ')' statement
        todo!("while_statement")
    }

    /// The `{` is already consumed. Declarations, not statements: a block is where a
    /// `var` most often appears, and the whole point of one is that the name dies at `}`.
    fn block(&mut self) -> Parsed<Vec<Stmt>> {
        // TODO(you): spec 3.1: '{' declaration* '}'
        todo!("block")
    }

    // --- expressions, lowest precedence first --------------------------------------

    fn expression(&mut self) -> Parsed<Expr> {
        self.depth += 1;
        let result = if self.depth > MAX_NESTING {
            self.error("Too deeply nested.");
            Err(())
        } else {
            self.assignment()
        };
        self.depth -= 1;
        result
    }

    /// Assignment is right-associative and its left side must be a bare name, but that is
    /// not knowable until the `=` is reached: `a` and `a.b` and `f()` all start the same
    /// way. So the left side is parsed as an ordinary expression and *then* checked, which
    /// is the standard answer to a rule that is not LL(1).
    fn assignment(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2. The left side parses as an ordinary expression and is checked
        //            afterwards, because the rule is not LL(1).
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        self.or()
    }

    fn or(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        self.and()
    }

    fn and(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        self.equality()
    }

    fn equality(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        self.comparison()
    }

    fn comparison(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        self.term()
    }

    fn term(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        todo!("term")
    }

    fn factor(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2, one rung of the precedence ladder.
        todo!("factor")
    }

    fn unary(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2: unary operators are right-associative.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        self.call()
    }

    /// A loop, not recursion: `f(1)(2)` is one callee called twice, and the grammar says
    /// `call → primary ( "(" arguments? ")" )*` for exactly that reason.
    fn call(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2: call -> primary ( '(' arguments? ')' )*. That is a loop, not
        //            recursion; f(1)(2) is one callee called twice.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        self.primary()
    }

    fn primary(&mut self) -> Parsed<Expr> {
        // TODO(you): spec 3.2: literals, identifiers and grouping.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        if self.matches(&[TokenType::Number]) {
            let n = self.previous().lexeme.parse::<f64>().unwrap();
            return Ok(self.literal(Value::Number(n)));
        }
        self.error("Expect expression.");
        Err(())
    }

    // --- error recovery ---------------------------------------------------------------

    /// Discard tokens until the parser is plausibly at the start of a statement again:
    /// just past a `;`, or sitting on a keyword that can only begin a declaration or
    /// statement. This is why one missing semicolon costs one error and not forty
    /// (spec section 5.2).
    ///
    /// It always steps over at least one token before it looks, which is what guarantees
    /// the parse makes progress; a recovery that could stand still would hang the
    /// compiler on the file that triggered it.
    fn synchronise(&mut self) {
        // TODO(you): spec 5.2: after an error, discard tokens until just past a ';' or on a
        //            statement keyword, so one mistake costs one message.
        // TEMPORARY: enough of this rule to let the ones below it run. Replace it in its
        //            own week; the marking tests for it fail until you do.
        self.depth = 0;
        while !self.at_end() {
            self.current += 1;
            if self.previous().kind == TokenType::Semicolon {
                return;
            }
        }
    }

    /// Wraps a value as a literal node carrying the line of the token just consumed.
    fn literal(&self, value: Value) -> Expr {
        Expr::Literal {
            value,
            line: self.previous().line,
        }
    }

    // --- primitives ---------------------------------------------------------------

    fn matches(&mut self, kinds: &[TokenType]) -> bool {
        for k in kinds {
            if self.check(*k) {
                self.current += 1;
                return true;
            }
        }
        false
    }

    fn check(&self, kind: TokenType) -> bool {
        !self.at_end() && self.peek().kind == kind
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, kind: TokenType, message: &str) -> Parsed<Token> {
        if self.check(kind) {
            self.current += 1;
            return Ok(self.previous());
        }
        self.error(message);
        Err(())
    }

    fn error(&mut self, message: &str) {
        let t = self.peek().clone();
        self.error_at(&t, message);
    }

    fn error_at(&mut self, t: &Token, message: &str) {
        let at = if t.kind == TokenType::Eof {
            "end".to_string()
        } else {
            format!("'{}'", one_line(&t.lexeme))
        };
        self.errors
            .push(format!("[line {}] Error at {}: {}", t.line, at, message));
    }
}
