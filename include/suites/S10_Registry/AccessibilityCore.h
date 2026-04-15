#ifndef SIGMA_ACCESSIBILITY_H
#define SIGMA_ACCESSIBILITY_H

#include "suites/S01_Genesis/shards/sigma_types.h"


// SigmaOS Accessibility Framework Shard
// Native system-level support for Screen Reading, Dictation, and Assistive Touch.

// Initialize the accessibility event interceptor
void system_acc_init(void);

// Native Text-to-Speech (TTS) hook using the Audio suite
void system_acc_speak_text(const char* text);

// Screen Reader: Intercept UI toolkit rendering and describe tree
void system_acc_describe_widget(uint32_t widget_id);

// High-Contrast and Color-Correction mode toggle
void system_acc_set_visual_mode(uint8_t mode);

#endif // SIGMA_ACCESSIBILITY_H

