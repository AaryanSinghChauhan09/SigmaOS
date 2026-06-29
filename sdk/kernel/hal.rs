// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Kernel HAL (Rust, no_std)
//! =========================================================================

pub trait SigmaHAL {
    fn read_register(&self, address: usize) -> u32;
    fn write_register(&mut self, address: usize, value: u32);
    fn irq_enable(&mut self);
    fn irq_disable(&mut self);
}

/// Neural Processing Unit HAL — for AI accelerator cluster support
pub trait SigmaNPU {
    /// Submit an inference task to the NPU
    fn submit_inference(&mut self, model_id: usize, input: &[u8]) -> bool;
    /// Poll the completion status of a previously submitted task
    fn poll_result(&self, task_id: usize) -> Option<u32>;
    /// Reset the NPU shard to idle
    fn reset(&mut self);
}

/// Tensor Processing Unit HAL — for large-scale AI training workloads
pub trait SigmaTPU {
    /// Load a tensor into TPU SRAM
    fn load_tensor(&mut self, tensor_id: usize, data: &[u8]) -> bool;
    /// Execute a matrix multiplication operation
    fn matmul(&mut self, tensor_a: usize, tensor_b: usize) -> usize;
    /// Flush TPU output buffer
    fn flush_output(&mut self, output_id: usize);
}
