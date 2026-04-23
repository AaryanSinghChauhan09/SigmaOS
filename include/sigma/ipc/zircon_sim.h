/**
 * SigmaOS: Sovereign Capability-Based IPC (S-Zircon)
 * Inspired by Google's Fuchsia/Zircon.
 * USP: Transfer permissions and capabilities via secure message handles.
 */

#ifndef SIGMA_ZIRCON_SIM_H
#define SIGMA_ZIRCON_SIM_H

#include <stdint.h>

typedef uint32_t sigma_handle_t;

#define HANDLE_INVALID 0

// Secure Handle Creation
sigma_handle_t sigma_handle_create(uint32_t shard_id, uint64_t caps);

// Capability Transfer
void sigma_ipc_send_handle(sigma_handle_t dst, sigma_handle_t payload);

// Permission Verification
int sigma_handle_verify(sigma_handle_t handle, uint64_t required_caps);

#endif // SIGMA_ZIRCON_SIM_H
