// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
// sigma_screen_reader.h — AT-SPI2-compatible screen reader for SigmaOS
#include <userland/accessibility/sigma_a11y.h>
#include <sigma_kernel_types.h>

namespace sigma::a11y {

class ScreenReader {
public:
    bool               enabled;
    float              magnification;
    float              speech_rate;
    sigma_a11y_node_t *focused_node;

    ScreenReader();
    void enable();
    void disable();
    void set_speech_rate(float rate);
    void set_magnification(float factor);

    static void on_focus_changed(sigma_a11y_node_t *node, void *ctx);
    static void on_live_region(sigma_a11y_node_t *node, void *ctx);
    static void on_key_event(sigma_u32 keycode, sigma_u32 modifiers, void *ctx);
};

} // namespace sigma::a11y
