// SigmaOS Sovereign I/O Subsystem
// Absorbs Linux io_uring + Windows IOCP + macOS kqueue
// Fully async, zero-copy, lock-free I/O for kernel and userspace

#include "sigma_types.h"


#define SIGMA_IO_RING_SIZE     4096  // Number of entries in submission/completion ring
#define SIGMA_IO_MAX_FIXED_FD  256

typedef enum {
    SIGMA_IO_OP_READ    = 0,
    SIGMA_IO_OP_WRITE   = 1,
    SIGMA_IO_OP_POLL    = 2,
    SIGMA_IO_OP_CONNECT = 3,
    SIGMA_IO_OP_ACCEPT  = 4,
    SIGMA_IO_OP_SEND    = 5,
    SIGMA_IO_OP_RECV    = 6,
} SigmaIOOpType;

typedef struct {
    SigmaIOOpType op;
    uint32_t      fd;
    void*         buffer;
    uint32_t      length;
    uint64_t      offset;
    uint64_t      user_data;  // Correlator token for callback identification
} SigmaIOSubmission;

typedef struct {
    uint32_t  result;
    uint32_t  flags;
    uint64_t  user_data;
} SigmaIOCompletion;

// Initialize io_uring-style submission + completion ring pair
void sigma_io_ring_init(void);

// Submit a batch of I/O operations (SQE → kernel ring)
uint32_t sigma_io_submit(SigmaIOSubmission* ops, uint32_t count);

// Drain completed I/O results from the completion ring
uint32_t sigma_io_reap(SigmaIOCompletion* results, uint32_t max_results);

// Register fixed file descriptors for zero-copy I/O
void sigma_io_register_fixed_fds(uint32_t* fds, uint32_t count);



