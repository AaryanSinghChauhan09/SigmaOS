#ifndef SIGMA_AI_INFERENCE_H
#define SIGMA_AI_INFERENCE_H

#include <sigma_types.h>

// SigmaOS On-Device AI/ML Inference Engine
// Zero-dependency neural processing abstraction

// Initialize NPU or fall back to GPU compute
void intel_init_inference_engine(void);

// Load quantized models (e.g., GGUF, ONNX formats) natively into memory
uint32_t intel_load_model(const char* model_path);

// Execute an inference pass, returns handle to result buffer
void* intel_execute_inference(uint32_t model_id, const void* input_tensor, uint32_t input_size);

// Hook for native Voice Assistant integration (constant listener)
void intel_start_voice_assistant(void);

// Real-time Gesture Recognition parsing
void intel_process_gesture_stream(void* camera_feed_buffer);

#endif // SIGMA_AI_INFERENCE_H

