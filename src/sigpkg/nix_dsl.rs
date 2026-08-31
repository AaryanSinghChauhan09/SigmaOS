#![allow(dead_code)]
use alloc::boxed::Box;
// Purely functional Nix DSL parser and derivation evaluator for SigmaOS
// Enables content-addressed store derivations, deterministic hashes, and Nix expressions

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Represents a Nix AST expression node
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NixExpr {
    StringLit(String),
    IntLit(i64),
    BoolLit(bool),
    List(Vec<NixExpr>),
    AttrSet(BTreeMap<String, NixExpr>),
    Function {
        arg_name: String,
        body: alloc::boxed::Box<NixExpr>,
    },
    Var(String),
    DerivationCall(NixDerivationSpec),
}

/// Representation of a Nix derivation (.drv) specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NixDerivationSpec {
    pub name: String,
    pub system: String,
    pub builder: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub src_hash: Option<String>,
}

impl NixDerivationSpec {
    pub fn new(name: &str, system: &str, builder: &str) -> Self {
        Self {
            name: name.to_string(),
            system: system.to_string(),
            builder: builder.to_string(),
            args: Vec::new(),
            env: BTreeMap::new(),
            src_hash: None,
        }
    }

    /// Computes deterministic content-addressed store path hash
    pub fn compute_store_hash(&self) -> String {
        let mut crc: u32 = 0xFFFFFFFF;
        let combined = format!(
            "name:{};system:{};builder:{};args:{:?};env:{:?};hash:{:?}",
            self.name, self.system, self.builder, self.args, self.env, self.src_hash
        );

        for byte in combined.as_bytes() {
            crc ^= *byte as u32;
            for _ in 0..8 {
                if (crc & 1) != 0 {
                    crc = (crc >> 1) ^ 0x82F63B78;
                } else {
                    crc >>= 1;
                }
            }
        }
        format!("{:08x}{:08x}", crc, !crc)
    }

    /// Evaluates store output path (/nix/store/<hash>-<name>)
    pub fn get_out_path(&self) -> String {
        format!("/nix/store/{}-{}", self.compute_store_hash(), self.name)
    }
}

/// Evaluator engine for Nix expression AST
pub struct NixDslEvaluator {
    pub store_paths: Vec<String>,
    pub environment_scope: BTreeMap<String, NixExpr>,
}

impl NixDslEvaluator {
    pub fn new() -> Self {
        let mut scope = BTreeMap::new();
        scope.insert("true".to_string(), NixExpr::BoolLit(true));
        scope.insert("false".to_string(), NixExpr::BoolLit(false));
        Self {
            store_paths: Vec::new(),
            environment_scope: scope,
        }
    }

    pub fn evaluate(&mut self, expr: &NixExpr) -> Result<NixExpr, &'static str> {
        match expr {
            NixExpr::StringLit(_) | NixExpr::IntLit(_) | NixExpr::BoolLit(_) => Ok(expr.clone()),
            NixExpr::Var(name) => self
                .environment_scope
                .get(name)
                .cloned()
                .ok_or("Undefined variable in Nix expression scope"),
            NixExpr::List(list) => {
                let mut eval_list = Vec::new();
                for item in list {
                    eval_list.push(self.evaluate(item)?);
                }
                Ok(NixExpr::List(eval_list))
            }
            NixExpr::AttrSet(attrs) => {
                let mut eval_attrs = BTreeMap::new();
                for (k, v) in attrs {
                    eval_attrs.insert(k.clone(), self.evaluate(v)?);
                }
                Ok(NixExpr::AttrSet(eval_attrs))
            }
            NixExpr::Function { arg_name, body } => Ok(NixExpr::Function {
                arg_name: arg_name.clone(),
                body: body.clone(),
            }),
            NixExpr::DerivationCall(spec) => {
                let out_path = spec.get_out_path();
                self.store_paths.push(out_path.clone());
                Ok(NixExpr::StringLit(out_path))
            }
        }
    }

    pub fn apply_function(
        &mut self,
        func: &NixExpr,
        arg: &NixExpr,
    ) -> Result<NixExpr, &'static str> {
        if let NixExpr::Function { arg_name, body } = func {
            let eval_arg = self.evaluate(arg)?;
            let previous = self.environment_scope.insert(arg_name.clone(), eval_arg);
            let result = self.evaluate(body);
            if let Some(prev) = previous {
                self.environment_scope.insert(arg_name.clone(), prev);
            } else {
                self.environment_scope.remove(arg_name);
            }
            result
        } else {
            Err("Attempted to apply non-function Nix expression")
        }
    }
}

impl Default for NixDslEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_derivation_store_hash() {
        let mut spec = NixDerivationSpec::new("hello-2.12", "x86_64-linux", "/bin/sh");
        spec.args.push("-c".to_string());
        spec.args.push("make install".to_string());
        spec.env
            .insert("src".to_string(), "hello-2.12.tar.gz".to_string());

        let out_path = spec.get_out_path();
        assert!(out_path.starts_with("/nix/store/"));
        assert!(out_path.ends_with("-hello-2.12"));
    }

    #[test]
    fn test_nix_dsl_evaluator() {
        let mut evaluator = NixDslEvaluator::new();
        let spec = NixDerivationSpec::new("coreutils", "x86_64-linux", "stdenv.mkDerivation");
        let expr = NixExpr::DerivationCall(spec);

        let res = evaluator.evaluate(&expr).unwrap();
        if let NixExpr::StringLit(path) = res {
            assert!(path.contains("coreutils"));
            assert_eq!(evaluator.store_paths.len(), 1);
        } else {
            panic!("Expected StringLit out path");
        }
    }
}
