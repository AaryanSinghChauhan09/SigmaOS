# Interactive Shell & Line Editing Specification

The interactive shell module (`src/shell/interactive/`) provides:
- **Prompt Rendering**: Customizable prompt strings with status code reflection.
- **Multiline Quote Detection**: Automatically prompts for continuation (`> `) when quotes or blocks are unclosed.
- **Signal Handling**: Handles `SIGINT` (Ctrl+C) to cancel input line without exiting, and `EOF` (Ctrl+D) to exit cleanly.
- **Tab Completion**: Candidate generation for path completion and PATH commands.
