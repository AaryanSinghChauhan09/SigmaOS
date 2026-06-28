// SPDX-License-Identifier: GPL-2.0-only
// sigma_boot_hardening.h — SigmaOS Boot Architecture & Hardening
// Purpose: < 2s boot target. UEFI-direct (no GRUB). Parallel sigma-init.
//          Hardware-profile-optimised kernel. Atomic A/B partitions.
//          Hibernation resume < 3s. Memory safety roadmap (Rust migration).

#pragma once
#include <stdint.h>
#include <stdbool.h>

// ---------------------------------------------------------------------------
// Boot Timing Targets
// ---------------------------------------------------------------------------

#define SIGMA_BOOT_TARGET_TOTAL_MS       2000   // < 2 seconds to desktop
#define SIGMA_BOOT_TARGET_KERNEL_MS       400   // Kernel init
#define SIGMA_BOOT_TARGET_SERVICES_MS     800   // All services parallel start
#define SIGMA_BOOT_TARGET_DESKTOP_MS      800   // Zenith WM + DM ready
#define SIGMA_BOOT_HIBERNATE_RESUME_MS   3000   // < 3 seconds from hibernate

// Boot sequence:
// UEFI firmware
//   └─ sigma-boot (UEFI app, EFI/sigmaos/sigma-boot.efi)
//       └─ Kernel (hardware-profiled build for this machine)
//           └─ sigma-init (parallel, not sequential)
//               ├─ sigma-bus (IPC, needed first)
//               ├─ sigma-trustd + sigma-ids (security)
//               ├─ sigma-netd (networking)
//               ├─ sigma-apid (management API)
//               ├─ sigma-heald (self-heal daemon)
//               ├─ sigma-commnetd (if enabled)
//               ├─ sigma-display (framebuffer + DRM)
//               └─ sigma-dm (display manager → Zenith WM)

// ---------------------------------------------------------------------------
// Kernel Hardware Profiling (sigma-dna)
// ---------------------------------------------------------------------------

typedef struct {
    // CPU
    char     cpu_vendor[32];         // "GenuineIntel", "AuthenticAMD"
    char     cpu_model[64];
    uint8_t  cpu_physical_cores;
    uint8_t  cpu_logical_cores;
    bool     has_avx2;
    bool     has_avx512;
    bool     has_aes_ni;             // Hardware AES (used by CryptFS)
    bool     has_sha_ni;             // Hardware SHA
    bool     has_tpm2;
    // GPU
    char     gpu_vendor[32];
    char     gpu_driver[32];         // Detected best driver
    bool     has_vulkan;
    // RAM
    uint32_t ram_mb;
    // Storage
    char     boot_device[32];        // /dev/nvme0n1, /dev/sda
    bool     is_nvme;
    bool     is_ssd;
    // Features to enable based on profile
    bool     enable_hugepages;       // If RAM ≥ 8GB
    bool     enable_zram;            // If RAM < 4GB
    bool     enable_preempt_rt;      // If latency-sensitive workload detected
    bool     use_bfq_scheduler;      // Desktop I/O scheduler
    bool     use_mq_deadline;        // Server I/O scheduler
    // Energy profile
    bool     is_laptop;
    bool     enable_powersave;
} sigma_dna_profile_t;

int sigma_dna_profile_detect(sigma_dna_profile_t *out);
int sigma_dna_kernel_args_generate(const sigma_dna_profile_t *profile,
                                    char *args_out, size_t len);

// ---------------------------------------------------------------------------
// A/B Partition Update Manager
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_BOOT_SLOT_A   = 0,
    SIGMA_BOOT_SLOT_B   = 1,
} sigma_boot_slot_t;

typedef struct {
    sigma_boot_slot_t current_slot;    // Currently running slot
    sigma_boot_slot_t active_slot;     // Will boot into next time
    char   slot_a_version[32];
    char   slot_b_version[32];
    bool   slot_a_healthy;             // Marked healthy after successful boot
    bool   slot_b_healthy;
    bool   update_pending;             // Update downloaded to inactive slot
    char   pending_version[32];
    uint32_t boot_attempt_count;       // Resets on successful sigma-healthd report
    uint32_t max_boot_attempts;        // If exceeded → rollback to other slot
    char   rollback_reason[128];
} sigma_boot_ab_status_t;

int sigma_boot_ab_get_status(sigma_boot_ab_status_t *out);
int sigma_boot_ab_switch_slot(sigma_boot_slot_t target);
int sigma_boot_ab_mark_healthy(void);    // Called by sigma-healthd after boot
int sigma_boot_ab_rollback(const char *reason);

// ---------------------------------------------------------------------------
// Parallel Init (sigma-init)
// ---------------------------------------------------------------------------

typedef struct {
    char     service_name[64];
    uint32_t start_latency_ms;       // How long this service took to start
    bool     started_ok;
    char     error[128];
    uint32_t depends_on_count;
    char     depends_on[8][64];      // Hard dependencies (must start first)
    bool     parallel_ok;            // Can start without all deps done
} sigma_init_service_t;

// sigma-init starts all services simultaneously:
// - sigma-bus starts first (IPC needed by all)
// - All others start in parallel, runtime dep resolution
// - Total target: all critical services ready in < 800ms
int sigma_init_boot_report(sigma_init_service_t *services, int *count,
                             uint32_t *total_ms);

// ---------------------------------------------------------------------------
// Memory Safety Roadmap (compile-time feature flags)
// ---------------------------------------------------------------------------

// Phase 1 (current): new subsystems in Rust
//   sigma-net, sigma-fs driver layer, SDF driver framework
// Phase 2: Rust rewrites of critical C++ paths
//   sigma_sched → sigma_sched_rs
//   sigma_mm    → sigma_mm_rs
//   sigma_ipc   → sigma_ipc_rs
// Phase 3: Formal verification (Frama-C / seL4 proof style)
//   Scheduler: machine-verified correct
//   IPC: formally proven race-condition free
//   Allocator: proven no leaks

#ifdef SIGMA_RUST_SCHED
extern int sigma_sched_rs_init(void);
extern int sigma_sched_rs_tick(void);
#endif

#ifdef SIGMA_RUST_NET
extern int sigma_net_rs_init(void);
#endif

// Build flags summary:
// SIGMA_RUST_SCHED    — use Rust scheduler (Phase 2)
// SIGMA_RUST_NET      — use Rust network stack (Phase 1, default)
// SIGMA_RUST_FS       — use Rust filesystem layer (Phase 1, default)
// SIGMA_FORMAL_VERIFY — enable Frama-C assertion hooks (Phase 3)
// SIGMA_DP_EPSILON    — differential privacy budget for fedlearn (default 0.5)

// ---------------------------------------------------------------------------
// Anti-NVIDIA Nightmare: Sigma Driver Framework (SDF) ABI Stability
// ---------------------------------------------------------------------------

// SDF guarantee: driver binary compiled for SigmaOS 1.0 works on SigmaOS 5.0
// No DKMS. No recompile after kernel update. No driver breakage.
// Drivers run in userspace with capability channels (Fuchsia DDK inspired)

typedef struct {
    char     driver_name[64];
    char     driver_version[16];
    char     sdf_abi_version[8];     // Must match running kernel SDF ABI
    bool     abi_compatible;
    bool     running_in_userspace;   // Always true for SDF drivers
    bool     crash_isolated;         // Driver crash ≠ kernel panic
    bool     hot_reload_capable;     // Can reload without reboot
    uint32_t crash_count;
    uint32_t restart_count;          // Auto-restarted by sigma-rs
} sigma_sdf_driver_status_t;

int sigma_sdf_driver_list(sigma_sdf_driver_status_t *drivers, int *count);
int sigma_sdf_driver_reload(const char *driver_name);
int sigma_sdf_abi_version_get(char *version_out, size_t len);
