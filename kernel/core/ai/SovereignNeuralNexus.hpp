#ifndef SOVEREIGN_NEURAL_NEXUS_HPP
#define SOVEREIGN_NEURAL_NEXUS_HPP

#include "sigma_kernel_types.h"

class SovereignNeuralEngine {
public:
    static SovereignNeuralEngine& getInstance();
    void init();
    bool loadModel(const char* model_name, sigma_u32 parameters_mb);
    void inferAnomaly(const void* system_telemetry, sigma_u32 size);
    void predict(const void* input_tensor, void* output_tensor);
    void reportStatus();
    bool transpileUI(const char* css_shard, char* out_morphic_shard);

private:
    SovereignNeuralEngine();
    bool probeNPUHardware();
    bool npu_available;
    sigma_u32 active_models;
    bool initialized;
    bool avx512_busy;
};

extern "C" {
    void neural_init();
    bool neural_load_model(const char* model_name, sigma_u32 parameters_mb);
    void neural_infer_anomaly(const void* system_telemetry, sigma_u32 size);
    void neural_predict(const void* input_tensor, void* output_tensor);
    void neural_report_status();
}

#endif
 