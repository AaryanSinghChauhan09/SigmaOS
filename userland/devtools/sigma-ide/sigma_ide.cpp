// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_ide.cpp — Native AI-assisted IDE for SigmaOS
//
// This is the killer differentiating app: a native IDE that only runs on
// SigmaOS, tightly integrated with sigma-bus IPC, sigma_trace (strace),
// sigma-healthd, and the AI inference engine (port 17392).
//
// Components:
//   sigma_ide.cpp           — main editor window (Wayland/sigma_display)
//   sigma_lsp_bridge.cpp    — LSP client (clangd, rust-analyzer, gopls)
//   sigma_ai_debugger.cpp   — AI crash explainer + fix suggester
//   sigma_build.cpp         — integrated CMake build runner
//   sigma_trace.cpp         — embedded syscall tracer (strace-equivalent)
//   sigma_top.cpp           — process monitor in IDE sidebar
//
// Inspired by:
//   • Helix editor (Rust, modal, tree-sitter)
//   • Zed (modern GPU-rendered editor)
//   • VS Code language server protocol (LSP)
//   • JetBrains Fleet (distributed AI IDE)
//   • GitHub Copilot (AI completion)

#include "sigma_ide.h"
#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdbool.h>

// ── Editor buffer (gap buffer for O(1) insert/delete) ─────────────────────────

#define GAP_SIZE_INIT  256
#define MAX_BUFFERS    32

typedef struct sigma_buffer {
    char    *data;         // gap buffer storage
    size_t   gap_start;    // cursor position (start of gap)
    size_t   gap_end;      // end of gap
    size_t   capacity;     // total buffer size (gap + content)
    char     path[512];    // file path
    bool     modified;
    uint32_t language;     // hash of language ID ("cpp", "go", "rust", ...)
    uint32_t lsp_doc_version;
} sigma_buffer_t;

static sigma_buffer_t g_buffers[MAX_BUFFERS];
static uint32_t       g_nbuffers = 0;
static uint32_t       g_active   = 0;

// ── Gap buffer operations ─────────────────────────────────────────────────────

extern void *sigma_kmalloc(size_t);
extern void  sigma_kfree(void *);

static int buf_init(sigma_buffer_t *b, const char *path, size_t initial_cap) {
    if (!initial_cap) initial_cap = GAP_SIZE_INIT;
    b->data      = (char *)sigma_kmalloc(initial_cap);
    if (!b->data) return -1;
    b->gap_start = 0;
    b->gap_end   = initial_cap;
    b->capacity  = initial_cap;
    b->modified  = false;
    b->lsp_doc_version = 0;
    size_t plen = strlen(path);
    if (plen >= sizeof(b->path)) plen = sizeof(b->path) - 1;
    memcpy(b->path, path, plen);
    b->path[plen] = '\0';
    return 0;
}

// Insert character at current cursor position
static void buf_insert(sigma_buffer_t *b, char c) {
    if (b->gap_start == b->gap_end) {
        // Gap exhausted — grow (double capacity)
        size_t new_cap = b->capacity * 2 + GAP_SIZE_INIT;
        char *new_data = (char *)sigma_kmalloc(new_cap);
        if (!new_data) return;
        // Copy pre-gap
        memcpy(new_data, b->data, b->gap_start);
        // Copy post-gap
        size_t post = b->capacity - b->gap_end;
        memcpy(new_data + new_cap - post, b->data + b->gap_end, post);
        sigma_kfree(b->data);
        b->data     = new_data;
        b->gap_end  = new_cap - post;
        b->capacity = new_cap;
    }
    b->data[b->gap_start++] = c;
    b->modified = true;
}

// Delete character before cursor (backspace)
static void buf_delete_before(sigma_buffer_t *b) {
    if (b->gap_start > 0) {
        b->gap_start--;
        b->modified = true;
    }
}

// Move cursor right
static void buf_move_right(sigma_buffer_t *b) {
    if (b->gap_end < b->capacity) {
        b->data[b->gap_start++] = b->data[b->gap_end++];
    }
}

// Move cursor left
static void buf_move_left(sigma_buffer_t *b) {
    if (b->gap_start > 0) {
        b->data[--b->gap_end] = b->data[--b->gap_start];
    }
}

// ── LSP bridge (sends didChange, completion, hover) ───────────────────────────

static void lsp_send_did_change(sigma_buffer_t *b) {
    // In production: serialise the buffer content and POST to the LSP server
    // via sigma-bus IPC (channel "lsp.clangd" or "lsp.gopls").
    b->lsp_doc_version++;
    // sigma_bus_send_typed(LSP_CHANNEL, "didChange", b->path, b->lsp_doc_version,
    //                      buf_to_string(b), buf_content_len(b));
}

typedef struct lsp_completion {
    char label[128];
    char detail[256];
    char insert_text[256];
    uint8_t kind;   // 1=text 2=method 3=function etc.
} lsp_completion_t;

int sigma_ide_complete(uint32_t buf_id, uint32_t line, uint32_t col,
                       lsp_completion_t *out, uint32_t max) {
    if (buf_id >= g_nbuffers || !out || max == 0) return 0;
    sigma_buffer_t *b = &g_buffers[buf_id];
    lsp_send_did_change(b);

    // Send LSP textDocument/completion request via sigma-bus
    // and block on response (up to 500ms timeout).
    // Returns number of completions filled in @out.
    // (Real implementation calls sigma_bus_call_sync("lsp", "complete", ...))
    (void)line; (void)col; (void)b;
    return 0;  // placeholder — LSP bridge in sigma_lsp_bridge.cpp
}

// ── AI debugger ───────────────────────────────────────────────────────────────
// When a process crashes (SIGSEGV / SIGABRT), sigma-crash sends the minidump
// to this function, which forwards to the AI engine on port 17392 for
// natural-language explanation + suggested patch.

typedef struct ai_debug_result {
    char explanation[1024];   // "Your null pointer at line 42 is because..."
    char suggested_fix[2048]; // unified diff
} ai_debug_result_t;

int sigma_ide_explain_crash(const uint8_t *minidump, size_t len,
                            ai_debug_result_t *out) {
    if (!minidump || !out || len == 0) return -1;
    // POST minidump to AI engine: http://127.0.0.1:17392/v1/debug
    // Parse JSON response: {"explanation": "...", "fix": "..."}
    // For now: stub returning placeholder
    strncpy(out->explanation,
            "AI analysis: forward to sigma-ai-engine (port 17392)",
            sizeof(out->explanation) - 1);
    strncpy(out->suggested_fix, "", sizeof(out->suggested_fix) - 1);
    (void)len;
    return 0;
}

// ── Build runner ──────────────────────────────────────────────────────────────

typedef struct build_result {
    int     exit_code;
    char    output[8192];
    uint32_t errors;
    uint32_t warnings;
} build_result_t;

int sigma_ide_build(const char *build_dir, const char *target,
                    build_result_t *out) {
    if (!build_dir || !out) return -1;
    // Spawn: cmake --build <build_dir> --target <target>
    // Capture stdout/stderr via sigma_spawn_capture (from sigma_process.cpp)
    // Parse GCC/Clang diagnostics to populate out->errors, out->warnings
    (void)target;
    out->exit_code = 0;
    strncpy(out->output, "Build system integration pending (sigma_spawn_capture)",
            sizeof(out->output) - 1);
    return 0;
}

// ── Open buffer ───────────────────────────────────────────────────────────────

int sigma_ide_open(const char *path) {
    if (g_nbuffers >= MAX_BUFFERS) return -1;
    sigma_buffer_t *b = &g_buffers[g_nbuffers];
    if (buf_init(b, path, 4096) != 0) return -1;

    // Read file content into buffer
    // (sigma_vfs_read not shown — uses sigma_scheme "file:")
    return (int)g_nbuffers++;
}

// ── Main editor event loop ────────────────────────────────────────────────────

void sigma_ide_run(void) {
    // Opens the IDE window via sigma_display_protocol.h,
    // enters a Wayland-compatible event loop handling:
    //   • keyboard input → buf_insert / buf_delete_before / buf_move_*
    //   • mouse click → cursor placement, selection
    //   • Ctrl+Space → sigma_ide_complete()
    //   • F12 → sigma_ide_goto_definition()
    //   • Ctrl+B → sigma_ide_build()
    //   • Ctrl+Shift+D → sigma_ide_explain_crash()
    //   • Ctrl+S → sigma_ide_save()
    //
    // Implementation dispatches through sigma_display_event_loop()
    // defined in userland/display/sigma_display_protocol.h
    sigma_ide_open("/sigma/home/README.md");  // open default file
    // sigma_display_event_loop(sigma_ide_handle_event);
}
