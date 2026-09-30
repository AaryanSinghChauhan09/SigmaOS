// SigmaOS Compiler — Type Checker
// Performs semantic analysis: type inference, type checking, scope resolution,
// and produces a typed intermediate representation.

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use super::parser::{
    BinaryOp, Block, Expr, FunctionDef, Item, Pattern, Program, Stmt, TypeExpr, UnaryOp,
};

// ─── Type System ────────────────────────────────────────────────────────────

/// Internal type representation used during type checking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    // Primitives
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128,
    F32, F64,
    Bool,
    Char,
    Str,
    Unit,
    Never,

    // Compound
    Array(Box<Type>, Option<u64>),
    Tuple(Vec<Type>),
    Reference { mutable: bool, inner: Box<Type> },
    Pointer { mutable: bool, inner: Box<Type> },
    Function { params: Vec<Type>, ret: Box<Type> },

    // Named / user-defined
    Named(String),
    Generic(String, Vec<Type>),

    // Inference placeholder
    Inferred(u64),
    Unknown,
}

impl Type {
    /// Resolve a TypeExpr from the AST into an internal Type.
    pub fn from_type_expr(expr: &TypeExpr) -> Type {
        match expr {
            TypeExpr::Named(n) => Self::resolve_primitive(n),
            TypeExpr::Generic(name, args) => {
                Type::Generic(name.clone(), args.iter().map(Self::from_type_expr).collect())
            }
            TypeExpr::Reference { mutable, inner } => Type::Reference {
                mutable: *mutable,
                inner: Box::new(Self::from_type_expr(inner)),
            },
            TypeExpr::Pointer { mutable, inner } => Type::Pointer {
                mutable: *mutable,
                inner: Box::new(Self::from_type_expr(inner)),
            },
            TypeExpr::Array { element, size } => {
                Type::Array(Box::new(Self::from_type_expr(element)), *size)
            }
            TypeExpr::Tuple(types) => {
                Type::Tuple(types.iter().map(Self::from_type_expr).collect())
            }
            TypeExpr::Function { params, ret } => Type::Function {
                params: params.iter().map(Self::from_type_expr).collect(),
                ret: Box::new(Self::from_type_expr(ret)),
            },
            TypeExpr::Infer => Type::Unknown,
            TypeExpr::Unit => Type::Unit,
        }
    }

    fn resolve_primitive(name: &str) -> Type {
        match name {
            "i8" => Type::I8,
            "i16" => Type::I16,
            "i32" => Type::I32,
            "i64" => Type::I64,
            "i128" => Type::I128,
            "u8" => Type::U8,
            "u16" => Type::U16,
            "u32" => Type::U32,
            "u64" => Type::U64,
            "u128" => Type::U128,
            "f32" => Type::F32,
            "f64" => Type::F64,
            "bool" => Type::Bool,
            "char" => Type::Char,
            "str" | "String" => Type::Str,
            "()" => Type::Unit,
            other => Type::Named(String::from(other)),
        }
    }

    /// Check if two types are compatible for assignment / comparison.
    pub fn is_assignable_from(&self, other: &Type) -> bool {
        if self == other {
            return true;
        }
        // Integer widening
        if self.is_integer() && other.is_integer() {
            return self.integer_bits() >= other.integer_bits();
        }
        // Float widening
        if self.is_float() && other.is_float() {
            return true;
        }
        // Unknown matches anything (for inference)
        if matches!(self, Type::Unknown) || matches!(other, Type::Unknown) {
            return true;
        }
        false
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Type::I8 | Type::I16 | Type::I32 | Type::I64 | Type::I128
                | Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::U128
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Type::F32 | Type::F64)
    }

    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    pub fn integer_bits(&self) -> u32 {
        match self {
            Type::I8 | Type::U8 => 8,
            Type::I16 | Type::U16 => 16,
            Type::I32 | Type::U32 => 32,
            Type::I64 | Type::U64 => 64,
            Type::I128 | Type::U128 => 128,
            _ => 0,
        }
    }
}

// ─── Symbol Table ───────────────────────────────────────────────────────────

/// A symbol in the current scope.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub ty: Type,
    pub mutable: bool,
    pub defined: bool,
}

/// Scoped symbol table with parent chain.
#[derive(Debug)]
pub struct SymbolTable {
    scopes: Vec<BTreeMap<String, Symbol>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![BTreeMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(BTreeMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, name: &str, ty: Type, mutable: bool) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(
                String::from(name),
                Symbol {
                    name: String::from(name),
                    ty,
                    mutable,
                    defined: true,
                },
            );
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.get(name) {
                return Some(sym);
            }
        }
        None
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(sym) = scope.get_mut(name) {
                return Some(sym);
            }
        }
        None
    }
}

// ─── Type Checker ───────────────────────────────────────────────────────────

/// A type-check error.
#[derive(Debug, Clone)]
pub struct TypeError {
    pub message: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
}

/// Struct definitions registered for field lookups.
#[derive(Debug, Clone)]
pub struct StructInfo {
    pub name: String,
    pub fields: BTreeMap<String, Type>,
}

/// Function signatures registered for call checking.
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
}

/// The type checker engine.
pub struct TypeChecker {
    symbols: SymbolTable,
    structs: BTreeMap<String, StructInfo>,
    functions: BTreeMap<String, FunctionInfo>,
    errors: Vec<TypeError>,
    next_infer_id: u64,
    current_file: String,
    current_return_type: Option<Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            structs: BTreeMap::new(),
            functions: BTreeMap::new(),
            errors: Vec::new(),
            next_infer_id: 0,
            current_file: String::from("<unknown>"),
            current_return_type: None,
        }
    }

    /// Run type checking on a parsed program.
    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
        // First pass: register all type and function declarations
        for item in &program.items {
            self.register_item(item);
        }

        // Second pass: check function bodies
        for item in &program.items {
            self.check_item(item);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn register_item(&mut self, item: &Item) {
        match item {
            Item::Function(f) => {
                let params: Vec<(String, Type)> = f
                    .params
                    .iter()
                    .map(|(n, t)| (n.clone(), Type::from_type_expr(t)))
                    .collect();
                let return_type = f
                    .return_type
                    .as_ref()
                    .map(Type::from_type_expr)
                    .unwrap_or(Type::Unit);
                self.functions.insert(
                    f.name.clone(),
                    FunctionInfo {
                        name: f.name.clone(),
                        params,
                        return_type,
                    },
                );
                // Also add to symbol table as a function type
                let param_types: Vec<Type> = f
                    .params
                    .iter()
                    .map(|(_, t)| Type::from_type_expr(t))
                    .collect();
                let ret = f
                    .return_type
                    .as_ref()
                    .map(Type::from_type_expr)
                    .unwrap_or(Type::Unit);
                self.symbols.define(
                    &f.name,
                    Type::Function {
                        params: param_types,
                        ret: Box::new(ret),
                    },
                    false,
                );
            }
            Item::StructDef(s) => {
                let mut fields = BTreeMap::new();
                for (fname, ftype) in &s.fields {
                    fields.insert(fname.clone(), Type::from_type_expr(ftype));
                }
                self.structs.insert(
                    s.name.clone(),
                    StructInfo {
                        name: s.name.clone(),
                        fields,
                    },
                );
                self.symbols.define(&s.name, Type::Named(s.name.clone()), false);
            }
            Item::EnumDef(e) => {
                self.symbols.define(&e.name, Type::Named(e.name.clone()), false);
            }
            _ => {}
        }
    }

    fn check_item(&mut self, item: &Item) {
        match item {
            Item::Function(f) => self.check_function(f),
            _ => {}
        }
    }

    fn check_function(&mut self, f: &FunctionDef) {
        self.symbols.push_scope();

        let return_type = f
            .return_type
            .as_ref()
            .map(Type::from_type_expr)
            .unwrap_or(Type::Unit);
        self.current_return_type = Some(return_type);

        // Bind parameters
        for (name, ty_expr) in &f.params {
            let ty = Type::from_type_expr(ty_expr);
            self.symbols.define(name, ty, false);
        }

        self.check_block(&f.body);

        self.current_return_type = None;
        self.symbols.pop_scope();
    }

    fn check_block(&mut self, block: &Block) {
        self.symbols.push_scope();
        for stmt in &block.stmts {
            self.check_stmt(stmt);
        }
        self.symbols.pop_scope();
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let {
                name,
                mutable,
                ty,
                value,
                span,
            } => {
                let declared_type = ty.as_ref().map(Type::from_type_expr);
                let value_type = value.as_ref().map(|e| self.infer_expr(e));

                let final_type = match (&declared_type, &value_type) {
                    (Some(d), Some(v)) => {
                        if !d.is_assignable_from(v) {
                            self.errors.push(TypeError {
                                message: alloc::format!(
                                    "type mismatch: cannot assign {:?} to {:?}",
                                    v, d
                                ),
                                file: span.file.clone(),
                                line: span.line,
                                column: span.column,
                            });
                        }
                        d.clone()
                    }
                    (Some(d), None) => d.clone(),
                    (None, Some(v)) => v.clone(),
                    (None, None) => {
                        self.errors.push(TypeError {
                            message: alloc::format!(
                                "cannot infer type for '{}' without type annotation or initializer",
                                name
                            ),
                            file: span.file.clone(),
                            line: span.line,
                            column: span.column,
                        });
                        Type::Unknown
                    }
                };

                self.symbols.define(name, final_type, *mutable);
            }
            Stmt::Return(expr, span) => {
                let ret_type = expr.as_ref().map(|e| self.infer_expr(e)).unwrap_or(Type::Unit);
                if let Some(expected) = &self.current_return_type {
                    if !expected.is_assignable_from(&ret_type) {
                        self.errors.push(TypeError {
                            message: alloc::format!(
                                "return type mismatch: expected {:?}, got {:?}",
                                expected, ret_type
                            ),
                            file: span.file.clone(),
                            line: span.line,
                            column: span.column,
                        });
                    }
                }
            }
            Stmt::While { condition, body, span } => {
                let cond_type = self.infer_expr(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown {
                    self.errors.push(TypeError {
                        message: alloc::format!(
                            "while condition must be bool, got {:?}",
                            cond_type
                        ),
                        file: span.file.clone(),
                        line: span.line,
                        column: span.column,
                    });
                }
                self.check_block(body);
            }
            Stmt::For { variable, iterable: _, body, .. } => {
                self.symbols.push_scope();
                self.symbols.define(variable, Type::Unknown, false);
                self.check_block(body);
                self.symbols.pop_scope();
            }
            Stmt::Loop { body, .. } => {
                self.check_block(body);
            }
            Stmt::Expr(expr) => {
                let _ = self.infer_expr(expr);
            }
            Stmt::Break(_) | Stmt::Continue(_) => {}
        }
    }

    /// Infer the type of an expression.
    fn infer_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::IntLiteral(_, _) => Type::I64,
            Expr::FloatLiteral(_, _) => Type::F64,
            Expr::StringLiteral(_, _) => Type::Str,
            Expr::CharLiteral(_, _) => Type::Char,
            Expr::BoolLiteral(_, _) => Type::Bool,

            Expr::Ident(name, span) => {
                if let Some(sym) = self.symbols.lookup(name) {
                    sym.ty.clone()
                } else {
                    self.errors.push(TypeError {
                        message: alloc::format!("undefined variable '{}'", name),
                        file: span.file.clone(),
                        line: span.line,
                        column: span.column,
                    });
                    Type::Unknown
                }
            }

            Expr::Binary { left, op, right, span } => {
                let lt = self.infer_expr(left);
                let rt = self.infer_expr(right);

                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        if !lt.is_numeric() && lt != Type::Unknown {
                            self.errors.push(TypeError {
                                message: alloc::format!(
                                    "arithmetic operator requires numeric type, got {:?}",
                                    lt
                                ),
                                file: span.file.clone(),
                                line: span.line,
                                column: span.column,
                            });
                        }
                        // Promote to wider type
                        if lt.is_float() || rt.is_float() {
                            Type::F64
                        } else {
                            lt
                        }
                    }
                    BinaryOp::Eq | BinaryOp::NotEq | BinaryOp::Lt | BinaryOp::LtEq
                    | BinaryOp::Gt | BinaryOp::GtEq => Type::Bool,
                    BinaryOp::And | BinaryOp::Or => Type::Bool,
                    BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor
                    | BinaryOp::Shl | BinaryOp::Shr => lt,
                }
            }

            Expr::Unary { op, operand, .. } => {
                let inner = self.infer_expr(operand);
                match op {
                    UnaryOp::Neg => inner,
                    UnaryOp::Not => Type::Bool,
                    UnaryOp::BitNot => inner,
                    UnaryOp::Ref => Type::Reference {
                        mutable: false,
                        inner: Box::new(inner),
                    },
                    UnaryOp::Deref => match inner {
                        Type::Reference { inner, .. } | Type::Pointer { inner, .. } => *inner,
                        _ => Type::Unknown,
                    },
                }
            }

            Expr::Call { callee, args, span } => {
                let callee_type = self.infer_expr(callee);
                match callee_type {
                    Type::Function { params, ret } => {
                        if args.len() != params.len() {
                            self.errors.push(TypeError {
                                message: alloc::format!(
                                    "expected {} arguments, got {}",
                                    params.len(),
                                    args.len()
                                ),
                                file: span.file.clone(),
                                line: span.line,
                                column: span.column,
                            });
                        }
                        for (arg, param_ty) in args.iter().zip(params.iter()) {
                            let arg_ty = self.infer_expr(arg);
                            if !param_ty.is_assignable_from(&arg_ty) {
                                self.errors.push(TypeError {
                                    message: alloc::format!(
                                        "argument type mismatch: expected {:?}, got {:?}",
                                        param_ty, arg_ty
                                    ),
                                    file: span.file.clone(),
                                    line: span.line,
                                    column: span.column,
                                });
                            }
                        }
                        *ret
                    }
                    Type::Unknown => Type::Unknown,
                    _ => {
                        self.errors.push(TypeError {
                            message: alloc::format!("cannot call non-function type {:?}", callee_type),
                            file: span.file.clone(),
                            line: span.line,
                            column: span.column,
                        });
                        Type::Unknown
                    }
                }
            }

            Expr::FieldAccess { object, field, span } => {
                let obj_type = self.infer_expr(object);
                if let Type::Named(name) = &obj_type {
                    if let Some(info) = self.structs.get(name) {
                        if let Some(ft) = info.fields.get(field) {
                            return ft.clone();
                        }
                        self.errors.push(TypeError {
                            message: alloc::format!("no field '{}' on struct '{}'", field, name),
                            file: span.file.clone(),
                            line: span.line,
                            column: span.column,
                        });
                    }
                }
                Type::Unknown
            }

            Expr::Index { object, index, span } => {
                let obj_type = self.infer_expr(object);
                let idx_type = self.infer_expr(index);
                if !idx_type.is_integer() && idx_type != Type::Unknown {
                    self.errors.push(TypeError {
                        message: alloc::format!("index must be integer, got {:?}", idx_type),
                        file: span.file.clone(),
                        line: span.line,
                        column: span.column,
                    });
                }
                match obj_type {
                    Type::Array(elem, _) => *elem,
                    _ => Type::Unknown,
                }
            }

            Expr::If { condition, then_block, else_block, span } => {
                let cond_type = self.infer_expr(condition);
                if cond_type != Type::Bool && cond_type != Type::Unknown {
                    self.errors.push(TypeError {
                        message: alloc::format!("if condition must be bool, got {:?}", cond_type),
                        file: span.file.clone(),
                        line: span.line,
                        column: span.column,
                    });
                }
                self.check_block(then_block);
                if let Some(eb) = else_block {
                    self.check_block(eb);
                }
                Type::Unit
            }

            Expr::Match { scrutinee, arms, .. } => {
                let _ = self.infer_expr(scrutinee);
                for arm in arms {
                    let _ = self.infer_expr(&arm.body);
                }
                Type::Unknown
            }

            Expr::Block(block) => {
                self.check_block(block);
                Type::Unit
            }

            Expr::Array(elems, _) => {
                if elems.is_empty() {
                    return Type::Array(Box::new(Type::Unknown), Some(0));
                }
                let first = self.infer_expr(&elems[0]);
                Type::Array(Box::new(first), Some(elems.len() as u64))
            }

            Expr::Assign { target, value, span } => {
                let target_type = self.infer_expr(target);
                let value_type = self.infer_expr(value);
                if !target_type.is_assignable_from(&value_type) {
                    self.errors.push(TypeError {
                        message: alloc::format!(
                            "cannot assign {:?} to {:?}",
                            value_type, target_type
                        ),
                        file: span.file.clone(),
                        line: span.line,
                        column: span.column,
                    });
                }
                Type::Unit
            }

            Expr::Cast { target_type, .. } => Type::from_type_expr(target_type),

            Expr::StructLit { name, fields, span } => {
                if let Some(info) = self.structs.get(name).cloned() {
                    for (fname, fexpr) in fields {
                        let expr_type = self.infer_expr(fexpr);
                        if let Some(expected) = info.fields.get(fname) {
                            if !expected.is_assignable_from(&expr_type) {
                                self.errors.push(TypeError {
                                    message: alloc::format!(
                                        "field '{}' type mismatch: expected {:?}, got {:?}",
                                        fname, expected, expr_type
                                    ),
                                    file: span.file.clone(),
                                    line: span.line,
                                    column: span.column,
                                });
                            }
                        }
                    }
                }
                Type::Named(name.clone())
            }

            Expr::Path(_, _) => Type::Unknown,
        }
    }

    #[allow(dead_code)]
    fn fresh_infer(&mut self) -> Type {
        let id = self.next_infer_id;
        self.next_infer_id += 1;
        Type::Inferred(id)
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::lexer::Lexer;
    use crate::compiler::parser::Parser;

    fn typecheck(source: &str) -> Result<(), Vec<TypeError>> {
        let mut lexer = Lexer::new(source, "test.sg");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let program = parser.parse_program().unwrap();
        let mut checker = TypeChecker::new();
        checker.check_program(&program)
    }

    #[test]
    fn test_valid_function() {
        assert!(typecheck("fn add(a: i32, b: i32) -> i32 { return a }").is_ok());
    }

    #[test]
    fn test_type_mismatch_in_let() {
        let result = typecheck(r#"fn test() { let x: i32 = "hello" }"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_undefined_variable() {
        let result = typecheck("fn test() { return x }");
        assert!(result.is_err());
    }

    #[test]
    fn test_struct_field_access() {
        let result = typecheck(
            "struct Point { x: i32, y: i32 }\nfn test(p: Point) -> i32 { return p.x }",
        );
        assert!(result.is_ok());
    }
}
