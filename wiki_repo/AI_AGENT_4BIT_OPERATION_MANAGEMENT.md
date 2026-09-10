# AI Agent 4-Bit Operation & INT4 Quantization Management

This document defines the operational standards, memory allocation guidelines, and hardware execution dispatch protocols for **4-bit quantized operations (INT4 / Q4_K_M)** within SigmaOS.

---

## 1. Overview & Architecture

SigmaOS natively supports **4-bit quantized tensor execution** to enable edge AI inference, low-power ambient processing, and local LLM/Speech-to-Text execution with minimal memory footprint (<2GB RAM).

### Core Components
- **`TensorDtype::Int4` / `TensorDtype::INT4`** (`src/ai/tensor_memory.rs`, `src/ai/quantization.rs`): 4-bit integer representation packed as two 4-bit elements per `u8` byte.
- **`QuantizedMatrix`** (`src/ai/quantization.rs`): Container holding 4-bit weight arrays, per-block scale factors (`FP16`), and zero-point offsets (`INT8`).
- **`QuantizationType::Q4KM` / `QuantizationType::Int4`** (`src/ai/local_llm.rs`, `src/ai/llm.rs`): Quantization profile reducing FP32 weight memory requirements by 87.5% (0.125x memory multiplier).
- **`WhisperSpeechToText` 4-bit GGUF Decoder** (`src/ai/voice.rs`): Local STT audio decoding using 4-bit quantized Whisper weights.

---

## 2. 4-Bit Weight Packing & Tensor Layout

In SigmaOS, 4-bit integer values are nibble-packed into standard 8-bit unsigned integer arrays (`Vec<u8>`):

```
Byte Offset N:  [ High Nibble (Bits 7..4): Element 2k+1 | Low Nibble (Bits 3..0): Element 2k ]
```

### Quantization & Packing Protocol
1. **Symmetric & Asymmetric Quantization**:
   $$\text{Scale } S = \frac{X_{\max} - X_{\min}}{15}$$
   $$q_i = \text{clamp}\left(\text{round}\left(\frac{x_i - X_{\min}}{S}\right), 0, 15\right)$$
2. **Byte Packing**:
   ```rust
   let packed = ((q_odd & 0x0F) << 4) | (q_even & 0x0F);
   quantized_data.push(packed);
   ```
3. **Unpacking & Dequantization**:
   ```rust
   let q_even = packed & 0x0F;
   let q_odd = (packed >> 4) & 0x0F;
   let fp_val_even = (q_even as f32) * scale + zero_point;
   ```

---

## 3. Memory Management & Allocation Rules for AI Agents

When allocating memory for 4-bit models or tensor matrices:
- **Element Size Calculation**: 4-bit tensors consume 0.5 bytes per element. Buffer capacity is computed as `(total_elements + 1) / 2`.
- **Zero-Allocation Execution**: Compute blocks and dequantization buffers must leverage `SovereignBuddyAllocator` or stack-pinned scratchpad memory (`ScratchpadBuffer`).
- **RAM Thresholds for Local Models**:
  - 8B Parameter Models (`Q4_K_M`): ~4.5 GB RAM footprint.
  - 3B Parameter Models (`INT4`): ~1.8 GB RAM footprint.
  - Whisper STT Small (`4-bit GGUF`): ~240 MB RAM footprint.

---

## 4. Hardware Execution Dispatching

AI agents orchestrating 4-bit inference in SigmaOS must verify hardware SIMD extensions:
- **x86_64**: AVX-512 VNNI / AVX2 nibble masking and `vpmaddubsw` dot-product accumulation.
- **AArch64**: ARM NEON `dotprod` (`SDOT`/`UDOT`) and INT4 matrix multiply primitives (`I8MM`).
- **RISC-V 64**: RISC-V Vector Extension (`rvv`) `vwmaccu.vx` widened accumulation.

---

## 5. Security & Verification Directives

1. **W^X Memory Protection**: Tensor scratchpad pages allocated for dequantization MUST be marked Read-Write (`RW`) and NEVER Executable (`X`).
2. **Bounds Checking**: Tensors must validate element alignment and buffer bounds before dispatching SIMD nibble unpack kernels.
