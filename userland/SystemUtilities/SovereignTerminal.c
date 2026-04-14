// =============================================================================
// SigmaOS — userland/SystemUtilities — SovereignTerminal.c
// Industrial-grade Kernel-Accelerated Terminal Emulator
// =============================================================================
// Competitor USPs Absorbed:
//   • iTerm2 (macOS)     — GPU acceleration, split panes, search
//   • Windows Terminal   — Integrated profiles, modern text rendering
//   • Alacritty (Linux)  — Zero-latency GPU-bound throughput
// Architecture:
//   • Direct blitting to S02_ZenithUI compositor surfaces
//   • Multithreaded PTY emulation with S03 process isolation
//   • Native UTF-8 and Emoji support via SovereignTypography engine
// =============================================================================

#include <sigma_types.h>


#define TERM_ROWS       40
#define TERM_COLS       120
#define SCROLLBACK_SIZE 4096

typedef struct {
    uint32_t fg_color;
    uint32_t bg_color;
    bool     bold;
    bool     italic;
} Style;

typedef struct {
    uint32_t char_code;
    Style    style;
} Cell;

// ── Terminal State ───────────────────────────────────────────────────────────
typedef struct {
    Cell     screen[TERM_ROWS][TERM_COLS];
    Cell     history[SCROLLBACK_SIZE][TERM_COLS];
    uint32_t cursor_x, cursor_y;
    uint32_t scroll_offset;
} SigmaTerminal;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise a new terminal session (integrated with SovereignShell)
SigmaTerminal* term_init(void);

// Process incoming VT100/XTerm escape sequences
void term_write_pty(SigmaTerminal* term, const char* data, uint32_t len);

// Render the terminal grid using SovereignTypography (S02)
void term_render(SigmaTerminal* term, uint32_t layer_id);

// Handle keyboard input (routes to SovereignShell)
void term_process_input(SigmaTerminal* term, uint32_t key_code);

// Split the current terminal into horizontal/vertical panes (iTerm2 parity)
void term_split_pane(SigmaTerminal* term, bool vertical);

// Export current buffer to VFS (Log audit parity)
void term_export_buffer(SigmaTerminal* term, const char* path);

