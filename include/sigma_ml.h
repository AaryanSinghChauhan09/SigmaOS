/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MACHINE LEARNING (S-ML)
 * =========================================================================
 * Mission: On-device, private, and silicon-direct ML inference.
 * Principle: PQC-signed models, zero-dependency compute kernels.
 * =========================================================================
 */

#ifndef SIGMA_ML_H
#define SIGMA_ML_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    ML_BACKEND_CPU,
    ML_BACKEND_GPU,
    ML_BACKEND_TPU_ACCELERATOR
} sigma_ml_backend_t;

typedef struct {
    char model_name[64];
    sigma_u32 params_count;
    sigma_u8  pqc_signature[64];
} sigma_model_meta_t;

/* --- ML Primitives --- */
void      ml_init(void);
bool      ml_load_model(const char* path, sigma_ml_backend_t backend);
void      ml_infer(const void* input, void* output, sigma_u32 input_size);
void      ml_report_acceleration_status(void);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace AI {

class SovereignMLEngine {
public:
    static SovereignMLEngine& getInstance() {
        static SovereignMLEngine instance;
        return instance;
    }

    void init();
    bool loadModel(const char* path, sigma_ml_backend_t backend);
    void runInference(const void* in, void* out, sigma_u32 size);
    void reportStatus();

private:
    SovereignMLEngine() : m_active_backend(ML_BACKEND_CPU) {}
    sigma_ml_backend_t m_active_backend;
};

} // namespace AI
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_ML_H */
