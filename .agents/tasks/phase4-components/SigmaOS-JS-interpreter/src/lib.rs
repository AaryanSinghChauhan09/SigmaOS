//! SigmaOS JS Interpreter
//!
//! Lightweight embedded JavaScript interpreter for SigmaOS automation
//! scripts, extension manifests, and policy evaluation. Distinct from
//! mozjs (full SpiderMonkey engine): this is a minimal tree-walking
//! interpreter for safe, sandboxed script execution.
//! Upstream: Boa engine (https://boajs.dev/),
//!           QuickJS (https://bellard.org/quickjs/)

use std::collections::HashMap;

/// Primitive value types supported by the interpreter.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Undefined,
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Value>),
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Value::Undefined | Value::Null => false,
            Value::Bool(b)  => *b,
            Value::Int(n)   => *n != 0,
            Value::Float(f) => *f != 0.0 && !f.is_nan(),
            Value::Str(s)   => !s.is_empty(),
            Value::List(l)  => !l.is_empty(),
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(n)   => Some(*n),
            Value::Float(f) => Some(*f as i64),
            Value::Bool(b)  => Some(if *b { 1 } else { 0 }),
            _               => None,
        }
    }

    pub fn type_tag(&self) -> &'static str {
        match self {
            Value::Undefined => "undefined",
            Value::Null      => "null",
            Value::Bool(_)   => "boolean",
            Value::Int(_)    => "number",
            Value::Float(_)  => "number",
            Value::Str(_)    => "string",
            Value::List(_)   => "object",
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Undefined   => write!(f, "undefined"),
            Value::Null        => write!(f, "null"),
            Value::Bool(b)     => write!(f, "{b}"),
            Value::Int(n)      => write!(f, "{n}"),
            Value::Float(v)    => write!(f, "{v}"),
            Value::Str(s)      => write!(f, "{s}"),
            Value::List(l)     => write!(f, "[{}]", l.iter().map(|v| v.to_string())
                                            .collect::<Vec<_>>().join(", ")),
        }
    }
}

/// Execution error with position information.
#[derive(Debug, Clone)]
pub struct InterpError {
    pub message: String,
    pub line:    usize,
}

impl InterpError {
    pub fn new(msg: &str, line: usize) -> Self {
        Self { message: msg.to_owned(), line }
    }
}

impl std::fmt::Display for InterpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at line {}: {}", self.line, self.message)
    }
}

/// Sandbox limits for the interpreter.
#[derive(Debug, Clone)]
pub struct SandboxLimits {
    pub max_steps:    u64,
    pub max_stack:    usize,
    pub allow_io:     bool,
    pub allow_net:    bool,
}

impl Default for SandboxLimits {
    fn default() -> Self {
        Self { max_steps: 100_000, max_stack: 256, allow_io: false, allow_net: false }
    }
}

/// The interpreter: environment + sandbox.
pub struct Interpreter {
    env:    HashMap<String, Value>,
    limits: SandboxLimits,
    steps:  u64,
    output: Vec<String>,
}

impl Interpreter {
    pub fn new(limits: SandboxLimits) -> Self {
        let mut env = HashMap::new();
        env.insert("undefined".into(), Value::Undefined);
        env.insert("null".into(),      Value::Null);
        env.insert("true".into(),      Value::Bool(true));
        env.insert("false".into(),     Value::Bool(false));
        Self { env, limits, steps: 0, output: Vec::new() }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.env.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> { self.env.get(name) }

    /// Evaluate a simple expression / statement.
    /// Supports: literals, variable lookup, `print(...)`, arithmetic (+/-).
    pub fn eval(&mut self, src: &str) -> Result<Value, InterpError> {
        self.steps += 1;
        if self.steps > self.limits.max_steps {
            return Err(InterpError::new("step limit exceeded", 0));
        }

        let src = src.trim();
        if src.is_empty() { return Ok(Value::Undefined); }

        // print(...) / console.log(...)
        let print_call = if src.starts_with("print(") && src.ends_with(')') {
            Some(&src["print(".len()..src.len()-1])
        } else if src.starts_with("console.log(") && src.ends_with(')') {
            Some(&src["console.log(".len()..src.len()-1])
        } else {
            None
        };
        if let Some(inner) = print_call {
            let v = self.eval(inner)?;
            self.output.push(v.to_string());
            return Ok(Value::Undefined);
        }

        // Addition: a + b (integers only)
        if let Some(pos) = src.find(" + ") {
            let left  = self.eval(&src[..pos])?;
            let right = self.eval(&src[pos+3..])?;
            match (&left, &right) {
                (Value::Int(a),   Value::Int(b))   => return Ok(Value::Int(a + b)),
                (Value::Float(a), Value::Float(b)) => return Ok(Value::Float(a + b)),
                (Value::Str(a),   Value::Str(b))   => return Ok(Value::Str(format!("{a}{b}"))),
                _ => {}
            }
        }

        // Subtraction: a - b
        if let Some(pos) = src.rfind(" - ") {
            let left  = self.eval(&src[..pos])?;
            let right = self.eval(&src[pos+3..])?;
            match (&left, &right) {
                (Value::Int(a),   Value::Int(b))   => return Ok(Value::Int(a - b)),
                (Value::Float(a), Value::Float(b)) => return Ok(Value::Float(a - b)),
                _ => {}
            }
        }

        // Boolean literals
        if src == "true"  { return Ok(Value::Bool(true));  }
        if src == "false" { return Ok(Value::Bool(false)); }
        if src == "null"  { return Ok(Value::Null); }
        if src == "undefined" { return Ok(Value::Undefined); }

        // Integer literal
        if let Ok(n) = src.parse::<i64>() { return Ok(Value::Int(n)); }
        // Float literal
        if let Ok(f) = src.parse::<f64>() { return Ok(Value::Float(f)); }

        // String literal
        if (src.starts_with('"') && src.ends_with('"'))
            || (src.starts_with('\'') && src.ends_with('\'')) {
            let inner = &src[1..src.len()-1];
            return Ok(Value::Str(inner.to_owned()));
        }

        // Variable lookup
        if let Some(v) = self.env.get(src) { return Ok(v.clone()); }

        Err(InterpError::new(&format!("undefined: {src}"), 0))
    }

    /// Run multiple statements separated by semicolons or newlines.
    pub fn run(&mut self, script: &str) -> Result<Value, InterpError> {
        let stmts: Vec<&str> = script
            .split(|c| c == ';' || c == '\n')
            .map(str::trim)
            .filter(|s| !s.is_empty() && !s.starts_with("//"))
            .collect();
        let mut last = Value::Undefined;
        for stmt in stmts {
            last = self.eval(stmt)?;
        }
        Ok(last)
    }

    pub fn output(&self) -> &[String] { &self.output }
    pub fn steps_used(&self) -> u64   { self.steps }
    pub fn reset_steps(&mut self)     { self.steps = 0; }
}

impl Default for Interpreter {
    fn default() -> Self { Self::new(SandboxLimits::default()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_literals() {
        let mut i = Interpreter::default();
        assert_eq!(i.eval("42").unwrap(),      Value::Int(42));
        assert_eq!(i.eval("true").unwrap(),    Value::Bool(true));
        assert_eq!(i.eval("null").unwrap(),    Value::Null);
        assert_eq!(i.eval("\"hi\"").unwrap(),  Value::Str("hi".into()));
    }

    #[test]
    fn eval_addition() {
        let mut i = Interpreter::default();
        assert_eq!(i.eval("1 + 2").unwrap(), Value::Int(3));
    }

    #[test]
    fn eval_string_concat() {
        let mut i = Interpreter::default();
        assert_eq!(i.eval("\"foo\" + \"bar\"").unwrap(), Value::Str("foobar".into()));
    }

    #[test]
    fn variable_set_get() {
        let mut i = Interpreter::default();
        i.set("x", Value::Int(10));
        assert_eq!(i.eval("x").unwrap(), Value::Int(10));
    }

    #[test]
    fn print_captures_output() {
        let mut i = Interpreter::default();
        i.eval("print(42)").unwrap();
        assert_eq!(i.output(), &["42"]);
    }

    #[test]
    fn run_multi_statement() {
        let mut i = Interpreter::default();
        i.set("a", Value::Int(5));
        i.set("b", Value::Int(3));
        let r = i.run("print(\"start\"); a + b").unwrap();
        assert_eq!(r, Value::Int(8));
        assert_eq!(i.output()[0], "start");
    }

    #[test]
    fn step_limit_enforced() {
        let mut i = Interpreter::new(SandboxLimits { max_steps: 2, ..Default::default() });
        i.eval("1").unwrap();
        i.eval("2").unwrap();
        assert!(i.eval("3").is_err());
    }

    #[test]
    fn value_truthy() {
        assert!( Value::Int(1).truthy());
        assert!(!Value::Int(0).truthy());
        assert!(!Value::Null.truthy());
        assert!( Value::Str("x".into()).truthy());
    }
}
