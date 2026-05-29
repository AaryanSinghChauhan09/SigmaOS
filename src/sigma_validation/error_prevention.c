/*
 * =============================================================================
 * Σ SIGMAOS: GLOBAL ERROR PREVENTION SYSTEM
 * =============================================================================
 * Implements a global error registry to catch and prevent cascading failures.
 * =============================================================================
 */

#include <stdint.h>
#include <stddef.h>

#define SIGMA_MAX_ERRORS 1000

/* Severity Levels */
#define ERR_CRITICAL 1
#define ERR_HIGH     2
#define ERR_MEDIUM   3
#define ERR_LOW      4

typedef struct {
    uint32_t error_id;
    const char* component;
    const char* description;
    uint32_t line_number;
    uint32_t severity;
} SigmaErrorEvent;

/* Global error registry */
static SigmaErrorEvent error_log[SIGMA_MAX_ERRORS];
static size_t error_count = 0;

/* Stub for kernel panic */
extern void system_panic(const char* fmt, ...);
/* Stub for printing */
extern void kprintf(const char* fmt, ...);

/* Log and prevent errors before they cascade */
int sigma_error_handler(const char* component, const char* msg, uint32_t line, uint32_t code) {
    if (error_count >= SIGMA_MAX_ERRORS) {
        /* Circular buffer wrap-around */
        error_count = 0;
    }
    
    error_log[error_count].error_id = code;
    error_log[error_count].component = component;
    error_log[error_count].description = msg;
    error_log[error_count].line_number = line;
    
    // Determine severity based on error code conventions (e.g. < 1000 is CRITICAL)
    if (code < 1000) {
        error_log[error_count].severity = ERR_CRITICAL;
    } else if (code < 5000) {
        error_log[error_count].severity = ERR_HIGH;
    } else {
        error_log[error_count].severity = ERR_MEDIUM;
    }
    
    error_count++;
    
    /* Prevent continuation if CRITICAL */
    if (code < 1000) {
        /* If system_panic exists, call it. For compilation, we mock the panic. */
        // system_panic("CRITICAL ERROR IN %s:%u - %s", component, line, msg);
        while(1) {} /* Halt execution */
    }
    
    return -1;
}

/* Print current diagnostic state */
void sigma_print_error_log() {
    kprintf("=== SigmaOS Error Log ===\n");
    for (size_t i = 0; i < error_count; i++) {
        kprintf("[%s:%u] Code: %u - %s\n", 
               error_log[i].component, 
               error_log[i].line_number, 
               error_log[i].error_id, 
               error_log[i].description);
    }
}
