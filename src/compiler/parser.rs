// SigmaOS Compiler — Parser
// Recursive-descent parser producing a typed AST from the token stream.

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use super::lexer::{Keyword, SourceSpan, Token, TokenKind};

// ─── AST Nodes ──────────────────────────────────────────────────────────────

/// Top-level program: a sequence of items.
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

/// Top-level items.
#[derive(Debug, Clone)]
pub enum Item {
    Function(FunctionDef),
    StructDef(StructDef),
    EnumDef(EnumDef),
    TraitDef(TraitDef),
    ImplBlock(ImplBlock),
    ConstDef(ConstDef),
    UseDecl(UseDecl),
    ModDecl(String),
    ExternBlock(Vec<FunctionSig>),
}

/// A function definition.
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub name: String,
    pub params: Vec<(String, TypeExpr)>,
    pub return_type: Option<TypeExpr>,
    pub body: Block,
    pub is_pub: bool,
    pub is_unsafe: bool,
    pub span: SourceSpan,
}

/// A function signature (for traits / extern).
#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub name: String,
    pub params: Vec<(String, TypeExpr)>,
    pub return_type: Option<TypeExpr>,
    pub span: SourceSpan,
}

/// A struct definition.
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<(String, TypeExpr)>,
    pub is_pub: bool,
    pub span: SourceSpan,
}

/// An enum definition.
#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub is_pub: bool,
    pub span: SourceSpan,
}

/// An enum variant.
#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub fields: Option<Vec<TypeExpr>>,
}

/// A trait definition.
#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: String,
    pub methods: Vec<FunctionSig>,
    pub is_pub: bool,
    pub span: SourceSpan,
}

/// An impl block.
#[derive(Debug, Clone)]
pub struct ImplBlock {
    pub target_type: String,
    pub trait_name: Option<String>,
    pub methods: Vec<FunctionDef>,
    pub span: SourceSpan,
}

/// A constant definition.
#[derive(Debug, Clone)]
pub struct ConstDef {
    pub name: String,
    pub ty: TypeExpr,
    pub value: Expr,
    pub span: SourceSpan,
}

/// A use declaration.
#[derive(Debug, Clone)]
pub struct UseDecl {
    pub path: Vec<String>,
    pub span: SourceSpan,
}

/// A block of statements.
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: SourceSpan,
}

/// Statement types.
#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        ty: Option<TypeExpr>,
        value: Option<Expr>,
        span: SourceSpan,
    },
    Expr(Expr),
    Return(Option<Expr>, SourceSpan),
    Break(SourceSpan),
    Continue(SourceSpan),
    While {
        condition: Expr,
        body: Block,
        span: SourceSpan,
    },
    For {
        variable: String,
        iterable: Expr,
        body: Block,
        span: SourceSpan,
    },
    Loop {
        body: Block,
        span: SourceSpan,
    },
}

/// Expression types.
#[derive(Debug, Clone)]
pub enum Expr {
    IntLiteral(u64, SourceSpan),
    FloatLiteral(f64, SourceSpan),
    StringLiteral(String, SourceSpan),
    CharLiteral(char, SourceSpan),
    BoolLiteral(bool, SourceSpan),
    Ident(String, SourceSpan),

    // Binary / Unary
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: SourceSpan,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
        span: SourceSpan,
    },

    // Access
    FieldAccess {
        object: Box<Expr>,
        field: String,
        span: SourceSpan,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        span: SourceSpan,
    },

    // Call
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: SourceSpan,
    },

    // Struct literal
    StructLit {
        name: String,
        fields: Vec<(String, Expr)>,
        span: SourceSpan,
    },

    // Control flow as expressions
    If {
        condition: Box<Expr>,
        then_block: Block,
        else_block: Option<Block>,
        span: SourceSpan,
    },
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
        span: SourceSpan,
    },
    Block(Block),

    // Assignment
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
        span: SourceSpan,
    },

    // Array literal
    Array(Vec<Expr>, SourceSpan),

    // Cast
    Cast {
        expr: Box<Expr>,
        target_type: TypeExpr,
        span: SourceSpan,
    },

    // Path expression (e.g., std::io::read)
    Path(Vec<String>, SourceSpan),
}

/// Match arm.
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
}

/// Pattern for match arms.
#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Ident(String),
    IntLiteral(u64),
    BoolLiteral(bool),
    StringLiteral(String),
    Tuple(Vec<Pattern>),
    Enum { name: String, fields: Vec<Pattern> },
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, NotEq, Lt, LtEq, Gt, GtEq,
    And, Or,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
    Ref,
    Deref,
}

/// Type expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpr {
    Named(String),
    Generic(String, Vec<TypeExpr>),
    Reference { mutable: bool, inner: Box<TypeExpr> },
    Pointer { mutable: bool, inner: Box<TypeExpr> },
    Array { element: Box<TypeExpr>, size: Option<u64> },
    Tuple(Vec<TypeExpr>),
    Function { params: Vec<TypeExpr>, ret: Box<TypeExpr> },
    Infer,
    Unit,
}

// ─── Parser ─────────────────────────────────────────────────────────────────

/// Parse error.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: SourceSpan,
}

/// Recursive-descent parser for the Sigma language.
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parse a complete program.
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();
        self.skip_newlines();
        while !self.is_eof() {
            items.push(self.parse_item()?);
            self.skip_newlines();
        }
        Ok(Program { items })
    }

    fn parse_item(&mut self) -> Result<Item, ParseError> {
        let is_pub = self.check_keyword(Keyword::Pub);
        if is_pub {
            self.advance();
        }

        match self.peek_kind() {
            Some(TokenKind::Keyword(Keyword::Fn)) => {
                Ok(Item::Function(self.parse_function(is_pub, false)?))
            }
            Some(TokenKind::Keyword(Keyword::Struct)) => {
                Ok(Item::StructDef(self.parse_struct(is_pub)?))
            }
            Some(TokenKind::Keyword(Keyword::Enum)) => {
                Ok(Item::EnumDef(self.parse_enum(is_pub)?))
            }
            Some(TokenKind::Keyword(Keyword::Trait)) => {
                Ok(Item::TraitDef(self.parse_trait(is_pub)?))
            }
            Some(TokenKind::Keyword(Keyword::Impl)) => {
                Ok(Item::ImplBlock(self.parse_impl()?))
            }
            Some(TokenKind::Keyword(Keyword::Const)) | Some(TokenKind::Keyword(Keyword::Static)) => {
                Ok(Item::ConstDef(self.parse_const()?))
            }
            Some(TokenKind::Keyword(Keyword::Use)) => {
                Ok(Item::UseDecl(self.parse_use()?))
            }
            Some(TokenKind::Keyword(Keyword::Mod)) => {
                self.advance();
                let name = self.expect_ident()?;
                self.skip_newlines();
                Ok(Item::ModDecl(name))
            }
            Some(TokenKind::Keyword(Keyword::Extern)) => {
                self.advance();
                self.expect(TokenKind::LBrace)?;
                let mut sigs = Vec::new();
                self.skip_newlines();
                while !self.check(TokenKind::RBrace) && !self.is_eof() {
                    sigs.push(self.parse_function_sig()?);
                    self.skip_newlines();
                }
                self.expect(TokenKind::RBrace)?;
                Ok(Item::ExternBlock(sigs))
            }
            _ => Err(self.error("expected item (fn, struct, enum, trait, impl, const, use, mod)")),
        }
    }

    fn parse_function(&mut self, is_pub: bool, is_unsafe: bool) -> Result<FunctionDef, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::Fn)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(TokenKind::RParen)?;

        let return_type = if self.check(TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(FunctionDef {
            name,
            params,
            return_type,
            body,
            is_pub,
            is_unsafe,
            span,
        })
    }

    fn parse_function_sig(&mut self) -> Result<FunctionSig, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::Fn)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LParen)?;
        let params = self.parse_param_list()?;
        self.expect(TokenKind::RParen)?;

        let return_type = if self.check(TokenKind::Arrow) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        Ok(FunctionSig { name, params, return_type, span })
    }

    fn parse_param_list(&mut self) -> Result<Vec<(String, TypeExpr)>, ParseError> {
        let mut params = Vec::new();
        self.skip_newlines();
        if self.check(TokenKind::RParen) {
            return Ok(params);
        }
        loop {
            self.skip_newlines();
            let name = self.expect_ident()?;
            self.expect(TokenKind::Colon)?;
            let ty = self.parse_type()?;
            params.push((name, ty));
            self.skip_newlines();
            if !self.check(TokenKind::Comma) {
                break;
            }
            self.advance();
        }
        Ok(params)
    }

    fn parse_struct(&mut self, is_pub: bool) -> Result<StructDef, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::Struct)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        self.skip_newlines();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            let fname = self.expect_ident()?;
            self.expect(TokenKind::Colon)?;
            let ty = self.parse_type()?;
            fields.push((fname, ty));
            self.skip_newlines();
            if self.check(TokenKind::Comma) {
                self.advance();
            }
            self.skip_newlines();
        }
        self.expect(TokenKind::RBrace)?;
        Ok(StructDef { name, fields, is_pub, span })
    }

    fn parse_enum(&mut self, is_pub: bool) -> Result<EnumDef, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::Enum)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LBrace)?;
        let mut variants = Vec::new();
        self.skip_newlines();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            let vname = self.expect_ident()?;
            let fields = if self.check(TokenKind::LParen) {
                self.advance();
                let mut types = Vec::new();
                loop {
                    types.push(self.parse_type()?);
                    if !self.check(TokenKind::Comma) {
                        break;
                    }
                    self.advance();
                }
                self.expect(TokenKind::RParen)?;
                Some(types)
            } else {
                None
            };
            variants.push(EnumVariant { name: vname, fields });
            self.skip_newlines();
            if self.check(TokenKind::Comma) {
                self.advance();
            }
            self.skip_newlines();
        }
        self.expect(TokenKind::RBrace)?;
        Ok(EnumDef { name, variants, is_pub, span })
    }

    fn parse_trait(&mut self, is_pub: bool) -> Result<TraitDef, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::Trait)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LBrace)?;
        let mut methods = Vec::new();
        self.skip_newlines();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            methods.push(self.parse_function_sig()?);
            self.skip_newlines();
        }
        self.expect(TokenKind::RBrace)?;
        Ok(TraitDef { name, methods, is_pub, span })
    }

    fn parse_impl(&mut self) -> Result<ImplBlock, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::Impl)?;
        let first_name = self.expect_ident()?;
        let (target, trait_name) = if self.check_keyword(Keyword::For) {
            self.advance();
            let target = self.expect_ident()?;
            (target, Some(first_name))
        } else {
            (first_name, None)
        };
        self.expect(TokenKind::LBrace)?;
        let mut methods = Vec::new();
        self.skip_newlines();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            let is_pub = self.check_keyword(Keyword::Pub);
            if is_pub {
                self.advance();
            }
            methods.push(self.parse_function(is_pub, false)?);
            self.skip_newlines();
        }
        self.expect(TokenKind::RBrace)?;
        Ok(ImplBlock {
            target_type: target,
            trait_name,
            methods,
            span,
        })
    }

    fn parse_const(&mut self) -> Result<ConstDef, ParseError> {
        let span = self.current_span();
        self.advance(); // skip 'const' or 'static'
        let name = self.expect_ident()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_type()?;
        self.expect(TokenKind::Eq)?;
        let value = self.parse_expr()?;
        Ok(ConstDef { name, ty, value, span })
    }

    fn parse_use(&mut self) -> Result<UseDecl, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::Use)?;
        let mut path = Vec::new();
        path.push(self.expect_ident()?);
        while self.check(TokenKind::ColonColon) {
            self.advance();
            path.push(self.expect_ident()?);
        }
        Ok(UseDecl { path, span })
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        let span = self.current_span();
        self.expect(TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Block { stmts, span })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek_kind() {
            Some(TokenKind::Keyword(Keyword::Let)) => self.parse_let(),
            Some(TokenKind::Keyword(Keyword::Return)) => {
                let span = self.current_span();
                self.advance();
                if self.check(TokenKind::Newline)
                    || self.check(TokenKind::Semicolon)
                    || self.check(TokenKind::RBrace)
                {
                    if self.check(TokenKind::Semicolon) { self.advance(); }
                    Ok(Stmt::Return(None, span))
                } else {
                    let val = self.parse_expr()?;
                    if self.check(TokenKind::Semicolon) { self.advance(); }
                    Ok(Stmt::Return(Some(val), span))
                }
            }
            Some(TokenKind::Keyword(Keyword::Break)) => {
                let span = self.current_span();
                self.advance();
                Ok(Stmt::Break(span))
            }
            Some(TokenKind::Keyword(Keyword::Continue)) => {
                let span = self.current_span();
                self.advance();
                Ok(Stmt::Continue(span))
            }
            Some(TokenKind::Keyword(Keyword::While)) => {
                let span = self.current_span();
                self.advance();
                let condition = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Stmt::While { condition, body, span })
            }
            Some(TokenKind::Keyword(Keyword::For)) => {
                let span = self.current_span();
                self.advance();
                let variable = self.expect_ident()?;
                self.expect_keyword(Keyword::In)?;
                let iterable = self.parse_expr()?;
                let body = self.parse_block()?;
                Ok(Stmt::For { variable, iterable, body, span })
            }
            Some(TokenKind::Keyword(Keyword::Loop)) => {
                let span = self.current_span();
                self.advance();
                let body = self.parse_block()?;
                Ok(Stmt::Loop { body, span })
            }
            _ => {
                let expr = self.parse_expr()?;
                if self.check(TokenKind::Semicolon) {
                    self.advance();
                }
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::Let)?;
        let mutable = self.check_keyword(Keyword::Mut);
        if mutable {
            self.advance();
        }
        let name = self.expect_ident()?;
        let ty = if self.check(TokenKind::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };
        let value = if self.check(TokenKind::Eq) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };
        if self.check(TokenKind::Semicolon) {
            self.advance();
        }
        Ok(Stmt::Let { name, mutable, ty, value, span })
    }

    // ── Expression Parsing (Pratt-style precedence climbing) ─────────

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_or()?;
        if self.check(TokenKind::Eq) {
            let span = self.current_span();
            self.advance();
            let right = self.parse_assignment()?;
            return Ok(Expr::Assign {
                target: Box::new(left),
                value: Box::new(right),
                span,
            });
        }
        Ok(left)
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while self.check(TokenKind::PipePipe) {
            let span = self.current_span();
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;
        while self.check(TokenKind::AmpAmp) {
            let span = self.current_span();
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek_kind() {
                Some(TokenKind::EqEq) => BinaryOp::Eq,
                Some(TokenKind::BangEq) => BinaryOp::NotEq,
                Some(TokenKind::Lt) => BinaryOp::Lt,
                Some(TokenKind::LtEq) => BinaryOp::LtEq,
                Some(TokenKind::Gt) => BinaryOp::Gt,
                Some(TokenKind::GtEq) => BinaryOp::GtEq,
                _ => break,
            };
            let span = self.current_span();
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.peek_kind() {
                Some(TokenKind::Plus) => BinaryOp::Add,
                Some(TokenKind::Minus) => BinaryOp::Sub,
                _ => break,
            };
            let span = self.current_span();
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek_kind() {
                Some(TokenKind::Star) => BinaryOp::Mul,
                Some(TokenKind::Slash) => BinaryOp::Div,
                Some(TokenKind::Percent) => BinaryOp::Mod,
                _ => break,
            };
            let span = self.current_span();
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek_kind() {
            Some(TokenKind::Minus) => {
                let span = self.current_span();
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Neg, operand: Box::new(operand), span })
            }
            Some(TokenKind::Bang) => {
                let span = self.current_span();
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Not, operand: Box::new(operand), span })
            }
            Some(TokenKind::Tilde) => {
                let span = self.current_span();
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::BitNot, operand: Box::new(operand), span })
            }
            Some(TokenKind::Ampersand) => {
                let span = self.current_span();
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Ref, operand: Box::new(operand), span })
            }
            Some(TokenKind::Star) => {
                let span = self.current_span();
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Deref, operand: Box::new(operand), span })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek_kind() {
                Some(TokenKind::LParen) => {
                    let span = self.current_span();
                    self.advance();
                    let mut args = Vec::new();
                    self.skip_newlines();
                    if !self.check(TokenKind::RParen) {
                        loop {
                            self.skip_newlines();
                            args.push(self.parse_expr()?);
                            self.skip_newlines();
                            if !self.check(TokenKind::Comma) {
                                break;
                            }
                            self.advance();
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                        span,
                    };
                }
                Some(TokenKind::Dot) => {
                    let span = self.current_span();
                    self.advance();
                    let field = self.expect_ident()?;
                    expr = Expr::FieldAccess {
                        object: Box::new(expr),
                        field,
                        span,
                    };
                }
                Some(TokenKind::LBracket) => {
                    let span = self.current_span();
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(TokenKind::RBracket)?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                        span,
                    };
                }
                Some(TokenKind::Keyword(Keyword::As)) => {
                    let span = self.current_span();
                    self.advance();
                    let target_type = self.parse_type()?;
                    expr = Expr::Cast {
                        expr: Box::new(expr),
                        target_type,
                        span,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.peek_kind() {
            Some(TokenKind::IntegerLiteral(n)) => {
                let n = n;
                let span = self.current_span();
                self.advance();
                Ok(Expr::IntLiteral(n, span))
            }
            Some(TokenKind::FloatLiteral(f)) => {
                let f = f;
                let span = self.current_span();
                self.advance();
                Ok(Expr::FloatLiteral(f, span))
            }
            Some(TokenKind::StringLiteral(s)) => {
                let s = s.clone();
                let span = self.current_span();
                self.advance();
                Ok(Expr::StringLiteral(s, span))
            }
            Some(TokenKind::CharLiteral(c)) => {
                let c = c;
                let span = self.current_span();
                self.advance();
                Ok(Expr::CharLiteral(c, span))
            }
            Some(TokenKind::Keyword(Keyword::True)) => {
                let span = self.current_span();
                self.advance();
                Ok(Expr::BoolLiteral(true, span))
            }
            Some(TokenKind::Keyword(Keyword::False)) => {
                let span = self.current_span();
                self.advance();
                Ok(Expr::BoolLiteral(false, span))
            }
            Some(TokenKind::Identifier(_)) => {
                let span = self.current_span();
                let name = self.expect_ident()?;
                // Check for :: path
                if self.check(TokenKind::ColonColon) {
                    let mut path = vec![name];
                    while self.check(TokenKind::ColonColon) {
                        self.advance();
                        path.push(self.expect_ident()?);
                    }
                    return Ok(Expr::Path(path, span));
                }
                Ok(Expr::Ident(name, span))
            }
            Some(TokenKind::LParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            Some(TokenKind::LBracket) => {
                let span = self.current_span();
                self.advance();
                let mut elems = Vec::new();
                self.skip_newlines();
                if !self.check(TokenKind::RBracket) {
                    loop {
                        self.skip_newlines();
                        elems.push(self.parse_expr()?);
                        self.skip_newlines();
                        if !self.check(TokenKind::Comma) {
                            break;
                        }
                        self.advance();
                    }
                }
                self.expect(TokenKind::RBracket)?;
                Ok(Expr::Array(elems, span))
            }
            Some(TokenKind::LBrace) => {
                let block = self.parse_block()?;
                Ok(Expr::Block(block))
            }
            Some(TokenKind::Keyword(Keyword::If)) => self.parse_if_expr(),
            Some(TokenKind::Keyword(Keyword::Match)) => self.parse_match_expr(),
            _ => Err(self.error("expected expression")),
        }
    }

    fn parse_if_expr(&mut self) -> Result<Expr, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::If)?;
        let condition = self.parse_expr()?;
        let then_block = self.parse_block()?;
        let else_block = if self.check_keyword(Keyword::Else) {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Expr::If {
            condition: Box::new(condition),
            then_block,
            else_block,
            span,
        })
    }

    fn parse_match_expr(&mut self) -> Result<Expr, ParseError> {
        let span = self.current_span();
        self.expect_keyword(Keyword::Match)?;
        let scrutinee = self.parse_expr()?;
        self.expect(TokenKind::LBrace)?;
        let mut arms = Vec::new();
        self.skip_newlines();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            let pattern = self.parse_pattern()?;
            self.expect(TokenKind::FatArrow)?;
            let body = self.parse_expr()?;
            arms.push(MatchArm { pattern, body });
            self.skip_newlines();
            if self.check(TokenKind::Comma) {
                self.advance();
            }
            self.skip_newlines();
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Expr::Match {
            scrutinee: Box::new(scrutinee),
            arms,
            span,
        })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        match self.peek_kind() {
            Some(TokenKind::Identifier(s)) if s == "_" => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            Some(TokenKind::Identifier(_)) => {
                let name = self.expect_ident()?;
                if self.check(TokenKind::LParen) {
                    self.advance();
                    let mut fields = Vec::new();
                    if !self.check(TokenKind::RParen) {
                        loop {
                            fields.push(self.parse_pattern()?);
                            if !self.check(TokenKind::Comma) {
                                break;
                            }
                            self.advance();
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    Ok(Pattern::Enum { name, fields })
                } else {
                    Ok(Pattern::Ident(name))
                }
            }
            Some(TokenKind::IntegerLiteral(n)) => {
                let n = n;
                self.advance();
                Ok(Pattern::IntLiteral(n))
            }
            Some(TokenKind::Keyword(Keyword::True)) => {
                self.advance();
                Ok(Pattern::BoolLiteral(true))
            }
            Some(TokenKind::Keyword(Keyword::False)) => {
                self.advance();
                Ok(Pattern::BoolLiteral(false))
            }
            Some(TokenKind::StringLiteral(s)) => {
                let s = s.clone();
                self.advance();
                Ok(Pattern::StringLiteral(s))
            }
            _ => Err(self.error("expected pattern")),
        }
    }

    // ── Type parsing ────────────────────────────────────────────────

    fn parse_type(&mut self) -> Result<TypeExpr, ParseError> {
        match self.peek_kind() {
            Some(TokenKind::Ampersand) => {
                self.advance();
                let mutable = self.check_keyword(Keyword::Mut);
                if mutable {
                    self.advance();
                }
                let inner = self.parse_type()?;
                Ok(TypeExpr::Reference { mutable, inner: Box::new(inner) })
            }
            Some(TokenKind::Star) => {
                self.advance();
                let mutable = self.check_keyword(Keyword::Mut);
                if mutable {
                    self.advance();
                }
                let inner = self.parse_type()?;
                Ok(TypeExpr::Pointer { mutable, inner: Box::new(inner) })
            }
            Some(TokenKind::LBracket) => {
                self.advance();
                let element = self.parse_type()?;
                let size = if self.check(TokenKind::Semicolon) {
                    self.advance();
                    if let Some(TokenKind::IntegerLiteral(n)) = self.peek_kind() {
                        let n = n;
                        self.advance();
                        Some(n)
                    } else {
                        None
                    }
                } else {
                    None
                };
                self.expect(TokenKind::RBracket)?;
                Ok(TypeExpr::Array { element: Box::new(element), size })
            }
            Some(TokenKind::LParen) => {
                self.advance();
                let mut types = Vec::new();
                if !self.check(TokenKind::RParen) {
                    loop {
                        types.push(self.parse_type()?);
                        if !self.check(TokenKind::Comma) {
                            break;
                        }
                        self.advance();
                    }
                }
                self.expect(TokenKind::RParen)?;
                if types.is_empty() {
                    Ok(TypeExpr::Unit)
                } else {
                    Ok(TypeExpr::Tuple(types))
                }
            }
            Some(TokenKind::Identifier(_)) => {
                let name = self.expect_ident()?;
                if self.check(TokenKind::Lt) {
                    self.advance();
                    let mut args = Vec::new();
                    loop {
                        args.push(self.parse_type()?);
                        if !self.check(TokenKind::Comma) {
                            break;
                        }
                        self.advance();
                    }
                    self.expect(TokenKind::Gt)?;
                    Ok(TypeExpr::Generic(name, args))
                } else {
                    Ok(TypeExpr::Named(name))
                }
            }
            _ => {
                Ok(TypeExpr::Infer)
            }
        }
    }

    // ── Utility ─────────────────────────────────────────────────────

    fn peek_kind(&self) -> Option<TokenKind> {
        self.tokens.get(self.pos).map(|t| t.kind.clone())
    }

    fn current_span(&self) -> SourceSpan {
        self.tokens
            .get(self.pos)
            .map(|t| t.span.clone())
            .unwrap_or_else(SourceSpan::synthetic)
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek_kind().map_or(false, |k| {
            core::mem::discriminant(&k) == core::mem::discriminant(&kind)
        })
    }

    fn check_keyword(&self, kw: Keyword) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::Keyword(k)) if k == kw)
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.check(kind.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&alloc::format!("expected {:?}", kind)))
        }
    }

    fn expect_keyword(&mut self, kw: Keyword) -> Result<(), ParseError> {
        if self.check_keyword(kw) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&alloc::format!("expected keyword {:?}", kw)))
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParseError> {
        match self.peek_kind() {
            Some(TokenKind::Identifier(s)) => {
                self.advance();
                Ok(s)
            }
            _ => Err(self.error("expected identifier")),
        }
    }

    fn is_eof(&self) -> bool {
        matches!(self.peek_kind(), Some(TokenKind::Eof) | None)
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek_kind(), Some(TokenKind::Newline) | Some(TokenKind::Semicolon)) {
            self.advance();
        }
    }

    fn error(&self, msg: &str) -> ParseError {
        ParseError {
            message: String::from(msg),
            span: self.current_span(),
        }
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lexer::Lexer;

    fn parse(source: &str) -> Result<Program, ParseError> {
        let mut lexer = Lexer::new(source, "test.sg");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    #[test]
    fn test_parse_simple_function() {
        let prog = parse("fn main() { return 42 }").unwrap();
        assert_eq!(prog.items.len(), 1);
        match &prog.items[0] {
            Item::Function(f) => {
                assert_eq!(f.name, "main");
                assert!(f.params.is_empty());
            }
            _ => panic!("expected function"),
        }
    }

    #[test]
    fn test_parse_struct() {
        let prog = parse("struct Point { x: i32, y: i32 }").unwrap();
        match &prog.items[0] {
            Item::StructDef(s) => {
                assert_eq!(s.name, "Point");
                assert_eq!(s.fields.len(), 2);
            }
            _ => panic!("expected struct"),
        }
    }

    #[test]
    fn test_parse_let_binding() {
        let prog = parse("fn test() { let mut x: i32 = 10 }").unwrap();
        match &prog.items[0] {
            Item::Function(f) => {
                match &f.body.stmts[0] {
                    Stmt::Let { name, mutable, .. } => {
                        assert_eq!(name, "x");
                        assert!(*mutable);
                    }
                    _ => panic!("expected let"),
                }
            }
            _ => panic!("expected function"),
        }
    }

    #[test]
    fn test_parse_binary_expr() {
        let prog = parse("fn test() { 1 + 2 * 3 }").unwrap();
        match &prog.items[0] {
            Item::Function(f) => {
                assert_eq!(f.body.stmts.len(), 1);
            }
            _ => panic!("expected function"),
        }
    }
}
