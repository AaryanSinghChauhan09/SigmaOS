use alloc::vec;
// Dynamic Matrix Quantization & Multi-Device Execution Fallback for SigmaOS
// Inspired by vLLM, llama.cpp, and ROCm runtime fallback pipelines.

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TensorDtype {
    Fp32,
    Fp16,
    Bf16,
    Int8,
    Int4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ComputeDeviceTarget {
    CPU,
    CpuSimd,
    GPU,
    DiscreteGpu,
    IntegratedGpu,
    NPU,
    IntegratedNpu,
    TPU,
    AutoSelect,
}

/// Quantized Tensor Container holding weights, scales, and zero-points.
#[derive(Debug, Clone)]
pub struct QuantizedMatrix {
    pub name: String,
    pub original_dtype: TensorDtype,
    pub target_dtype: TensorDtype,
    pub rows: usize,
    pub cols: usize,
    pub quantized_data: Vec<u8>,
    pub scales: Vec<f32>,
    pub zero_points: Vec<i8>,
    pub compression_ratio: f32,
}

impl QuantizedMatrix {
    /// Quantizes raw FP32 weights into target dtype (FP16, INT8, or INT4).
    pub fn quantize_fp32_matrix(
        name: &str,
        rows: usize,
        cols: usize,
        weights: &[f32],
        target_dtype: TensorDtype,
    ) -> Result<Self, &'static str> {
        if weights.len() != rows * cols {
            return Err("Weights length does not match specified matrix dimensions");
        }

        let mut quantized_data = Vec::new();
        let mut scales = Vec::new();
        let mut zero_points = Vec::new();

        match target_dtype {
            TensorDtype::INT8 => {
                // Per-row symmetric INT8 quantization
                for r in 0..rows {
                    let row_slice = &weights[r * cols..(r + 1) * cols];
                    let max_val = row_slice.iter().map(|v| v.abs()).fold(0.0f32, f32::max);
                    let scale = if max_val == 0.0 { 1.0 } else { max_val / 127.0 };
                    scales.push(scale);
                    zero_points.push(0);

                    for &w in row_slice {
                        let q = (w / scale).clamp(-128.0, 127.0) as i8;
                        quantized_data.push(q as u8);
                    }
                }
            }
            TensorDtype::INT4 => {
                // Pack two 4-bit elements per byte
                for r in 0..rows {
                    let row_slice = &weights[r * cols..(r + 1) * cols];
                    let max_val = row_slice.iter().map(|v| v.abs()).fold(0.0f32, f32::max);
                    let scale = if max_val == 0.0 { 1.0 } else { max_val / 7.0 };
                    scales.push(scale);
                    zero_points.push(0);

                    for chunk in row_slice.chunks(2) {
                        let q0 = (chunk[0] / scale).clamp(-8.0, 7.0) as i8 & 0x0F;
                        let q1 = if chunk.len() > 1 {
                            (chunk[1] / scale).clamp(-8.0, 7.0) as i8 & 0x0F
                        } else {
                            0
                        };
                        let packed = (q0 as u8) | ((q1 as u8) << 4);
                        quantized_data.push(packed);
                    }
                }
            }
            TensorDtype::FP16 | TensorDtype::BF16 => {
                // Simulated FP16 conversion (2 bytes per weight)
                for &w in weights {
                    let bytes = (w as f32).to_le_bytes();
                    quantized_data.push(bytes[0]);
                    quantized_data.push(bytes[1]);
                }
                scales.push(1.0);
                zero_points.push(0);
            }
            TensorDtype::FP32 => {
                for &w in weights {
                    quantized_data.extend_from_slice(&w.to_le_bytes());
                }
                scales.push(1.0);
                zero_points.push(0);
            }
        };

        let orig_size = weights.len() * 4;
        let comp_size = quantized_data.len();
        let compression_ratio = if comp_size > 0 {
            orig_size as f32 / comp_size as f32
        } else {
            1.0
        };

        Ok(Self {
            name: name.to_string(),
            original_dtype: TensorDtype::FP32,
            target_dtype,
            rows,
            cols,
            quantized_data,
            scales,
            zero_points,
            compression_ratio,
        })
    }
}

/// Fallback Execution Route for AI operators.
#[derive(Debug, Clone)]
pub struct DeviceFallbackRoute {
    pub primary_device: ComputeDeviceTarget,
    pub active_device: ComputeDeviceTarget,
    pub is_fallback_active: bool,
    pub fallback_reason: String,
}

/// Multi-Device Execution Fallback Dispatcher.
pub struct AiExecutionDispatcher {
    discrete_gpu_available: bool,
    integrated_npu_available: bool,
    cpu_simd_available: bool,
    fallback_count: usize,
}

impl AiExecutionDispatcher {
    pub fn new(gpu: bool, npu: bool, simd: bool) -> Self {
        Self {
            discrete_gpu_available: gpu,
            integrated_npu_available: npu,
            cpu_simd_available: simd,
            fallback_count: 0,
        }
    }

    /// Resolves best available target device with automatic fallback hierarchy.
    pub fn resolve_device_route(&mut self, requested: ComputeDeviceTarget) -> DeviceFallbackRoute {
        match requested {
            ComputeDeviceTarget::DiscreteGpu => {
                if self.discrete_gpu_available {
                    DeviceFallbackRoute {
                        primary_device: requested,
                        active_device: ComputeDeviceTarget::DiscreteGpu,
                        is_fallback_active: false,
                        fallback_reason: String::new(),
                    }
                } else if self.integrated_npu_available {
                    self.fallback_count += 1;
                    DeviceFallbackRoute {
                        primary_device: requested,
                        active_device: ComputeDeviceTarget::IntegratedNpu,
                        is_fallback_active: true,
                        fallback_reason: "Discrete GPU unavailable, falling back to Integrated NPU"
                            .to_string(),
                    }
                } else {
                    self.fallback_count += 1;
                    DeviceFallbackRoute {
                        primary_device: requested,
                        active_device: ComputeDeviceTarget::CPU_SIMD,
                        is_fallback_active: true,
                        fallback_reason:
                            "Discrete GPU and NPU unavailable, falling back to CPU SIMD/AVX-512"
                                .to_string(),
                    }
                }
            }
            ComputeDeviceTarget::IntegratedNpu => {
                if self.integrated_npu_available {
                    DeviceFallbackRoute {
                        primary_device: requested,
                        active_device: ComputeDeviceTarget::IntegratedNpu,
                        is_fallback_active: false,
                        fallback_reason: String::new(),
                    }
                } else {
                    self.fallback_count += 1;
                    DeviceFallbackRoute {
                        primary_device: requested,
                        active_device: ComputeDeviceTarget::CPU_SIMD,
                        is_fallback_active: true,
                        fallback_reason:
                            "Integrated NPU unavailable, falling back to CPU SIMD/AVX-512"
                                .to_string(),
                    }
                }
            }
            _ => DeviceFallbackRoute {
                primary_device: requested,
                active_device: ComputeDeviceTarget::CPU_SIMD,
                is_fallback_active: false,
                fallback_reason: String::new(),
            },
        }
    }

    /// Simulates matrix GEMM operation execution on resolved target device.
    pub fn execute_gemm(
        &self,
        matrix: &QuantizedMatrix,
        route: &DeviceFallbackRoute,
    ) -> (usize, u64) {
        let ops = matrix.rows * matrix.cols * 2;
        let exec_time_us = match route.active_device {
            ComputeDeviceTarget::DiscreteGpu => ops as u64 / 10_000,
            ComputeDeviceTarget::IntegratedNpu => ops as u64 / 5_000,
            ComputeDeviceTarget::CPU_SIMD | ComputeDeviceTarget::AutoSelect => ops as u64 / 1_000,
            _ => ops as u64 / 1_000,
        };

        (ops, exec_time_us.max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantization_and_fallback_dispatcher() {
        let weights = vec![1.2f32, -0.8, 0.5, 2.1, -1.5, 0.0, 0.9, -0.4];
        let qmat = QuantizedMatrix::quantize_fp32_matrix(
            "layer1.weight",
            2,
            4,
            &weights,
            TensorDtype::Int8,
        )
        .unwrap();
        assert_eq!(qmat.quantized_data.len(), 8);
        assert!(qmat.compression_ratio >= 3.9);

        let mut dispatcher = AiExecutionDispatcher::new(false, true, true);
        let route = dispatcher.resolve_device_route(ComputeDeviceTarget::DiscreteGpu);
        assert!(route.is_fallback_active);
        assert_eq!(route.active_device, ComputeDeviceTarget::IntegratedNpu);

        let (ops, us) = dispatcher.execute_gemm(&qmat, &route);
        assert_eq!(ops, 16);
        assert!(us >= 1);
    }
}
