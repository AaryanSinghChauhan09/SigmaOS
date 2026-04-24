//! =============================================================================
//! Σ SIGMAOS: MOCK NPU BACKEND (v1.0)
//! =============================================================================
//! A stub driver simulating a hardware Neural Processing Unit.
//! Used to test the TensorOps contract without requiring physical NPU silicon.
//!
//! Standard: `#![no_std]`, bare-metal compatible
//! =============================================================================

#![no_std]

use crate::tensor_ops::{
    TensorOps, Tensor, ActivationKind, PoolingKind, 
    BackendCapabilities, OffloadError
};

pub struct MockNpuBackend {
    pub is_online: bool,
}

impl MockNpuBackend {
    pub fn new() -> Self {
        MockNpuBackend { is_online: true }
    }
}

impl TensorOps for MockNpuBackend {
    fn matmul(&self, a: &Tensor, _b: &Tensor) -> Result<Tensor, OffloadError> {
        if !self.is_online { return Err(OffloadError::DeviceBusy); }
        // Simulate hardware MMIO dispatch here
        // Return a dummy tensor indicating successful offload
        Ok(Tensor {
            id: 999,
            shape: a.shape,
            data_ptr: core::ptr::null_mut(),
        })
    }

    fn convolution(&self, input: &Tensor, _kernel: &Tensor, _stride: usize, _padding: usize) -> Result<Tensor, OffloadError> {
        if !self.is_online { return Err(OffloadError::DeviceBusy); }
        Ok(Tensor {
            id: 998,
            shape: input.shape,
            data_ptr: core::ptr::null_mut(),
        })
    }

    fn activation(&self, input: &Tensor, _kind: ActivationKind) -> Result<Tensor, OffloadError> {
        if !self.is_online { return Err(OffloadError::DeviceBusy); }
        Ok(Tensor {
            id: 997,
            shape: input.shape,
            data_ptr: core::ptr::null_mut(),
        })
    }

    fn pooling(&self, input: &Tensor, _kind: PoolingKind, _kernel_size: usize, _stride: usize) -> Result<Tensor, OffloadError> {
        if !self.is_online { return Err(OffloadError::DeviceBusy); }
        Ok(Tensor {
            id: 996,
            shape: input.shape,
            data_ptr: core::ptr::null_mut(),
        })
    }

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            supports_matmul: true,
            supports_convolution: true,
            max_tensor_dim: 4096,
            supported_activations_mask: 0xFFFFFFFF, // All supported
            supported_pooling_mask: 0xFFFFFFFF,     // All supported
        }
    }
}
