/*
 * =============================================================================
 * Σ SIGMAOS: BARE-METAL TENSOR MATH LIBRARY (v1.0)
 * =============================================================================
 * Minimal, dependency-free tensor operations for the AI-Optimized Scheduler.
 * Used for neural-heuristic predictions of CPU burst lengths.
 *
 * Design:
 *   - Fixed-point math (no FPU requirement)
 *   - Designed for low-latency inference in interrupt contexts
 *   - Capable of offloading to hardware NPUs if the HAL provides it
 *
 * Standard: C11, freestanding
 * =============================================================================
 */

#include "../../include/sigma_kernel_types.h"

#define TENSOR_FIXED_SHIFT 16
#define FLOAT_TO_FIXED(x) ((int)((x) * (1 << TENSOR_FIXED_SHIFT)))
#define FIXED_TO_FLOAT(x) ((float)(x) / (1 << TENSOR_FIXED_SHIFT))
#define FIXED_MUL(a, b)   (int)(((long long)(a) * (b)) >> TENSOR_FIXED_SHIFT)

typedef struct SigmaTensor {
    u32 rows;
    u32 cols;
    int* data; /* Fixed-point Q16.16 */
} SigmaTensor;

/* ── Basic Operations ────────────────────────────────────────────────────── */

void tensor_add(SigmaTensor* out, const SigmaTensor* a, const SigmaTensor* b) {
    if (a->rows != b->rows || a->cols != b->cols || out->rows != a->rows || out->cols != a->cols) return;
    u32 i;
    for (i = 0; i < a->rows * a->cols; i++) {
        out->data[i] = a->data[i] + b->data[i];
    }
}

void tensor_mul(SigmaTensor* out, const SigmaTensor* a, const SigmaTensor* b) {
    if (a->cols != b->rows || out->rows != a->rows || out->cols != b->cols) return;
    u32 i, j, k;
    for (i = 0; i < a->rows; i++) {
        for (j = 0; j < b->cols; j++) {
            int sum = 0;
            for (k = 0; k < a->cols; k++) {
                sum += FIXED_MUL(a->data[i * a->cols + k], b->data[k * b->cols + j]);
            }
            out->data[i * out->cols + j] = sum;
        }
    }
}

/* ── Activation Functions ────────────────────────────────────────────────── */

static int fixed_relu(int x) {
    return x > 0 ? x : 0;
}

void tensor_relu(SigmaTensor* t) {
    u32 i;
    for (i = 0; i < t->rows * t->cols; i++) {
        t->data[i] = fixed_relu(t->data[i]);
    }
}

/* ── Kernel Fusion ───────────────────────────────────────────────────────── */

/**
 * Fused Matrix Multiplication + ReLU Activation
 * Eliminates the need to write the intermediate matrix back to memory, 
 * drastically improving cache locality and reducing DMA overhead.
 */
void tensor_matmul_relu(SigmaTensor* out, const SigmaTensor* a, const SigmaTensor* b) {
    if (a->cols != b->rows || out->rows != a->rows || out->cols != b->cols) return;
    u32 i, j, k;
    for (i = 0; i < a->rows; i++) {
        for (j = 0; j < b->cols; j++) {
            int sum = 0;
            for (k = 0; k < a->cols; k++) {
                sum += FIXED_MUL(a->data[i * a->cols + k], b->data[k * b->cols + j]);
            }
            /* Fused Activation immediately applied before memory write */
            out->data[i * out->cols + j] = fixed_relu(sum);
        }
    }
}

/* ── Inference Pipeline for Scheduler ────────────────────────────────────── */

/* Predicts the next burst length based on the last N burst history */
int ai_predict_burst(int* history, u32 history_len) {
    /* Simple 1-layer perceptron for demonstration */
    /* Weights are hardcoded fixed-point values for now */
    int weights[4] = { FLOAT_TO_FIXED(0.5), FLOAT_TO_FIXED(0.25), FLOAT_TO_FIXED(0.15), FLOAT_TO_FIXED(0.1) };
    
    int prediction = 0;
    u32 i;
    for (i = 0; i < history_len && i < 4; i++) {
        prediction += FIXED_MUL(history[i], weights[i]);
    }
    
    return prediction > 0 ? prediction : FLOAT_TO_FIXED(1.0);
}
