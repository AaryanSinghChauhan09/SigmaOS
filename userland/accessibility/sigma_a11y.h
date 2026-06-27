// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_a11y.h — Accessibility framework (screen reader, magnifier, contrast)
 *
 * Inspired by AT-SPI2 (Linux), UIAutomation (Windows), Accessibility API (macOS).
 *
 * SigmaOS accessibility stack:
 *   App (uses sigma_a11y_node_t API)
 *       → sigma-a11yd daemon (aggregates tree)
 *           → AT-SPI2 bridge (for Orca screen reader compat)
 *           → sigma-orca (built-in TTS screen reader, espeak-ng backend)
 *           → sigma-magnify (kernel compositor magnification)
 *           → sigma-contrast (per-output color matrix)
 *
 * Compliance target: WCAG 2.2 AA, ARIA 1.2, Section 508
 */

#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Accessible role ─────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_A11Y_ROLE_UNKNOWN     = 0,
    SIGMA_A11Y_ROLE_WINDOW      = 1,
    SIGMA_A11Y_ROLE_BUTTON      = 2,
    SIGMA_A11Y_ROLE_CHECKBOX    = 3,
    SIGMA_A11Y_ROLE_COMBOBOX    = 4,
    SIGMA_A11Y_ROLE_DIALOG      = 5,
    SIGMA_A11Y_ROLE_LABEL       = 6,
    SIGMA_A11Y_ROLE_LINK        = 7,
    SIGMA_A11Y_ROLE_LIST        = 8,
    SIGMA_A11Y_ROLE_LIST_ITEM   = 9,
    SIGMA_A11Y_ROLE_MENU        = 10,
    SIGMA_A11Y_ROLE_MENU_ITEM   = 11,
    SIGMA_A11Y_ROLE_PROGRESS    = 12,
    SIGMA_A11Y_ROLE_SLIDER      = 13,
    SIGMA_A11Y_ROLE_TABLE       = 14,
    SIGMA_A11Y_ROLE_TEXT        = 15,
    SIGMA_A11Y_ROLE_TEXT_INPUT  = 16,
    SIGMA_A11Y_ROLE_TOOLBAR     = 17,
    SIGMA_A11Y_ROLE_TREE        = 18,
    SIGMA_A11Y_ROLE_TREE_ITEM   = 19,
    SIGMA_A11Y_ROLE_IMAGE       = 20,
    SIGMA_A11Y_ROLE_ALERT       = 21,
    SIGMA_A11Y_ROLE_DOCUMENT    = 22,
} sigma_a11y_role_t;

/* ── Accessible state flags ──────────────────────────────────────────────── */
#define SIGMA_A11Y_STATE_ENABLED    (1 << 0)
#define SIGMA_A11Y_STATE_FOCUSED    (1 << 1)
#define SIGMA_A11Y_STATE_SELECTED   (1 << 2)
#define SIGMA_A11Y_STATE_CHECKED    (1 << 3)
#define SIGMA_A11Y_STATE_EXPANDED   (1 << 4)
#define SIGMA_A11Y_STATE_VISIBLE    (1 << 5)
#define SIGMA_A11Y_STATE_EDITABLE   (1 << 6)
#define SIGMA_A11Y_STATE_BUSY       (1 << 7)

/* ── Accessible node ─────────────────────────────────────────────────────── */
typedef struct sigma_a11y_node sigma_a11y_node_t;

typedef struct {
    sigma_a11y_role_t role;
    sigma_u32         state;        /* bitmask of SIGMA_A11Y_STATE_* flags    */
    char              name[128];    /* accessible name (button label, etc.)   */
    char              description[256];
    char              value[256];   /* current value for inputs/sliders       */
    sigma_u32         x, y, w, h;  /* bounding box in screen coords          */
    int               child_count;
} sigma_a11y_node_info_t;

/* ── App-side API ────────────────────────────────────────────────────────── */

/* Create an accessible node in the global accessibility tree. */
sigma_a11y_node_t* sigma_a11y_node_create(sigma_a11y_role_t role,
                                           const char* name);

/* Update node properties — triggers AT event for screen reader. */
int sigma_a11y_node_update(sigma_a11y_node_t* node,
                            const sigma_a11y_node_info_t* info);

/* Append child node. */
int sigma_a11y_node_add_child(sigma_a11y_node_t* parent,
                               sigma_a11y_node_t* child);

/* Signal that an event occurred (focus change, value change, etc.). */
typedef enum {
    SIGMA_A11Y_EVENT_FOCUS       = 1,
    SIGMA_A11Y_EVENT_VALUE_CHANGED = 2,
    SIGMA_A11Y_EVENT_STATE_CHANGED = 3,
    SIGMA_A11Y_EVENT_ALERT       = 4,
    SIGMA_A11Y_EVENT_LIVE_REGION = 5,  /* ARIA live region update             */
} sigma_a11y_event_t;

void sigma_a11y_emit(sigma_a11y_node_t* node, sigma_a11y_event_t event);

void sigma_a11y_node_destroy(sigma_a11y_node_t* node);

/* ── Screen reader / TTS ─────────────────────────────────────────────────── */

/* Speak text immediately (interrupts current speech). */
void sigma_a11y_speak(const char* text);

/* Queue text to be spoken after current speech finishes. */
void sigma_a11y_speak_queue(const char* text);

/* Set speech rate (0.5 = half speed, 1.0 = normal, 2.0 = double). */
void sigma_a11y_set_speech_rate(float rate);

/* ── Magnifier ───────────────────────────────────────────────────────────── */

void sigma_a11y_magnify_set(float factor);   /* 1.0 = off, up to 8.0        */
void sigma_a11y_magnify_follow_focus(bool enable);

/* ── High contrast / colour filters ─────────────────────────────────────── */

typedef enum {
    SIGMA_A11Y_CONTRAST_NORMAL      = 0,
    SIGMA_A11Y_CONTRAST_HIGH        = 1,  /* inverted lightness              */
    SIGMA_A11Y_CONTRAST_GRAYSCALE   = 2,
    SIGMA_A11Y_CONTRAST_PROTANOPIA  = 3,  /* red-blind filter                */
    SIGMA_A11Y_CONTRAST_DEUTERANOPIA= 4,  /* green-blind filter              */
    SIGMA_A11Y_CONTRAST_TRITANOPIA  = 5,  /* blue-blind filter               */
} sigma_a11y_contrast_t;

void sigma_a11y_set_contrast(sigma_a11y_contrast_t mode);

/* ── Keyboard navigation ─────────────────────────────────────────────────── */

/* Register a global shortcut for accessibility features. */
void sigma_a11y_register_shortcut(sigma_u32 keycode, sigma_u32 modifiers,
                                   void (*callback)(void*), void* userdata);
