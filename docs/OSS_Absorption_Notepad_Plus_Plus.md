# 🧩 Cleanroom Absorption: Notepad++ Text Engine

SigmaOS incorporates the high-efficiency **SigmaEdit** text engine, preserving the lightning-fast, tabbed textual manipulation pioneered by Notepad++.

---

## 🎯 Target Architecture: Notepad++

Notepad++ is legendary for its raw speed, tabbed editing, complex PCRE (Perl-Compatible Regular Expression) search and replace, custom macro playback, and multi-encoding support.

### Gaps in Legacy Notepad++:
- Tight coupling to Win32 APIs, requiring Wine emulation layers on Unix.
- Custom macro recording is strictly procedural and lacks intelligent syntax-aware refactoring.

---

## 📝 SigmaOS Sovereign Features

### 1. GPU-Accelerated Editor Context
- Renders massive files (several gigabytes) with zero scroll latency using GPU-direct buffer mappings.

### 2. Native Tree-Sitter Integration
- Instead of simple regex syntax highlighting, SigmaEdit uses microkernel-native tree-sitter lattices to understand full syntactic code trees in real-time.

### 3. AI Macro Synthesis
- Synthesizes complex textual refactoring operations via local AI from simple natural language commands (e.g., "reformat all snake_case variables to camelCase").

---

## 📊 Absorption Matrix

| Capability | Notepad++ | SigmaEdit |
|------------|-----------|-----------|
| Lightning Document Load | ✅ | ✅ |
| Regex Search & Replace | ✅ | ✅ Parallelized |
| Code Macros | ✅ (Procedural) | ✅ AI-Synthesized |
| Cross-Platform API | ❌ (Win32) | ✅ Rust Native Microkernel |
| Syntax Engine | Regex-based | ✅ Tree-Sitter AST |
| Binary Size | Minimal | Minimal (no external deps) |
