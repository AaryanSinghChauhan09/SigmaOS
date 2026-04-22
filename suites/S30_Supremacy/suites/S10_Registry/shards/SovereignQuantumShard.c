/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN QUANTUM SIMULATOR (v1.0)
 * =========================================================================
 * Mission: High-fidelity simulation of Quantum states and gates.
 * Principles: Superposition, Measurement, Hadamard Transformation.
 *
 * Implements a single-qubit simulator for advanced computational research.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"
#include <math.h>

typedef struct {
    sigma_f64 alpha_real;
    sigma_f64 alpha_imag;
    sigma_f64 beta_real;
    sigma_f64 beta_imag;
} SigmaQubit_t;

/**
 * sigma_quantum_init: Sets qubit to state |0>.
 */
void sigma_quantum_init(SigmaQubit_t* q) {
    q->alpha_real = 1.0;
    q->alpha_imag = 0.0;
    q->beta_real = 0.0;
    q->beta_imag = 0.0;
}

/**
 * sigma_quantum_hadamard: Applies the Hadamard gate (creates superposition).
 * H = 1/sqrt(2) * [[1, 1], [1, -1]]
 */
void sigma_quantum_hadamard(SigmaQubit_t* q) {
    sigma_f64 inv_sqrt2 = 0.70710678118;
    
    sigma_f64 a_r = (q->alpha_real + q->beta_real) * inv_sqrt2;
    sigma_f64 a_i = (q->alpha_imag + q->beta_imag) * inv_sqrt2;
    sigma_f64 b_r = (q->alpha_real - q->beta_real) * inv_sqrt2;
    sigma_f64 b_i = (q->alpha_imag - q->beta_imag) * inv_sqrt2;
    
    q->alpha_real = a_r;
    q->alpha_imag = a_i;
    q->beta_real = b_r;
    q->beta_imag = b_i;
}

/**
 * sigma_quantum_measure: Collapses the qubit to |0> or |1>.
 * Probability of |0> is |alpha|^2.
 */
int sigma_quantum_measure(SigmaQubit_t* q, sigma_f64 random_val) {
    sigma_f64 p0 = (q->alpha_real * q->alpha_real) + (q->alpha_imag * q->alpha_imag);
    
    if (random_val < p0) {
        q->alpha_real = 1.0; q->alpha_imag = 0.0;
        q->beta_real = 0.0; q->beta_imag = 0.0;
        return 0;
    } else {
        q->alpha_real = 0.0; q->alpha_imag = 0.0;
        q->beta_real = 1.0; q->beta_imag = 0.0;
        return 1;
    }
}

/* --- Module Factory --- */

void SovereignQuantum_Register(void) {
    sigma_sigma_sigma_printf("[ORCHESTRATION]: Sovereign Quantum Simulator active.\n");
}



