// sigma_ggml_inference.rs — Sovereign GGML Inference Engine
// Language: Rust (#![no_std], no external crates)
// OOP: Tensor trait, QuantizedTensor (impl), InferenceGraph (composition)
// Specification: wiki_repo/AI_SovereignGGML_Inference.md
#![no_std]
#![allow(dead_code)]

// ═══════════════════════════════════════════════════════════════
//  § 1. Fixed-point arithmetic primitives
//        Q4.4 format: top nibble integer, low nibble fraction
// ═══════════════════════════════════════════════════════════════

/// Quantized 8-bit value (INT8 in range [-128, 127])
pub type Q8 = i8;

/// Scale factor in 16-bit fixed-point (Q8.8)
pub type Scale16 = i16;

/// Multiply two Q8 values and return Q8 (saturating)
pub fn q8_mul(a: Q8, b: Q8) -> Q8 {
    let r = (a as i16) * (b as i16) >> 7;
    if r > 127 { 127 } else if r < -128 { -128 } else { r as Q8 }
}

/// Accumulate Q8 array into i32 (dot-product without overflow)
pub fn q8_dot(a: &[Q8], b: &[Q8]) -> i32 {
    let n = if a.len() < b.len() { a.len() } else { b.len() };
    let mut acc: i32 = 0;
    let mut i = 0;
    while i < n {
        acc += (a[i] as i32) * (b[i] as i32);
        i += 1;
    }
    acc
}

/// Dequantize a Q8 value back to float-equivalent (×scale÷128)
pub fn q8_dequant(v: Q8, scale: Scale16) -> i32 {
    ((v as i32) * (scale as i32)) >> 8
}

// ═══════════════════════════════════════════════════════════════
//  § 2. Tensor trait (abstract, OOP interface)
// ═══════════════════════════════════════════════════════════════

pub trait Tensor {
    fn ndim(&self)  -> usize;
    fn dim(&self, axis: usize) -> usize;
    fn total_elems(&self) -> usize;
    fn scale(&self) -> Scale16;
}

// ═══════════════════════════════════════════════════════════════
//  § 3. QuantizedTensor<N> — INT8 fixed-size tensor (implements Tensor)
// ═══════════════════════════════════════════════════════════════

pub struct QuantizedTensor<const ELEMS: usize> {
    pub data:   [Q8; ELEMS],
    pub shape:  [usize; 4],
    pub ndim:   usize,
    pub scale_q8_8: Scale16,   // Scale in Q8.8 fixed-point
}

impl<const ELEMS: usize> QuantizedTensor<ELEMS> {
    pub const fn new(shape: [usize; 4], ndim: usize, scale: Scale16) -> Self {
        Self {
            data: [0i8; ELEMS],
            shape,
            ndim,
            scale_q8_8: scale,
        }
    }

    pub fn fill_from_slice(&mut self, src: &[Q8]) {
        let n = if src.len() < ELEMS { src.len() } else { ELEMS };
        let mut i = 0;
        while i < n {
            self.data[i] = src[i];
            i += 1;
        }
    }

    /// Linear slice view (flattened)
    pub fn as_slice(&self) -> &[Q8] { &self.data[..] }

    /// Dequantize element at index
    pub fn deq(&self, idx: usize) -> i32 {
        if idx >= ELEMS { return 0; }
        q8_dequant(self.data[idx], self.scale_q8_8)
    }
}

impl<const ELEMS: usize> Tensor for QuantizedTensor<ELEMS> {
    fn ndim(&self) -> usize { self.ndim }
    fn dim(&self, axis: usize) -> usize {
        if axis < 4 { self.shape[axis] } else { 0 }
    }
    fn total_elems(&self) -> usize { ELEMS }
    fn scale(&self) -> Scale16 { self.scale_q8_8 }
}

// ═══════════════════════════════════════════════════════════════
//  § 4. Linear layer (matrix-vector multiply in INT8)
// ═══════════════════════════════════════════════════════════════

pub struct LinearLayer<const IN: usize, const OUT: usize> {
    pub weights: [[Q8; IN]; OUT],     // Weight matrix [OUT × IN]
    pub bias:    [i32; OUT],          // Bias in i32 (already dequantized)
    pub w_scale: Scale16,
}

impl<const IN: usize, const OUT: usize> LinearLayer<IN, OUT> {
    pub const fn new(w_scale: Scale16) -> Self {
        Self {
            weights: [[0i8; IN]; OUT],
            bias:    [0i32; OUT],
            w_scale,
        }
    }

    /// INT8 GEMV: out = W * x + bias, returns dequantized i32 array
    pub fn forward(&self, x: &[Q8; IN], out: &mut [i32; OUT]) {
        let mut i = 0;
        while i < OUT {
            let dot: i32 = q8_dot(&self.weights[i], x);
            // Dequantize: multiply scale factors and shift
            let dq = (dot * self.w_scale as i32) >> 8;
            out[i] = dq + self.bias[i];
            i += 1;
        }
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 5. ReLU activation (primitive, no external deps)
// ═══════════════════════════════════════════════════════════════

pub fn relu_i32(v: i32) -> i32 { if v > 0 { v } else { 0 } }

pub fn relu_slice(data: &mut [i32]) {
    let mut i = 0;
    while i < data.len() {
        data[i] = relu_i32(data[i]);
        i += 1;
    }
}

/// Re-quantize i32 slice back to Q8 (saturating, using output scale)
pub fn requantize(src: &[i32], dst: &mut [Q8], scale: Scale16) {
    let n = if src.len() < dst.len() { src.len() } else { dst.len() };
    let mut i = 0;
    while i < n {
        let q = (src[i] << 8) / (scale as i32);
        dst[i] = if q > 127 { 127 } else if q < -128 { -128 } else { q as Q8 };
        i += 1;
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 6. SovereignInferenceGraph — 2-layer MLP (composition)
//        Architecture: [INPUT → HIDDEN → OUTPUT]
// ═══════════════════════════════════════════════════════════════

const INPUT_DIM:  usize = 16;
const HIDDEN_DIM: usize = 8;
const OUTPUT_DIM: usize = 4;

pub struct SovereignInferenceGraph {
    pub layer1: LinearLayer<INPUT_DIM, HIDDEN_DIM>,
    pub layer2: LinearLayer<HIDDEN_DIM, OUTPUT_DIM>,
}

impl SovereignInferenceGraph {
    pub const fn new() -> Self {
        Self {
            layer1: LinearLayer::new(256),   // scale = 1.0 in Q8.8
            layer2: LinearLayer::new(256),
        }
    }

    /// Full forward pass: INPUT → L1(ReLU) → L2 → logits (i32)
    pub fn forward(&self, input: &[Q8; INPUT_DIM]) -> [i32; OUTPUT_DIM] {
        // Layer 1 forward
        let mut hidden_i32 = [0i32; HIDDEN_DIM];
        self.layer1.forward(input, &mut hidden_i32);
        relu_slice(&mut hidden_i32);

        // Re-quantize hidden to Q8 for layer 2
        let mut hidden_q8 = [0Q8; HIDDEN_DIM];
        requantize(&hidden_i32, &mut hidden_q8, 256);

        // Layer 2 forward
        let mut output_i32 = [0i32; OUTPUT_DIM];
        self.layer2.forward(&hidden_q8, &mut output_i32);
        output_i32
    }

    /// Argmax over output logits (classification)
    pub fn classify(&self, input: &[Q8; INPUT_DIM]) -> usize {
        let logits = self.forward(input);
        let mut best_idx = 0;
        let mut best_val = logits[0];
        let mut i = 1;
        while i < OUTPUT_DIM {
            if logits[i] > best_val {
                best_val = logits[i];
                best_idx = i;
            }
            i += 1;
        }
        best_idx
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 7. Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_q8_dot_product() {
        let a: [Q8; 4] = [1, 2, 3, 4];
        let b: [Q8; 4] = [4, 3, 2, 1];
        // 1×4 + 2×3 + 3×2 + 4×1 = 4+6+6+4 = 20
        assert_eq!(q8_dot(&a, &b), 20);
    }

    #[test]
    fn test_relu_slice() {
        let mut data = [-5, 0, 3, -1, 7i32];
        relu_slice(&mut data);
        assert_eq!(data, [0, 0, 3, 0, 7]);
    }

    #[test]
    fn test_inference_graph_classify() {
        let mut graph = SovereignInferenceGraph::new();
        // Set layer1 weight[0] = all 1s → any positive input → class 0 dominant
        graph.layer1.weights[0] = [1i8; INPUT_DIM];
        let input = [1i8; INPUT_DIM];
        let class = graph.classify(&input);
        assert!(class < OUTPUT_DIM);
    }

    #[test]
    fn test_quantized_tensor_ops() {
        const SZ: usize = 4;
        let mut t: QuantizedTensor<SZ> = QuantizedTensor::new([4, 1, 1, 1], 1, 512);
        let src: [Q8; SZ] = [10, 20, -10, 127];
        t.fill_from_slice(&src);
        assert_eq!(t.as_slice()[0], 10);
        assert_eq!(t.total_elems(), SZ);
    }
}
