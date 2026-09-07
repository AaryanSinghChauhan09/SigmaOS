# Sovereign AI Agent 4-Bit Operation Management Specification

This document specifies mandatory rules, bitwise manipulation standards, 4-bit nibble packing and unpacking protocols, AI model weight quantization standards (INT4 / NF4), Binary Coded Decimal (BCD) decoding rules, and hardware bitfield registers for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) operating within SigmaOS kernel and userland subsystems (`src/ai/quantization.rs`, `src/ai/voice.rs`).

---

## 1. 4-Bit Nibble Bitwise Packing & Unpacking Standards

A **nibble** represents a 4-bit unit of data ($2^4 = 16$ possible values, range `0x0` to `0xF` / `0` to `15`). SigmaOS uses nibble packing to achieve 50% memory footprint reduction for compact representations:

1. **Nibble Packing (Two 4-bit Values per Byte)**:
   - Upper (Most Significant) Nibble: `(high_nibble & 0x0F) << 4`
   - Lower (Least Significant) Nibble: `low_nibble & 0x0F`
   - Packed Byte Equation: `packed_byte = ((high_nibble & 0x0F) << 4) | (low_nibble & 0x0F)`
2. **Nibble Unpacking**:
   - Extract Upper Nibble: `high_nibble = (packed_byte >> 4) & 0x0F`
   - Extract Lower Nibble: `low_nibble = packed_byte & 0x0F`

---

## 2. AI Model Weight 4-Bit Quantization (INT4 / NF4) (`src/ai/quantization.rs`, `src/ai/voice.rs`)

Autonomous AI agents deploying edge LLMs, local Whisper voice models, or neural inference engines within SigmaOS must adhere to 4-bit quantization rules:

1. **INT4 Symmetric / Asymmetric Quantization**:
   - Quantizes 32-bit floating point weights (`f32`) into 4-bit unsigned integers (`0` to `15`) or signed integers (`-8` to `7`).
   - Scales weights using per-block scaling factors `f32_scale` and `f32_zero_point`:
     $$\text{quantized\_nibble} = \text{clamp}\left(\left\lfloor \frac{\text{weight}}{\text{scale}} \right\rfloor + \text{zero\_point}, 0, 15\right)$$
2. **NF4 (NormalFloat4) Quantization**:
   - Utilizes non-linear 4-bit quantization levels optimized for zero-mean normal distributions.
   - Two quantized NF4 weights are packed into a single byte array slice, cutting model memory bandwidth demands by 75% compared to FP16.
3. **De-quantization Hot Path**:
   - De-quantization routines must process packed bytes in SIMD 64-byte chunks, extracting nibbles without branching or dynamic heap allocations.

---

## 3. Binary Coded Decimal (BCD) Real-Time Clock Decoding

Legacy Real-Time Clock (RTC CMOS) registers store time values in Binary Coded Decimal (BCD) format where each decimal digit is encoded as a 4-bit nibble:

1. **BCD Nibble Extraction**:
   - `tens_digit = (bcd_byte >> 4) & 0x0F`
   - `units_digit = bcd_byte & 0x0F`
   - `binary_value = (tens_digit * 10) + units_digit`
2. **BCD Encoding**:
   - `bcd_byte = (((binary_value / 10) & 0x0F) << 4) | ((binary_value % 10) & 0x0F)`

---

## 4. Hardware Peripheral 4-Bit Control Registers & Bitfields

1. **AHCI SATA / PCI Capability Registers**:
   - PCI Express Link Speed, Interrupt Pin ID, and AHCI Port Multiplier Index fields are formatted as 4-bit bitfields.
2. **Bitfield Masking Invariants**:
   - Always clear target 4-bit fields using `val & !(0x0F << shift)` before OR-ing updated 4-bit values `(new_val & 0x0F) << shift` to avoid overwriting adjacent register bits.

---

## 5. AI Agent 4-Bit Directives Summary

1. **Mask Prior to Shift**: Always apply `& 0x0F` to isolate 4-bit values before shifting into the upper nibble position.
2. **Zero Heap Allocations on De-quantization**: Perform INT4/NF4 model weight unpacks in-place over pre-allocated slice buffers.
3. **Preserve Bitfield Bounds**: Ensure hardware register bitfield writes do not bleed outside their assigned 4-bit boundary.
