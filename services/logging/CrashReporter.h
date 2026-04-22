#ifndef CRASH_REPORTER_H
#define CRASH_REPORTER_H

#include <stdint.h>
#include <stdbool.h>

// Definitions for the processor's saved state during a hardware fault / kernel panic
typedef struct {
    uint64_t rax, rbx, rcx, rdx, rsi, rdi, rbp, rsp; // Core registers
    uint64_t rip;                                    // Instruction pointer at crash
    uint64_t error_code;                             // IDT fault code (e.g., Page Fault)
    uint64_t cr2;                                    // Memory address causing the fault
} CpuRegisters_t;

typedef struct {
    char panic_message[256];
    uint64_t timestamp;
    CpuRegisters_t registers;
    char stack_trace[1024];       // Unwound stack frame data
    bool telemetry_opt_in;        // Enterprise compliance check
} CrashDumpPayload_t;

// --- API ---

/**
 * Hooked directly into the Interrupt Descriptor Table (IDT).
 * Captures raw CPU state the microsecond a fatal exception occurs.
 */
void crash_catch_hardware_fault(CpuRegisters_t* state);

/**
 * Formats the raw CPU data into a readable CrashDumpPayload_t.
 */
void crash_generate_dump(const char* message, CpuRegisters_t* state, CrashDumpPayload_t* out_payload);

/**
 * Submits the crash report to the optional SigmaOS Enterprise Telemetry endpoints 
 * if the user has telemetry_opt_in enabled.
 */
void crash_submit_telemetry(CrashDumpPayload_t* payload);

#endif // CRASH_REPORTER_H
