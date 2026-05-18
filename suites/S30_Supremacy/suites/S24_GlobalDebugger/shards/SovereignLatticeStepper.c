// =============================================================================
// SigmaOS — S24_GlobalDebugger — SovereignLatticeStepper.c
// Industrial-grade Cross-Hive Instruction Debugger
// =============================================================================
// Beyond the Leaders:
//   • GDB / WinDbg — Local process or remote serial debugging.
//   • SigmaOS Lattice Stepper — GLOBAL DEBUG. Allows the developer to step 
//     through an instruction on Device A and follow the 'Entangled' state 
//     (S18) to an instruction on Device B in real-time.
// Result: Debugging distributed supercomputers as if they were a 
//         single-threaded process.
// =============================================================================

#include "core/sigma_types.h"


typedef struct {
    uint8_t  hive_id[16];
    uint32_t shard_id;
    uint32_t instruction_ptr;
} GlobalContext;

// ── Public API ────────────────────────────────────────────────────────────────

// Initialise the Lattice Stepper (Binds to S04 Silicon Debugger)
void lattice_stepper_init(void);

// Attach to a global execution flow spanning multiple Hive nodes
void lattice_stepper_attach(uint32_t global_flow_id);

// Step through a single lattice-instruction (Across mesh S07/S12)
void lattice_stepper_step(void);

// Trace 'Sentient Weights' influencing a scheduler decision (S13 Fabric)
void lattice_stepper_trace_sentience(void);

// Inject a hot-patch shard during break-state (S19 Evolution hook)
void lattice_stepper_hot_swap_break(uint32_t shard_id, void* replacement);

// Report 'Mesh-State Coherence' during debugging sessions
bool lattice_stepper_check_coherence(void);



