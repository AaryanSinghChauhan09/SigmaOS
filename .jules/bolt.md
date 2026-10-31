# Bolt's Persistent Architectural Journal ⚡

## Rules for this Journal:
- This journal is NOT a routine changelog.
- ONLY record CRITICAL learnings that will help avoid mistakes or make better architectural decisions.
- Do NOT journal routine optimizations, standard tips, or successful work without unique insights.

---

## 2026-10-31 - Zero-Copy SAT DPLL State Representation in Sovereign Package Solvers
**Learning:** In the Sovereign Distro Package Advancements SAT DPLL solver, cloning clause vectors during recursive branch exploration caused high heap churn in memory-constrained environments. Representing clauses with indexed token slices and trail stacks eliminated heap allocations entirely.
**Action:** When building constraint solvers or graph traversal engines in `src/package/` or `src/sched/`, always use pre-allocated arena indices and trail vectors rather than deep-cloning state structures.
