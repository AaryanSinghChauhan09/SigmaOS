#!/bin/bash

# ---------------------------------------------------------
# SigmaOS Module Scaffolding Generator
# Auto-generates C boilerplate for new kernel modules
# complete with Capsule hooks and Capability Registry integration.
# ---------------------------------------------------------

if [ "$#" -lt 2 ]; then
    echo "Usage: ./scaffold_module.sh <module_name> <type: driver|service|policy>"
    exit 1
fi

MOD_NAME=$1
MOD_TYPE=$2
MOD_DIR="modules/ext/plugins/${MOD_NAME}"
C_FILE="${MOD_DIR}/${MOD_NAME}.c"

echo "[*] Generating SigmaOS Module: ${MOD_NAME} (${MOD_TYPE})"

# Create directory
mkdir -p "${MOD_DIR}"

# Generate C Boilerplate
cat <<EOF > "${C_FILE}"
#include <stdint.h>
#include <stddef.h>
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Auto-Generated Module: ${MOD_NAME}
// Type: ${MOD_TYPE}
// ---------------------------------------------------------

// External Sovereign APIs
extern uint32_t capsule_register(const char*, uint32_t, void(*)(void), void(*)(void), void(*)(void), void(*)(void));
extern int cap_registry_register(uint32_t, uint32_t, uint32_t, const char*, uint8_t, uint8_t);
extern void log_event(uint32_t pid, int level, const char* message);

static uint32_t my_capsule_id = 0;

// Capsule Hooks
static void ${MOD_NAME}_init(void) {
    log_event(0, 1 /* LOG_INFO */, "[${MOD_NAME}] Initializing...");
    
    // Auto-register default capabilities
    // cap_registry_register(cap_id, owner_pid, module_id, resource_name, rights, auto_revoke)
    cap_registry_register(0x1000, 0, my_capsule_id, "${MOD_NAME}_core_mem", 0x07, 1);
}

static void ${MOD_NAME}_suspend(void) {
    log_event(0, 1, "[${MOD_NAME}] Suspending state...");
}

static void ${MOD_NAME}_resume(void) {
    log_event(0, 1, "[${MOD_NAME}] Resuming from suspension...");
}

static void ${MOD_NAME}_teardown(void) {
    log_event(0, 1, "[${MOD_NAME}] Tearing down module...");
    // Auto-revocation handles capability cleanup
}

// OS Boot Entry Point
void _start_${MOD_NAME}(void) {
    my_capsule_id = capsule_register(
        "${MOD_NAME}", 
        0x00010000, // Version 1.0.0
        &${MOD_NAME}_init, 
        &${MOD_NAME}_suspend, 
        &${MOD_NAME}_resume, 
        &${MOD_NAME}_teardown
    );
}
EOF

echo "[+] Boilerplate written to ${C_FILE}"
echo "[*] Scaffold Complete. Ensure you call _start_${MOD_NAME}() in kernel_main.c"
