//! SigmaOS mozjs
//!
//! SpiderMonkey / mozjs JavaScript engine integration scaffold.
//! Provides the embedding API surface that SigmaOS components use
//! to evaluate JS policies, extensions, and automation scripts.
//! Upstream: mozjs crate (https://crates.io/crates/mozjs),
//!           SpiderMonkey (https://spidermonkey.dev/)

use std::collections::HashMap;

/// A JS value in the SigmaOS embedding model (simplified type system).
#[derive(Debug, Clone, PartialEq)]
pub enum JsValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsValue>),
    Object(HashMap<String, JsValue>),
}

impl JsValue {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Undefined | Self::Null => false,
            Self::Boolean(b)  => *b,
            Self::Number(n)   => *n != 0.0 && !n.is_nan(),
            Self::String(s)   => !s.is_empty(),
            Self::Array(_)    => true,
            Self::Object(_)   => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Undefined  => "undefined",
            Self::Null       => "null",
            Self::Boolean(_) => "boolean",
            Self::Number(_)  => "number",
            Self::String(_)  => "string",
            Self::Array(_)   => "array",
            Self::Object(_)  => "object",
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            Self::Undefined    => "undefined".to_owned(),
            Self::Null         => "null".to_owned(),
            Self::Boolean(b)   => b.to_string(),
            Self::Number(n)    => {
                if n.fract() == 0.0 { format!("{}", *n as i64) }
                else { format!("{n}") }
            }
            Self::String(s)    => s.clone(),
            Self::Array(a)     => format!("[{}]", a.len()),
            Self::Object(o)    => format!("{{...{} keys}}", o.len()),
        }
    }
}

/// Evaluation result from the JS runtime.
#[derive(Debug)]
pub struct EvalResult {
    pub value:     JsValue,
    pub error:     Option<String>,
    pub duration_us: u64,
}

impl EvalResult {
    pub fn ok(value: JsValue) -> Self { Self { value, error: None, duration_us: 0 } }
    pub fn err(msg: &str) -> Self { Self { value: JsValue::Undefined, error: Some(msg.to_owned()), duration_us: 0 } }
    pub fn is_ok(&self) -> bool { self.error.is_none() }
}

/// JS execution context (wraps an engine instance).
pub struct JsContext {
    globals: HashMap<String, JsValue>,
    log:     Vec<String>,
}

impl JsContext {
    pub fn new() -> Self {
        let mut ctx = Self { globals: HashMap::new(), log: Vec::new() };
        // Inject sigma global
        let mut sigma = HashMap::new();
        sigma.insert("version".into(), JsValue::String("0.5".into()));
        sigma.insert("platform".into(), JsValue::String("sigmaos".into()));
        ctx.globals.insert("sigma".into(), JsValue::Object(sigma));
        ctx
    }

    pub fn set_global(&mut self, name: &str, value: JsValue) {
        self.globals.insert(name.to_owned(), value);
    }

    pub fn get_global(&self, name: &str) -> Option<&JsValue> {
        self.globals.get(name)
    }

    /// Minimal "eval" scaffold: recognises a tiny expression subset.
    /// Real embedding would call into the mozjs crate.
    pub fn eval(&mut self, script: &str) -> EvalResult {
        let script = script.trim();
        // Literal values
        if script == "undefined" { return EvalResult::ok(JsValue::Undefined); }
        if script == "null"      { return EvalResult::ok(JsValue::Null); }
        if script == "true"      { return EvalResult::ok(JsValue::Boolean(true)); }
        if script == "false"     { return EvalResult::ok(JsValue::Boolean(false)); }

        // Number literal
        if let Ok(n) = script.parse::<f64>() {
            return EvalResult::ok(JsValue::Number(n));
        }
        // String literal "..."
        if script.starts_with('"') && script.ends_with('"') && script.len() >= 2 {
            return EvalResult::ok(JsValue::String(script[1..script.len()-1].to_owned()));
        }
        // Global variable lookup
        if let Some(v) = self.globals.get(script) {
            return EvalResult::ok(v.clone());
        }
        // console.log(...)
        if script.starts_with("console.log(") && script.ends_with(')') {
            let inner = &script["console.log(".len()..script.len()-1];
            self.log.push(inner.to_owned());
            return EvalResult::ok(JsValue::Undefined);
        }
        // Unknown → scaffold returns undefined with a note
        EvalResult::ok(JsValue::String(format!("[mozjs-scaffold: {script}")))
    }

    pub fn console_log(&self) -> &[String] { &self.log }
}

impl Default for JsContext { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_literals() {
        let mut ctx = JsContext::new();
        assert_eq!(ctx.eval("true").value,    JsValue::Boolean(true));
        assert_eq!(ctx.eval("false").value,   JsValue::Boolean(false));
        assert_eq!(ctx.eval("null").value,    JsValue::Null);
        assert_eq!(ctx.eval("42").value,      JsValue::Number(42.0));
        assert_eq!(ctx.eval("\"hello\"").value, JsValue::String("hello".into()));
    }

    #[test]
    fn eval_global_lookup() {
        let mut ctx = JsContext::new();
        ctx.set_global("x", JsValue::Number(7.0));
        let r = ctx.eval("x");
        assert_eq!(r.value, JsValue::Number(7.0));
    }

    #[test]
    fn js_value_truthy() {
        assert!(!JsValue::Undefined.is_truthy());
        assert!(!JsValue::Null.is_truthy());
        assert!(!JsValue::Boolean(false).is_truthy());
        assert!(JsValue::Boolean(true).is_truthy());
        assert!(JsValue::Number(1.0).is_truthy());
        assert!(!JsValue::Number(0.0).is_truthy());
        assert!(JsValue::String("hi".into()).is_truthy());
        assert!(!JsValue::String("".into()).is_truthy());
    }

    #[test]
    fn type_names() {
        assert_eq!(JsValue::Undefined.type_name(), "undefined");
        assert_eq!(JsValue::Number(0.0).type_name(), "number");
        assert_eq!(JsValue::String("".into()).type_name(), "string");
    }

    #[test]
    fn sigma_global_injected() {
        let ctx = JsContext::new();
        assert!(ctx.get_global("sigma").is_some());
        if let Some(JsValue::Object(obj)) = ctx.get_global("sigma") {
            assert_eq!(obj["platform"], JsValue::String("sigmaos".into()));
        }
    }
}
