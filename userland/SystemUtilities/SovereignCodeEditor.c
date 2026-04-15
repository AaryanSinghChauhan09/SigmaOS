// =============================================================================
// SigmaOS — userland/SystemUtilities — SovereignCodeEditor.c
// Native High-Performance Code Editor (Sigma Studio Core)
// =============================================================================
// Competitor USPs Absorbed:
//   • VS Code          — Extension model, rich UI, workspace management
//   • Sublime Text     — Near-instant startup, multi-select, hardware acceleration
//   • Vim / Emacs      — Modal editing, extreme keyboard productivity
// Architecture:
//   • GPU-accelerated texture-based rendering (No DOM overhead)
//   • Asynchronous Language Server Protocol (LSP) bridge
//   • Integrated SovereignVersionControl (S10) and sigma-build (Tools)
// =============================================================================

#include "suites/S01_Genesis/shards/sigma_types.h"


#define MAX_OPEN_FILES      16
#define MAX_LINE_LEN        2048

typedef struct {
    char*    buffer;
    uint32_t line_count;
    uint32_t cursor_line, cursor_col;
    bool     is_dirty;
    char     filename[256];
} CodeBuffer;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Sigma Studio Core
void editor_init(void);

// Load a file from S06 Storage into an editor buffer
CodeBuffer* editor_open_file(const char* path);

// Perform atomic save (uses SovereignTimeVault for safe shadow-writing)
void editor_save_file(CodeBuffer* buffer);

// Trigger syntax highlighting pass (GPU-accelerated via S04 GraphicsBridge)
void editor_render_syntax(CodeBuffer* buffer, uint32_t target_surface);

// Map modal hotkeys (Vim/Emacs mode toggle)
void editor_set_mode(uint8_t mode);

// Integrated Build: Trigger sigma-build and report errors on the UI
void editor_trigger_build(CodeBuffer* buffer);

// Integrated Search: Unified Spotlight (S02) query within the editor
void editor_global_search(const char* query);

