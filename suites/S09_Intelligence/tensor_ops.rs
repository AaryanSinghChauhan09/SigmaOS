//! =============================================================================
//! Σ SIGMAOS: HARDWARE-NATIVE INTELLIGENCE (v1.0)
//! =============================================================================
//! Abstract contracts for Tensor Operations.
//! Defines the interface between SigmaOS and hardware accelerators (NPUs/GPUs).
//!
//! Standard: `#![no_std]`, bare-metal compatible
//! =============================================================================

#![no_std]

/// Error types for hardware offload failures
#[derive(Debug)]
pub enum OffloadError {
    DeviceBusy,
    OutOfMemory,
    UnsupportedOperation,
    HardwareFault,
}

/// Abstract representation of a tensor resident in memory
pub struct Tensor {
    pub id: u32,
    pub shape: [usize; 4],
    pub data_ptr: *mut u8,
}

/// Activation functions supported by hardware backends
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationKind {
    ReLU,
    Sigmoid,
    Tanh,
    Softmax,
}

/// Pooling operations supported by hardware backends
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolingKind {
    Max,
    Average,
}

/// Descriptor of what a specific hardware backend can accelerate
pub struct BackendCapabilities {
    pub supports_matmul: bool,
    pub supports_convolution: bool,
    pub max_tensor_dim: usize,
    // Note: In no_std we avoid Vec, using bitmasks or fixed arrays instead
    pub supported_activations_mask: u32,
    pub supported_pooling_mask: u32,
}

/// Abstract contract for hardware-native intelligence.
/// Each backend (CPU, GPU, NPU) will implement this trait.
pub trait TensorOps {
    /// Matrix multiplication: C = A × B
    fn matmul(&self, a: &Tensor, b: &Tensor) -> Result<Tensor, OffloadError>;

    /// Convolution: output = conv(input, kernel)
    fn convolution(
        &self, 
        input: &Tensor, 
        kernel: &Tensor, 
        stride: usize, 
        padding: usize
    ) -> Result<Tensor, OffloadError>;

    /// Activation functions (ReLU, Sigmoid, etc.)
    fn activation(&self, input: &Tensor, kind: ActivationKind) -> Result<Tensor, OffloadError>;

    /// Pooling (max/avg)
    fn pooling(
        &self, 
        input: &Tensor, 
        kind: PoolingKind, 
        kernel_size: usize, 
        stride: usize
    ) -> Result<Tensor, OffloadError>;

    /// Capability discovery: report what this backend supports
    fn capabilities(&self) -> BackendCapabilities;
}
