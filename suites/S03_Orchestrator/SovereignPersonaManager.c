/**
 * @file SovereignPersonaManager.c
 * @brief SigmaOS Dynamic System Personalities
 * 
 * Allows the kernel to hot-swap system policies based on declarative personas.
 * Personas define scheduling priority, network visibility, and UI aesthetics.
 */

#include "sigma_kernel_types.h"

typedef struct {
    char name[32];
    u32 sched_quantum;
    u8  network_enabled;
    u8  gui_glassmorphism;
    u32 memory_quota_mb;
} SigmaPersona;

static SigmaPersona g_current_persona;

void persona_apply_focus_mode() {
    sigma_strcpy(g_current_persona.name, "Focus Mode");
    g_current_persona.sched_quantum = 100; // Longer quantum for deep work
    g_current_persona.network_enabled = 0; // Block distractions
    g_current_persona.gui_glassmorphism = 1;
    g_current_persona.memory_quota_mb = 1024;
    kprintf("[PERSONA] Switched to Focus Mode. All neural distrations blocked.\n");
}

void persona_apply_dev_mode() {
    sigma_strcpy(g_current_persona.name, "Developer Mode");
    g_current_persona.sched_quantum = 10; // Rapid response for compilation
    g_current_persona.network_enabled = 1;
    g_current_persona.gui_glassmorphism = 0; // Performance first
    g_current_persona.memory_quota_mb = 4096;
    kprintf("[PERSONA] Switched to Developer Mode. Maximum throughput enabled.\n");
}

void persona_init(const char* persona_name) {
    if (sigma_streq(persona_name, "focus")) {
        persona_apply_focus_mode();
    } else if (sigma_streq(persona_name, "dev")) {
        persona_apply_dev_mode();
    } else {
        kprintf("[PERSONA] Unknown persona '%s'. Defaulting to Standard.\n", persona_name);
    }
}
