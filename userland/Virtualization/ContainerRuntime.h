#ifndef SIGMA_CONTAINER_RUNTIME_H
#define SIGMA_CONTAINER_RUNTIME_H

#include "sigma_types.h"

// SigmaOS Hyper-Fast Containerization & Virtualization
// Achieving Docker/LXC parity at the C11 native level

// Initialize cgroups/namespace-equivalent isolations
void virt_init_namespaces(void);

// Launch a sandboxed container image
uint32_t virt_launch_container(const char* image_path);

// Start a heavyweight Virtual Machine leveraging KVM-like primitives
uint32_t virt_launch_vm(const char* disk_image, uint32_t ram_mb);

// Attach debugger/terminal to a running container
void virt_attach_container(uint32_t container_id);

#endif // SIGMA_CONTAINER_RUNTIME_H

