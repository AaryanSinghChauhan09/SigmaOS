#ifndef SIGMA_NOTIFICATION_ENGINE_H
#define SIGMA_NOTIFICATION_ENGINE_H

#include <sigma_types.h>


// SigmaOS Notification & Desktop Environment Module
// Natively integrated with ZenithUI to handle non-intrusive alerts and Universal Clipboard

// Push an interactive alert to the compositor rendering layer
void ui_notify_push(const char* title, const char* message, uint8_t urgency_level);

// Secure cross-clipboard manager handling memory-safe struct transfers
void* ui_clipboard_yank(void);
void ui_clipboard_paste(void* payload, uint32_t payload_size);

// Change the dynamic Zenith UI Theme Engine mathematically across all contexts
void ui_theme_apply_engine(const char* theme_json_path);

#endif // SIGMA_NOTIFICATION_ENGINE_H

