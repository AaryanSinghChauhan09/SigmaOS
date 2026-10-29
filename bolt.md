# Bolt's Journal — Critical Performance Learnings

## 2026-04-12 - Bash Script Termination Syntax & Test Runner Overhead
**Learning:** Shell script syntax errors (such as merged words `testfi` without newlines) in test harness runners cause silent test suite abortion prior to executing subsequent subsystem test suites.
**Action:** Always run `bash -n` on shell scripts after editing, and ensure clean line breaks after test execution commands in test harnesses.

## 2026-04-12 - SAT DPLL Constraint Solver Allocation Bottlenecks
**Learning:** Re-allocating dynamic `Vec<Clause>` and heap-allocating string keys inside the SAT DPLL loop during cross-distro package dependency resolution reduces package transaction throughput by over 60%.
**Action:** Use integer symbol tables (`u32` identifier indexing) and thread-local arena allocators for SAT DPLL solver state.
