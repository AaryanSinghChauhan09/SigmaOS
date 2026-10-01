// SigmaOS Compiler — Code Generator
// Emits x86_64 assembly (AT&T syntax) from the typed AST.
// Targets System V AMD64 ABI for self-hosting on SigmaOS.

extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::parser::{BinaryOp, Block, Expr, FunctionDef, Item, Program, Stmt, UnaryOp};

// ─── Assembly Builder ───────────────────────────────────────────────────────

/// Accumulated assembly output.
#[derive(Debug, Clone)]
pub struct AsmOutput {
    pub text_section: String,
    pub data_section: String,
    pub bss_section: String,
}

impl AsmOutput {
    pub fn new() -> Self {
        Self {
            text_section: String::new(),
            data_section: String::new(),
            bss_section: String::new(),
        }
    }

    /// Render the final assembly file.
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("# SigmaOS Compiler Output — x86_64 AT&T Syntax\n");
        out.push_str("# Target: System V AMD64 ABI\n\n");

        if !self.data_section.is_empty() {
            out.push_str(".section .data\n");
            out.push_str(&self.data_section);
            out.push('\n');
        }
        if !self.bss_section.is_empty() {
            out.push_str(".section .bss\n");
            out.push_str(&self.bss_section);
            out.push('\n');
        }
        out.push_str(".section .text\n");
        out.push_str(&self.text_section);
        out
    }
}

// ─── Code Generator ────────────────────────────────────────────────────────

/// Register allocator state (simple stack-based for now).
struct StackFrame {
    /// Map from variable name to stack offset (negative from RBP).
    locals: Vec<(String, i64)>,
    /// Current stack offset for next local.
    offset: i64,
    /// Label counter for unique labels within this function.
    label_counter: u64,
}

impl StackFrame {
    fn new() -> Self {
        Self {
            locals: Vec::new(),
            offset: 0,
            label_counter: 0,
        }
    }

    fn alloc_local(&mut self, name: &str) -> i64 {
        self.offset -= 8;
        self.locals.push((String::from(name), self.offset));
        self.offset
    }

    fn lookup_local(&self, name: &str) -> Option<i64> {
        for (n, off) in self.locals.iter().rev() {
            if n == name {
                return Some(*off);
            }
        }
        None
    }

    fn next_label(&mut self, prefix: &str) -> String {
        self.label_counter += 1;
        format!(".L{}_{}", prefix, self.label_counter)
    }
}

/// System V AMD64 ABI argument registers.
const ARG_REGS: [&str; 6] = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];

/// The x86_64 code generator.
pub struct CodeGenerator {
    output: AsmOutput,
    string_counter: u64,
    global_label_counter: u64,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            output: AsmOutput::new(),
            string_counter: 0,
            global_label_counter: 0,
        }
    }

    /// Generate assembly for an entire program.
    pub fn generate(&mut self, program: &Program) -> AsmOutput {
        for item in &program.items {
            self.gen_item(item);
        }
        self.output.clone()
    }

    fn gen_item(&mut self, item: &Item) {
        match item {
            Item::Function(f) => self.gen_function(f),
            _ => {
                // Struct/Enum/Trait definitions don't emit code directly
            }
        }
    }

    fn gen_function(&mut self, f: &FunctionDef) {
        let mut frame = StackFrame::new();

        // Function label
        if f.is_pub || f.name == "main" {
            self.emit_text(&format!(".globl {}", f.name));
        }
        self.emit_text(&format!("{}:", f.name));

        // Prologue
        self.emit_text("    pushq %rbp");
        self.emit_text("    movq %rsp, %rbp");

        // Reserve stack space (will be patched later if needed)
        let stack_reserve_label = format!(".Lstack_reserve_{}", f.name);
        self.emit_text(&format!("    subq ${}, %rsp", stack_reserve_label));

        // Move arguments from registers to stack
        for (i, (name, _)) in f.params.iter().enumerate() {
            let offset = frame.alloc_local(name);
            if i < ARG_REGS.len() {
                self.emit_text(&format!("    movq {}, {}(%rbp)", ARG_REGS[i], offset));
            } else {
                // Arguments beyond 6 are on the stack, above RBP
                let stack_arg_offset = 16 + (i - 6) as i64 * 8;
                self.emit_text(&format!("    movq {}(%rbp), %rax", stack_arg_offset));
                self.emit_text(&format!("    movq %rax, {}(%rbp)", offset));
            }
        }

        // Generate body
        self.gen_block(&f.body, &mut frame);

        // Default return (if body doesn't end with explicit return)
        self.emit_text(&format!(".Lreturn_{}:", f.name));
        self.emit_text("    movq %rbp, %rsp");
        self.emit_text("    popq %rbp");
        self.emit_text("    ret");

        // Patch stack reserve — align to 16 bytes
        let total_stack = ((-frame.offset + 15) / 16) * 16;
        // Replace the label reference with the actual value
        self.output.text_section = self.output.text_section.replace(
            &format!("${}", stack_reserve_label),
            &format!("${}", total_stack),
        );

        self.emit_text(""); // blank line after function
    }

    fn gen_block(&mut self, block: &Block, frame: &mut StackFrame) {
        for stmt in &block.stmts {
            self.gen_stmt(stmt, frame);
        }
    }

    fn gen_stmt(&mut self, stmt: &Stmt, frame: &mut StackFrame) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let offset = frame.alloc_local(name);
                if let Some(val) = value {
                    self.gen_expr(val, frame);
                    self.emit_text(&format!("    movq %rax, {}(%rbp)", offset));
                } else {
                    // Zero-initialize
                    self.emit_text(&format!("    movq $0, {}(%rbp)", offset));
                }
            }
            Stmt::Return(expr, _) => {
                if let Some(e) = expr {
                    self.gen_expr(e, frame);
                } else {
                    self.emit_text("    xorq %rax, %rax");
                }
                // Jump to function epilogue — we don't know the function name here,
                // so we use the return mechanism directly
                self.emit_text("    movq %rbp, %rsp");
                self.emit_text("    popq %rbp");
                self.emit_text("    ret");
            }
            Stmt::Expr(expr) => {
                self.gen_expr(expr, frame);
            }
            Stmt::While { condition, body, .. } => {
                let loop_label = frame.next_label("while");
                let end_label = frame.next_label("while_end");
                self.emit_text(&format!("{}:", loop_label));
                self.gen_expr(condition, frame);
                self.emit_text("    testq %rax, %rax");
                self.emit_text(&format!("    jz {}", end_label));
                self.gen_block(body, frame);
                self.emit_text(&format!("    jmp {}", loop_label));
                self.emit_text(&format!("{}:", end_label));
            }
            Stmt::For { variable, iterable, body, .. } => {
                // Simple range-like for: treat iterable result as count
                let offset = frame.alloc_local(variable);
                self.emit_text(&format!("    movq $0, {}(%rbp)", offset));
                let loop_label = frame.next_label("for");
                let end_label = frame.next_label("for_end");
                self.gen_expr(iterable, frame);
                self.emit_text("    movq %rax, %rcx"); // upper bound in rcx
                self.emit_text(&format!("{}:", loop_label));
                self.emit_text(&format!("    cmpq %rcx, {}(%rbp)", offset));
                self.emit_text(&format!("    jge {}", end_label));
                self.gen_block(body, frame);
                self.emit_text(&format!("    incq {}(%rbp)", offset));
                self.emit_text(&format!("    jmp {}", loop_label));
                self.emit_text(&format!("{}:", end_label));
            }
            Stmt::Loop { body, .. } => {
                let loop_label = frame.next_label("loop");
                self.emit_text(&format!("{}:", loop_label));
                self.gen_block(body, frame);
                self.emit_text(&format!("    jmp {}", loop_label));
            }
            Stmt::Break(_) => {
                // In a real implementation, this would jump to the enclosing loop's end label
                self.emit_text("    # break (requires loop context)");
            }
            Stmt::Continue(_) => {
                self.emit_text("    # continue (requires loop context)");
            }
        }
    }

    fn gen_expr(&mut self, expr: &Expr, frame: &mut StackFrame) {
        match expr {
            Expr::IntLiteral(n, _) => {
                if *n <= i32::MAX as u64 {
                    self.emit_text(&format!("    movq ${}, %rax", n));
                } else {
                    self.emit_text(&format!("    movabsq ${}, %rax", n));
                }
            }
            Expr::FloatLiteral(f, _) => {
                let bits = f.to_bits();
                self.emit_text(&format!("    movabsq ${}, %rax", bits));
                // For proper float handling, would use xmm registers
            }
            Expr::BoolLiteral(b, _) => {
                self.emit_text(&format!("    movq ${}, %rax", if *b { 1 } else { 0 }));
            }
            Expr::CharLiteral(c, _) => {
                self.emit_text(&format!("    movq ${}, %rax", *c as u64));
            }
            Expr::StringLiteral(s, _) => {
                let label = self.alloc_string(s);
                self.emit_text(&format!("    leaq {}(%rip), %rax", label));
            }
            Expr::Ident(name, _) => {
                if let Some(offset) = frame.lookup_local(name) {
                    self.emit_text(&format!("    movq {}(%rbp), %rax", offset));
                } else {
                    // Global or function reference
                    self.emit_text(&format!("    leaq {}(%rip), %rax", name));
                }
            }
            Expr::Binary { left, op, right, .. } => {
                // Evaluate right first, push, then evaluate left
                self.gen_expr(right, frame);
                self.emit_text("    pushq %rax");
                self.gen_expr(left, frame);
                self.emit_text("    popq %rcx");

                match op {
                    BinaryOp::Add => {
                        self.emit_text("    addq %rcx, %rax");
                    }
                    BinaryOp::Sub => {
                        self.emit_text("    subq %rcx, %rax");
                    }
                    BinaryOp::Mul => {
                        self.emit_text("    imulq %rcx, %rax");
                    }
                    BinaryOp::Div => {
                        self.emit_text("    cqto");
                        self.emit_text("    idivq %rcx");
                    }
                    BinaryOp::Mod => {
                        self.emit_text("    cqto");
                        self.emit_text("    idivq %rcx");
                        self.emit_text("    movq %rdx, %rax");
                    }
                    BinaryOp::Eq => {
                        self.emit_text("    cmpq %rcx, %rax");
                        self.emit_text("    sete %al");
                        self.emit_text("    movzbq %al, %rax");
                    }
                    BinaryOp::NotEq => {
                        self.emit_text("    cmpq %rcx, %rax");
                        self.emit_text("    setne %al");
                        self.emit_text("    movzbq %al, %rax");
                    }
                    BinaryOp::Lt => {
                        self.emit_text("    cmpq %rcx, %rax");
                        self.emit_text("    setl %al");
                        self.emit_text("    movzbq %al, %rax");
                    }
                    BinaryOp::LtEq => {
                        self.emit_text("    cmpq %rcx, %rax");
                        self.emit_text("    setle %al");
                        self.emit_text("    movzbq %al, %rax");
                    }
                    BinaryOp::Gt => {
                        self.emit_text("    cmpq %rcx, %rax");
                        self.emit_text("    setg %al");
                        self.emit_text("    movzbq %al, %rax");
                    }
                    BinaryOp::GtEq => {
                        self.emit_text("    cmpq %rcx, %rax");
                        self.emit_text("    setge %al");
                        self.emit_text("    movzbq %al, %rax");
                    }
                    BinaryOp::And => {
                        self.emit_text("    testq %rax, %rax");
                        let false_label = frame.next_label("and_false");
                        let end_label = frame.next_label("and_end");
                        self.emit_text(&format!("    jz {}", false_label));
                        self.emit_text("    testq %rcx, %rcx");
                        self.emit_text(&format!("    jz {}", false_label));
                        self.emit_text("    movq $1, %rax");
                        self.emit_text(&format!("    jmp {}", end_label));
                        self.emit_text(&format!("{}:", false_label));
                        self.emit_text("    xorq %rax, %rax");
                        self.emit_text(&format!("{}:", end_label));
                    }
                    BinaryOp::Or => {
                        self.emit_text("    orq %rcx, %rax");
                        self.emit_text("    testq %rax, %rax");
                        self.emit_text("    setne %al");
                        self.emit_text("    movzbq %al, %rax");
                    }
                    BinaryOp::BitAnd => {
                        self.emit_text("    andq %rcx, %rax");
                    }
                    BinaryOp::BitOr => {
                        self.emit_text("    orq %rcx, %rax");
                    }
                    BinaryOp::BitXor => {
                        self.emit_text("    xorq %rcx, %rax");
                    }
                    BinaryOp::Shl => {
                        // rcx already has shift amount, but shl uses %cl
                        self.emit_text("    shlq %cl, %rax");
                    }
                    BinaryOp::Shr => {
                        self.emit_text("    sarq %cl, %rax");
                    }
                }
            }
            Expr::Unary { op, operand, .. } => {
                self.gen_expr(operand, frame);
                match op {
                    UnaryOp::Neg => {
                        self.emit_text("    negq %rax");
                    }
                    UnaryOp::Not => {
                        self.emit_text("    testq %rax, %rax");
                        self.emit_text("    sete %al");
                        self.emit_text("    movzbq %al, %rax");
                    }
                    UnaryOp::BitNot => {
                        self.emit_text("    notq %rax");
                    }
                    UnaryOp::Ref => {
                        // Address-of: rax should contain the address (identity for now)
                    }
                    UnaryOp::Deref => {
                        self.emit_text("    movq (%rax), %rax");
                    }
                }
            }
            Expr::Call { callee, args, .. } => {
                // Push args that go on the stack (>6 args)
                if args.len() > 6 {
                    for arg in args[6..].iter().rev() {
                        self.gen_expr(arg, frame);
                        self.emit_text("    pushq %rax");
                    }
                }
                // Evaluate register args and save them
                for (i, arg) in args.iter().enumerate().take(6) {
                    self.gen_expr(arg, frame);
                    if i < 6 {
                        self.emit_text(&format!("    movq %rax, {}", ARG_REGS[i]));
                    }
                }
                // Determine callee
                match callee.as_ref() {
                    Expr::Ident(name, _) => {
                        self.emit_text(&format!("    call {}", name));
                    }
                    Expr::Path(path, _) => {
                        let name = path.join("_");
                        self.emit_text(&format!("    call {}", name));
                    }
                    _ => {
                        self.gen_expr(callee, frame);
                        self.emit_text("    call *%rax");
                    }
                }
                // Clean up stack args
                if args.len() > 6 {
                    let cleanup = (args.len() - 6) * 8;
                    self.emit_text(&format!("    addq ${}, %rsp", cleanup));
                }
            }
            Expr::FieldAccess { object, field, .. } => {
                self.gen_expr(object, frame);
                // In a real implementation, this would use struct layout info
                // For now, emit a comment
                self.emit_text(&format!("    # field access: .{}", field));
            }
            Expr::Index { object, index, .. } => {
                self.gen_expr(index, frame);
                self.emit_text("    pushq %rax");
                self.gen_expr(object, frame);
                self.emit_text("    popq %rcx");
                self.emit_text("    movq (%rax,%rcx,8), %rax");
            }
            Expr::If { condition, then_block, else_block, .. } => {
                let else_label = frame.next_label("if_else");
                let end_label = frame.next_label("if_end");

                self.gen_expr(condition, frame);
                self.emit_text("    testq %rax, %rax");
                self.emit_text(&format!("    jz {}", else_label));

                self.gen_block(then_block, frame);
                self.emit_text(&format!("    jmp {}", end_label));

                self.emit_text(&format!("{}:", else_label));
                if let Some(eb) = else_block {
                    self.gen_block(eb, frame);
                }

                self.emit_text(&format!("{}:", end_label));
            }
            Expr::Assign { target, value, .. } => {
                self.gen_expr(value, frame);
                match target.as_ref() {
                    Expr::Ident(name, _) => {
                        if let Some(offset) = frame.lookup_local(name) {
                            self.emit_text(&format!("    movq %rax, {}(%rbp)", offset));
                        }
                    }
                    Expr::Index { object, index, .. } => {
                        self.emit_text("    pushq %rax"); // save value
                        self.gen_expr(index, frame);
                        self.emit_text("    pushq %rax"); // save index
                        self.gen_expr(object, frame);
                        self.emit_text("    popq %rcx"); // index
                        self.emit_text("    popq %rdx"); // value
                        self.emit_text("    movq %rdx, (%rax,%rcx,8)");
                    }
                    _ => {
                        self.emit_text("    # complex assignment target");
                    }
                }
            }
            Expr::Block(block) => {
                self.gen_block(block, frame);
            }
            Expr::Array(elems, _) => {
                // Allocate array on stack
                let count = elems.len();
                self.emit_text(&format!("    subq ${}, %rsp", count * 8));
                self.emit_text("    movq %rsp, %rax");
                self.emit_text("    pushq %rax"); // save base
                for (i, elem) in elems.iter().enumerate() {
                    self.gen_expr(elem, frame);
                    self.emit_text(&format!("    movq %rax, {}(%rsp)", (i + 1) * 8));
                }
                self.emit_text("    popq %rax"); // restore base
            }
            Expr::Cast { expr, .. } => {
                // Most casts are no-ops at the assembly level for same-size types
                self.gen_expr(expr, frame);
            }
            Expr::Match { scrutinee, arms, .. } => {
                self.gen_expr(scrutinee, frame);
                let end_label = frame.next_label("match_end");
                for arm in arms {
                    let next_label = frame.next_label("match_arm");
                    // Simple integer/bool pattern matching
                    match &arm.pattern {
                        super::parser::Pattern::IntLiteral(n) => {
                            self.emit_text(&format!("    cmpq ${}, %rax", n));
                            self.emit_text(&format!("    jne {}", next_label));
                        }
                        super::parser::Pattern::BoolLiteral(b) => {
                            self.emit_text(&format!(
                                "    cmpq ${}, %rax",
                                if *b { 1 } else { 0 }
                            ));
                            self.emit_text(&format!("    jne {}", next_label));
                        }
                        super::parser::Pattern::Wildcard | super::parser::Pattern::Ident(_) => {
                            // Always matches
                        }
                        _ => {
                            self.emit_text("    # complex pattern match");
                        }
                    }
                    self.gen_expr(&arm.body, frame);
                    self.emit_text(&format!("    jmp {}", end_label));
                    self.emit_text(&format!("{}:", next_label));
                }
                self.emit_text(&format!("{}:", end_label));
            }
            Expr::Path(path, _) => {
                let mangled = path.join("_");
                self.emit_text(&format!("    leaq {}(%rip), %rax", mangled));
            }
            Expr::StructLit { .. } => {
                self.emit_text("    # struct literal (requires allocator)");
            }
        }
    }

    // ── Helpers ──────────────────────────────────────────────────────

    fn emit_text(&mut self, line: &str) {
        self.output.text_section.push_str(line);
        self.output.text_section.push('\n');
    }

    fn alloc_string(&mut self, s: &str) -> String {
        let label = format!(".Lstr_{}", self.string_counter);
        self.string_counter += 1;
        self.output
            .data_section
            .push_str(&format!("{}:\n    .asciz \"{}\"\n", label, Self::escape_asm(s)));
        label
    }

    fn escape_asm(s: &str) -> String {
        let mut out = String::new();
        for ch in s.chars() {
            match ch {
                '\n' => out.push_str("\\n"),
                '\t' => out.push_str("\\t"),
                '\r' => out.push_str("\\r"),
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\0' => out.push_str("\\0"),
                c => out.push(c),
            }
        }
        out
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lexer::Lexer;
    use crate::compiler::parser::Parser;

    fn compile(source: &str) -> String {
        let mut lexer = Lexer::new(source, "test.sg");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        let mut codegen = CodeGenerator::new();
        let output = codegen.generate(&program);
        output.render()
    }

    #[test]
    fn test_simple_function() {
        let asm = compile("fn main() { return 42 }");
        assert!(asm.contains("main:"));
        assert!(asm.contains("$42"));
        assert!(asm.contains("ret"));
    }

    #[test]
    fn test_binary_add() {
        let asm = compile("fn add(a: i64, b: i64) -> i64 { return a + b }");
        assert!(asm.contains("addq"));
    }

    #[test]
    fn test_string_literal() {
        let asm = compile(r#"fn greet() { "hello" }"#);
        assert!(asm.contains(".asciz \"hello\""));
    }

    #[test]
    fn test_conditional() {
        let asm = compile("fn test(x: i64) -> i64 { if x { return 1 } else { return 0 } }");
        assert!(asm.contains("testq"));
        assert!(asm.contains("jz"));
    }
}
