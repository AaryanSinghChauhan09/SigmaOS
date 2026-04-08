#include "../../../include/sigma_kernel.h"

// Activation function parity
sigma_f64 sigma_ai_sigmoid(sigma_f64 x) {
    return 1.0 / (1.0 + sigma_math_exp(-x));
}

void SovereignEdu_AI_ML_Init() {
    sigma_printf("Σ [ABSORB]: AI & Machine Learning & Deep Learning Syllabus Zenith Online.\n");
    sigma_printf("Σ [DS]: NumPy, Pandas, Scikit-Learn & TensorFlow/PyTorch concepts absorbed.\n");
}


