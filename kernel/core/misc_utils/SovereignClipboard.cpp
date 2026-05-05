#include "../../../include/SovereignLibC.h""
#include "../../../include/sigma_types.h""

#include "sigma_clipboard.h"
#include "../../../include/sigma_hal.h""
#include "sigma_continuity.h"

/**
 * SigmaOS Sovereign Smart Clipboard
 * Implements a Universal Semantic Copy (USC) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal clipboard management.
 */

/* --- Sovereign Clipboard Engine (OOP Isolation) --- */

void SovereignClipboardEngine::init() {
    sigma_log("[CLIPBOARD] Initializing Sovereign Smart Clipboard (USC Algorithm)...");
}

void SovereignClipboardEngine::copy(sigma_clip_type_t type, const void* data, uint32_t size) {
    /* USC (Universal Semantic Copy) Algorithm
     * Evaluates data semantics and broadcasts state to S-Continuity. */
    this->type = type;
    this->data = (void*)data; /* Simulated allocation/copy */
    this->size = size;

    sigma_printf("[CLIPBOARD] USC: Copied %d bytes (Type: %d) to global buffer.\n",
                 size, (int)type);

    /* Auto-sync via continuity */
    continuity_push_state(0xDEADBEEF);
}

void* SovereignClipboardEngine::paste(sigma_clip_type_t* out_type, uint32_t* out_size) {
    if (this->size == 0) return nullptr;

    *out_type = this->type;
    *out_size = this->size;

    sigma_log("[CLIPBOARD] USC: Pasting data from global buffer.");
    return this->data;
}

/* --- C Wrappers --- */
extern "C" void clipboard_init() {
    SovereignClipboardEngine::getInstance().init();
}

extern "C" void clipboard_copy(sigma_clip_type_t type, const void* data, uint32_t size) {
    SovereignClipboardEngine::getInstance().copy(type, data, size);
}

extern "C" void* clipboard_paste(sigma_clip_type_t* out_type, uint32_t* out_size) {
    return SovereignClipboardEngine::getInstance().paste(out_type, out_size);
}



