#include <stdint.h>
#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS AI/ML Hardware Abstraction Layer (HAL)
// USP: Zero-Copy Tensor Pipeline straight to physical memory
// bypassing user-space copy overhead.
// ---------------------------------------------------------

#define MAX_TENSORS 32

typedef struct {
    uint32_t tensor_id;
    uint32_t owner_pid;
    uint64_t phys_addr; // Direct physical memory address
    uint32_t size_bytes;
    uint8_t  precision; // 8 = INT8, 16 = FP16, 32 = FP32
    uint8_t  active;
} accel_tensor_t;

static accel_tensor_t tensor_registry[MAX_TENSORS];
static uint32_t tensor_count = 0;

extern int cap_registry_verify(uint32_t cap_id, uint32_t pid, uint8_t required_rights);
extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);
extern void serial_write(const char* str); // Mock IO

// Initialize the NPU/TPU hardware
void accel_hal_init(void) {
    // In real implementation: scan PCI bus for known NPU devices, map MMIO
    serial_write("[NPU] ML Accelerator HAL Initialised.\n");
}

// Register a tensor directly in physical memory (Zero-Copy)
// Requires CAP_AI_COMPUTE capability
int accel_tensor_register(uint32_t pid, uint64_t phys_addr, uint32_t size, uint8_t precision, uint32_t cap_token) {
    if (!cap_registry_verify(cap_token, pid, 0x01)) {
        audit_chain_append(pid, 3, "AI_ACCEL_DENIED_CAP_FAILURE");
        return -1;
    }

    if (tensor_count >= MAX_TENSORS) return -2;

    accel_tensor_t* t = &tensor_registry[tensor_count];
    t->tensor_id  = tensor_count++;
    t->owner_pid  = pid;
    t->phys_addr  = phys_addr;
    t->size_bytes = size;
    t->precision  = precision;
    t->active     = 1;

    audit_chain_append(pid, 1, "TENSOR_REGISTERED");
    return t->tensor_id;
}

// Dispatch tensor to hardware for processing
int accel_execute_model(uint32_t pid, uint32_t tensor_in_id, uint32_t tensor_out_id, uint32_t cap_token) {
    if (!cap_registry_verify(cap_token, pid, 0x01)) return -1;
    if (tensor_in_id >= tensor_count || tensor_out_id >= tensor_count) return -2;

    accel_tensor_t* t_in = &tensor_registry[tensor_in_id];
    accel_tensor_t* t_out = &tensor_registry[tensor_out_id];

    if (t_in->owner_pid != pid || t_out->owner_pid != pid) return -3;

    // Simulate hardware execution (e.g. ringing a doorbell on the PCI device)
    // mmio_write32(NPU_BASE + NPU_DOORBELL, t_in->phys_addr);
    audit_chain_append(pid, 1, "MODEL_EXECUTED_ON_NPU");

    return 0; // Success (Asynchronous interrupt will signal completion)
}
