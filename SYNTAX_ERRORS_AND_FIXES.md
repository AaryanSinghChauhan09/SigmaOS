# Suggested Syntax & Compilation Fixes for SigmaOS Workspace

During workspace compilation, several compile-time syntax, type, and trait errors were identified in various modules. Below is a comprehensive report documenting each issue and proposing exact code corrections to resolve them.

***

## 1. Missing `Ord` / `PartialOrd` Derivations for BTreeMap Keys

### File: `src/accessibility/framework.rs` (Line 130)

*   **Error:** The trait bound `accessibility::framework::AccessibilityFeature: Ord` is not satisfied.
*   **Root Cause:** `AccessibilityFeature` is used as a key in `BTreeMap<AccessibilityFeature, AccessibilitySetting>`, which requires the key type to implement `Ord` and `PartialOrd`.
*   **Suggested Fix:**
    ```rust
    <<<<<<< SEARCH
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AccessibilityFeature {
    =======
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum AccessibilityFeature {
    >>>>>>> REPLACE
    ```

### File: `src/distro/certification.rs` (Line 56)

*   **Error:** The trait bound `ComponentType: Ord` is not satisfied.
*   **Root Cause:** Used as a key in `BTreeMap<ComponentType, CertificationStatus>` on line 78.
*   **Suggested Fix:**
    ```rust
    <<<<<<< SEARCH
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ComponentType {
    =======
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ComponentType {
    >>>>>>> REPLACE
    ```

### File: `src/distro/compat_layers.rs` (Line 95)

*   **Error:** The trait bound `GdiObjectType: Ord` is not satisfied.
*   **Root Cause:** Used as a key in `BTreeMap<GdiObjectType, u32>` on line 104.
*   **Suggested Fix:**
    ```rust
    <<<<<<< SEARCH
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GdiObjectType {
    =======
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum GdiObjectType {
    >>>>>>> REPLACE
    ```

### File: `src/distro/developer.rs` (Line 23)

*   **Error:** The trait bound `TargetArch: Ord` is not satisfied.
*   **Root Cause:** Used as a key in `BTreeMap<TargetArch, String>` on line 145.
*   **Suggested Fix:**
    ```rust
    <<<<<<< SEARCH
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TargetArch {
    =======
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum TargetArch {
    >>>>>>> REPLACE
    ```

***

## 2. Generic `Ord` Constraint Missing on HashSet

### File: `src/klib/hashset.rs` (Line 11)

*   **Error:** The trait bound `T: Ord` is not satisfied inside the `HashSet` struct definition.
*   **Root Cause:** `HashSet<T>` wraps a `BTreeMap<T, ()>`, but its structural declaration does not constrain `T: Ord`.
*   **Suggested Fix:**
    ```rust
    <<<<<<< SEARCH
    pub struct HashSet<T> {
        map: BTreeMap<T, ()>,
    }
    =======
    pub struct HashSet<T>
    where
        T: Ord,
    {
        map: BTreeMap<T, ()>,
    }
    >>>>>>> REPLACE
    ```

***

## 3. Trait Method Signature Mismatch in Compliance Auditor

### File: `src/security/audit.rs` (Lines 157 & 175)

*   **Error:** Method `check_compliance` has an incompatible type for trait.
*   **Root Cause:** The `ComplianceAuditor` trait defines `check_compliance(&self, event: &dyn AuditEvent) -> bool;` returning a pure boolean, but the implementation on `SimpleAuditPolicy` returns `Result<bool, AuditError>`.
*   **Suggested Fix:**
    Update the method signature in the trait implementation to match (returning a boolean and handling/logging internal errors), or modify the trait definition to return a `Result`.
    *   **Option A (Returning bool to match trait):**
        ```rust
        <<<<<<< SEARCH
            fn check_compliance(&self, event: &dyn AuditEvent) -> Result<bool, AuditError> {
                // ...
                Ok(true)
            }
        =======
            fn check_compliance(&self, event: &dyn AuditEvent) -> bool {
                // ...
                true
            }
        >>>>>>> REPLACE
        ```

***

## 4. Undeclared / Missing Imports in Network Analyzer

### File: `src/network/analyzer.rs` (Line 294)

*   **Error:** Cannot find type `AlertType` and `AlertSeverity` in this scope.
*   **Root Cause:** `AlertType` and `AlertSeverity` are referenced on lines 294 and 295 but are not imported into the file.
*   **Suggested Fix:** Add the missing import statement at the top of the file:
    ```rust
    use crate::security::alert::{AlertType, AlertSeverity};
    ```
