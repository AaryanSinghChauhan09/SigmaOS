# Shell Diagnostics & Formatting Specification

Diagnostics formatting module (`src/shell/diagnostics/`) provides:
- **Compiler-Style Errors**: Displays source snippet, line number, column pointer (`^`), and detailed error message.
- **Command Suggestions**: Calculates string distance when command is not found and suggests similar PATH executables.
- **Redirection Error Reporting**: Clarifies permission denied, missing file, or target directory errors.
