#include <stdint.h>
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS Bare-Metal ML/AI Accelerator HAL
// Direct kernel-level access to GPU / TPU / NPU
// No bloated middleware — sovereign silicon-first
// ---------------------------------------------------------

#define MAX_ACCELERATORS 8

typedef enum {
    ACCEL_GPU,
    ACCEL_TPU,
    ACCEL_NPU,
    ACCEL_FPGA
} accel_type_t;

typedef struct {
    uint32_t     accel_id;
    accel_type_t type;
    uint64_t     mmio_base;    // Memory-Mapped I/O base address
    uint32_t     sram_size_kb; // On-chip SRAM for tensors
    uint32_t     compute_units;
    uint8_t      available;
} accel_desc_t;

// Tensor descriptor — zero-copy between CPU and accelerator
typedef struct {
    void*    data_ptr;          // Points to DMA-coherent memory
    uint32_t dims[4];           // Up to 4D tensor
    uint32_t dtype;             // 0=fp32, 1=fp16, 2=int8
    uint64_t dma_phys_addr;     // Physical address for accelerator DMA
} tensor_t;

static accel_desc_t accelerators[MAX_ACCELERATORS];
static uint32_t accel_count = 0;

// Register an accelerator (called from HAL/driver init)
int accel_register(accel_type_t type, uint64_t mmio_base,
                   uint32_t sram_kb, uint32_t compute_units) {
    if (accel_count >= MAX_ACCELERATORS) return -1;
    accel_desc_t* a = &accelerators[accel_count];
    a->accel_id = accel_count++;
    a->type = type;
    a->mmio_base = mmio_base;
    a->sram_size_kb = sram_kb;
    a->compute_units = compute_units;
    a->available = 1;
    return a->accel_id;
}

// Submit an inference job (zero-copy via DMA)
int accel_submit_inference(uint32_t accel_id, const tensor_t* input, tensor_t* output) {
    if (accel_id >= accel_count) return -1;
    accel_desc_t* a = &accelerators[accel_id];
    if (!a->available) return -2; // Busy

    a->available = 0;

    // Write DMA source/dest to MMIO registers
    volatile uint64_t* mmio = (volatile uint64_t*)a->mmio_base;
    mmio[0] = input->dma_phys_addr;   // Input tensor DMA addr
    mmio[1] = output->dma_phys_addr;  // Output tensor DMA addr
    mmio[2] = 1;                       // Kick off inference (write 1 to control reg)

    // In real implementation: wait for interrupt from accelerator
    // The scheduler would park this process (STATE_PAGING_WAIT equivalent)

    a->available = 1;
    return 0;
}

// Get energy stats from an accelerator (energy-aware sovereignty)
typedef struct {
    uint32_t milliwatts;
    uint32_t celsius_temp;
    uint32_t utilization_pct;
} accel_energy_t;

int accel_get_energy(uint32_t accel_id, accel_energy_t* out) {
    if (accel_id >= accel_count) return -1;
    // Read from MMIO energy registers
    volatile uint64_t* mmio = (volatile uint64_t*)accelerators[accel_id].mmio_base;
    out->milliwatts      = (uint32_t)mmio[8];
    out->celsius_temp    = (uint32_t)mmio[9];
    out->utilization_pct = (uint32_t)mmio[10];
    return 0;
}
