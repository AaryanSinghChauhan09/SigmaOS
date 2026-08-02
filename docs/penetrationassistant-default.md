# docs/penetrationassistant-default.md

## Problem Description
The repository expects a penetrationassistant interface but lacks a default implementation, leading to build errors or undefined behavior when tests expect a default behavior.

## Root Cause Analysis
An interface/trait is declared but no safe default exists. Test harnesses or tools assume a no-op default or a basic implementation, causing compile-time or runtime failures.

## Proposed Fix
Provide a minimal, well-documented default implementation that:
- Is safe and deterministic.
- Logs actions at a debug level.
- Can be replaced at runtime/configuration with stronger implementations.
- Design as a trait with default methods (Rust) so consumers can rely on default behavior.

## Code Snippet (Rust — trait + default impl)

```rust
// name=docs/examples/rust_penetrationassistant.rs
use std::fmt::Debug;

pub trait PenetrationAssistant: Debug + Send + Sync {
    fn assess(&self, target: &str) -> anyhow::Result<Assessment> {
        // Default implementation: no-op assessment, safe and deterministic
        Ok(Assessment {
            target: target.to_string(),
            severity: Severity::Info,
            notes: "default no-op assessment".to_string(),
        })
    }

    fn remediate(&self, _assessment: &Assessment) -> anyhow::Result<()> {
        // Default: do nothing; log in production code
        Ok(())
    }
}

#[derive(Debug)]
pub struct DefaultAssistant;

impl PenetrationAssistant for DefaultAssistant {}

#[derive(Debug)]
pub struct Assessment {
    pub target: String,
    pub severity: Severity,
    pub notes: String,
}

#[derive(Debug)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
}
```

## Validation Steps
1. Compile and ensure DefaultAssistant builds and tests compile.
2. Add integration test using the default assistant that runs basic assess() and remediate() calls.
3. Document how to inject a stronger assistant in runtime configuration.
