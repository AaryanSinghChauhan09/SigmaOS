#ifndef SIGMA_TYPOGRAPHY_H
#define SIGMA_TYPOGRAPHY_H

#include <sigma_types.h>

// SigmaOS Typography & Sub-pixel Engine
// Absorbs FreeType/DirectWrite paradigms for beautiful, hardware-accelerated font rendering.

// Load font vector shapes into GPU VRAM
uint32_t ui_font_load_to_vram(const char* ttf_path);

// Render anti-aliased sub-pixel typography directly onto the compositor buffer
void ui_font_render_string(uint32_t font_id, const char* text, uint32_t x, uint32_t y, uint32_t hex_color);

// Dynamically handle Input Method Edit (IME) logic for foreign character layouts
void ui_font_handle_ime_input(uint32_t keystroke_buffer);

#endif // SIGMA_TYPOGRAPHY_H

