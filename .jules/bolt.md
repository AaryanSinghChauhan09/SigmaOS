# Bolt Agent Journal ⚡

## 2025-05-18 - Shell Script Tool Dependency Fallback Optimization
**Learning:** Shell release gate scripts that depend on external arithmetic tools like `bc` fail silently or produce syntax errors when `bc` is missing in containerized or minimal environments. Using native Bash integer/awk fallback calculation eliminates external process dependency failures and reduces gate evaluation execution latency.
**Action:** Always prefer native POSIX shell arithmetic or fallback checks for build/release gates.
