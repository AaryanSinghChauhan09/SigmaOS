# SigmaOS HTML Dependency Reduction & Text-First Architecture Guide for AI Agents

This guide provides technical specifications, text-first user interface alternatives, Markdown/ANSI terminal documentation fallbacks, HTML escaping sanitization rules, and progressive enhancement principles for AI agents working in SigmaOS.

---

## 1. Zero-Dependency Text-First Principles

SigmaOS prioritizes sovereign text-based and terminal interfaces over heavy browser runtime or HTML dependencies (`src/docs/mod.rs`, `src/tools/olivetin.rs`, `src/shell/terminal_emulator.rs`):

* **Markdown & ASCII Documentation Preference:**
  Documentation generation engines (`src/docs/mod.rs`) MUST prioritize native `DocFormat::Markdown` and `DocFormat::AsciiDoc` over `DocFormat::Html`.
* **Zero-JS Progressive Enhancement:**
  Web interfaces (such as `web_ui/index.html` or `index.html`) MUST operate with full accessibility and zero-JS fallbacks, allowing basic command navigation without JavaScript or dynamic HTML rendering.
* **XSS Prevention & HTML Escaping (`escape_html` in `src/docs/mod.rs`):**
  When HTML string rendering is strictly necessary, AI agents MUST sanitize all dynamic input strings using `escape_html` to convert `<`, `>`, `&`, `"`, and `'` into neutralized HTML entities before output stream construction.

---

## 2. Guidelines for Reducing HTML Usage

1. **Terminal UI & OmniShell Alternatives:**
   Prefer CLI commands, ANSI escape codes, or text-based dashboards over HTML web panels when exposing system management utilities.
2. **Plain Text Clipboard Formats:**
   Clipboard engines (`src/tools/powertoys.rs` & `src/desktop/clipboard.rs`) MUST offer plain text fallback stripping (`strip_rich_formatting`) for HTML clipboard data.
3. **Escaping HTML Characters:**
   Do NOT construct raw unescaped HTML strings from user or system input. Always filter dynamic strings via `escape_html`.

---

## 3. Checklist for AI Agents Reducing HTML Dependency

1. **Prefer Text/Markdown Outputs:** Use Markdown or terminal formatting instead of HTML templates where possible.
2. **Test HTML Escaping Routines:**
   Run documentation generator and HTML escaping tests:
   ```bash
   cargo test --lib -- docs::tests::test_html_escaping_xss_prevention
   ./run_sigma_tests.sh
   ```
