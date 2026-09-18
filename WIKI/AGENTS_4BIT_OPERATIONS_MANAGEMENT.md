# SigmaOS AI Agent 4-Bit Operations Management Guidelines

## 1. Overview
SigmaOS implements 4-bit INT4 / FP4 quantization and packed SIMD tensor operations managed autonomously by AI agents (such as `AiQuantizationEngine`, `LocalLlmWrapper`, and `SimdTensorPacker`). These guidelines define 4-bit weight packing/unpacking, INT4/FP4 quantization scales, zero-allocation GGUF model execution, and SIMD hardware acceleration (`klib::isa`).

## 2. Core 4-Bit Operations Management Principles

### 2.1 4-Bit INT4 & FP4 Quantization
- **Weight Packing**: AI agents pack two 4-bit quantized weight values into a single 8-bit byte (`uint8_t packed = (w1 & 0x0F) | ((w2 & 0x0F) << 4)`), cutting model memory footprints by 75% compared to FP16.
- **Quantization Scale & Zero-Point**: Quantized blocks store FP16 scale factors and FP16 zero-points (`Q4_0` / `Q4_K` formats in llama.cpp) to preserve model accuracy.

### 2.2 Vectorized SIMD Packing & Unpacking (`klib::isa`)
- **Hardware-Accelerated Dequantization**: AVX-512, AVX2, and ARM NEON SIMD routines unpack 4-bit weight vectors into FP16/FP32 registers on the fly during matrix multiplication, avoiding memory bandwidth bottlenecks.

### 2.3 Zero-Allocation Local LLM Execution
- **Pinned Tensor Memory**: Quantized GGUF model weights are loaded into page-pinned zero-copy tensor buffers (`AiTensorMemoryManager`) allocated via physical page frame allocators (`src/memory/pmm_vmm.rs`).
- **Zero Heap Allocations**: Matrix multiplication loops operate on pre-allocated tensor workspaces without dynamic heap allocations.

---
*Maintained by the SigmaOS AI, Quantization & SIMD Steering Committee.*
