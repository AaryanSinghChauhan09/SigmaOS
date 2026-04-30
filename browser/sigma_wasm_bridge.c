/*
 * =============================================================================
 * ÃŽÂ£ SIGMAOS BROWSER-BRIDGE (v1.0)
 * =============================================================================
 * Algorithm: WASM-Inter-Op Sharding
 * Principles:
 *   - Zero-dependency kernel logic compiled to WASM.
 *   - JS-Bridge for Terminal I/O and Canvas rendering.
 *   - LocalStorage for sovereign state persistence.
 * =============================================================================
 */
#include "sigma_kernel_types.h"

/* Emscripten-style exports */
#ifdef __EMSCRIPTEN__
#include <emscripten.h>
#else
#define EMSCRIPTEN_KEEPALIVE
#endif

extern void kernel_main(void);
extern void omnishell_exec(const char* line);

EMSCRIPTEN_KEEPALIVE
void sigma_browser_init(void) {
    /* Initialize kernel in browser context */
    kernel_main();
}

EMSCRIPTEN_KEEPALIVE
void sigma_browser_shell_input(const char* line) {
    /* Pass browser terminal input to Omni-Shell */
    omnishell_exec(line);
}

/* Browser-side kprintf redirection */
void kprintf(const char* fmt, ...) {
    /* Redirect to JS console or terminal buffer */
    // EM_ASM({ console.log(UTF8ToString($0)); }, buffer);
}
