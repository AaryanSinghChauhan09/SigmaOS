/*
 * =========================================================================
 * Σ SIGMAOS TRANSFORMER ARCHITECTURE: NATIVE C11 ML CORE (SOVEREIGN)
 * =========================================================================
 * Mission: Zero-Dependency LLM Architecture for System-Level Reasoning.
 * Design: Decoder-Only / Multi-Head Attention / Feed-Forward / Quantized.
 * =========================================================================
 */

#ifndef SIGMA_TRANSFORMER_H
#define SIGMA_TRANSFORMER_H

#include "../sigma_kernel_types.h"

#define MAX_SEQ_LEN 128
#define D_MODEL     256
#define N_HEADS     4
#define N_LAYERS    2
#define VOCAB_SIZE  1024

/**
 * Σ SIGMA TRANSFORMER WEIGHTS (Quantized to 8-bit for Silicon Efficiency)
 */
typedef struct {
    float* embedding;
    float* att_qkv;    // Query, Key, Value weights
    float* att_out;    // Attention output
    float* ffn_w1;      // Feed-forward weights 1
    float* ffn_w2;      // Feed-forward weights 2
    float* norm_w;      // LayerNorm weights
} SigmaTransformerLayer;

typedef struct {
    int n_layers;
    int d_model;
    SigmaTransformerLayer* layers;
    float* final_norm;
} SigmaModel;

/**
 * Σ ML PIPELINE PROTOTYPES
 */
void SigmaML_Preprocess(const char* raw_text, int* tokens_out);
void SigmaML_TrainStep(SigmaModel* model, int* tokens_batch, float learning_rate);
void SigmaML_Inference(SigmaModel* model, const int* input_tokens, float* logits_out);
void SigmaML_FineTune(SigmaModel* model, const char* domain_dataset);

#endif // SIGMA_TRANSFORMER_H
