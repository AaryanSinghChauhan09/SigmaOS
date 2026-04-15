/* S SIGMAOS: SOVEREIGN QUANTUM SHARD HEADER */
#ifndef SOVEREIGN_QUANTUM_SHARD_H
#define SOVEREIGN_QUANTUM_SHARD_H
#include "sigma_types.h"

sigma_u64   sigma_quantum_entropy  (void);
void        sigma_quantum_simulate (sigma_u32 qubits);
void        SovereignQuantumShard_Init (void);
void        SovereignQuantum_Audit     (void);

#endif
