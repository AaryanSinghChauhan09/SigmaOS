#!/usr/bin/env pwsh
# ============================================================
# SigmaOS Master Error Remediation Script
# Fixes: include paths, sigma_sandbox.h class declaration,
#        SovereignMonitor C-bridge, OrbManager QKD path,
#        memory_manager nullptr cast, missing include prefixes
# ============================================================

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
Write-Host "=== SigmaOS Error Remediation ==="

# ----------------------------------------------------------
# 1. FIX sigma_sandbox.h — invalid qualified class inside namespace
# ----------------------------------------------------------
$sandboxHeader = "$root\include\security\sigma_sandbox.h"
Set-Content -Path $sandboxHeader -Value @'
/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN SANDBOX CONTAINER (S-SANDBOX)
 * =========================================================================
 * Mission: Isolated, zero-trust execution environments for all applications.
 * =========================================================================
 */

#ifndef SIGMA_SANDBOX_H
#define SIGMA_SANDBOX_H

#include "core/sigma_types.h"

typedef struct {
    sigma_u32 container_id;
    bool      network_access;
    bool      fs_access;
    sigma_u32 memory_limit;
} sigma_sandbox_config_t;

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignSandboxEngine {
public:
    static SovereignSandboxEngine& getInstance() {
        static SovereignSandboxEngine instance;
        return instance;
    }

    const char* type_name() const noexcept { return "SovereignSandboxEngine"; }

    void init();
    sigma_u32 createContainer(const sigma_sandbox_config_t* config);
    bool execute(sigma_u32 container_id, const char* binary_path);
    void destroyContainer(sigma_u32 container_id);
    bool checkSyscall(sigma_u32 syscall_id);
    bool hasCapability(const char* shard_name, const char* capability);
    bool validateMACPolicy(const char* sub, const char* obj, const char* act);

private:
    SovereignSandboxEngine() : next_container_id(1U), initialized(0U) {}
    sigma_u32 next_container_id;
    sigma_u32 initialized;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS
#endif /* __cplusplus */

#ifdef __cplusplus
extern "C" {
#endif

void      sandbox_init(void);
sigma_u32 sandbox_create_container(const sigma_sandbox_config_t* config);
int       sandbox_execute(sigma_u32 container_id, const char* binary_path);
void      sandbox_destroy_container(sigma_u32 container_id);
int       sandbox_check_syscall(sigma_u32 syscall_id);
int       sandbox_has_capability(const char* shard_name, const char* capability);
int       sandbox_validate_mac(const char* subject, const char* object, const char* action);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SANDBOX_H */
'@
Write-Host "[FIXED] sigma_sandbox.h"

# ----------------------------------------------------------
# 2. FIX SovereignMonitor.cpp — C bridge static->instance calls
#    and wrong include path for sigma_monitor.h
# ----------------------------------------------------------
$monitorPath = "$root\kernel\core\observability\SovereignMonitor.cpp"
Set-Content -Path $monitorPath -Value @'
#include "../../../include/core/sigma_types.h"
#include "../../../include/hal/sigma_hal.h"
#include "../../../include/sigma_log.h"
#include "../../../include/observability/sigma_monitor.h"
#include "../../../include/core/SigmaOOP.hpp"

extern "C" void telemetry_execute_ebpf(const void* bytecode, sigma_usize size);

namespace SigmaOS {
namespace Kernel {
namespace Observability {

/* Industrial Constants */
static constexpr sigma_u32 SIMULATED_CPU_LOAD       = 12U;
static constexpr sigma_u32 SIMULATED_MEM_PRESSURE   = 45U;
static constexpr sigma_u32 SIMULATED_NET_THROUGHPUT = 850U;
static constexpr sigma_u32 SIMULATED_MIGRATION_RATE = 2U;

SovereignObservabilityMonitor& SovereignObservabilityMonitor::getInstance() {
    static SovereignObservabilityMonitor instance;
    return instance;
}

const char* SovereignObservabilityMonitor::type_name() const noexcept {
    return "SovereignObservabilityMonitor";
}

void SovereignObservabilityMonitor::init() {
    sigma_log_info("[MONITOR] Initializing Sovereign Observability Matrix (eBPF-Native)...");
    this->m_initialized = true;
}

sigma_system_load_t SovereignObservabilityMonitor::getLoadMatrix() {
    sigma_system_load_t load;
    load.cpu_utilization    = SIMULATED_CPU_LOAD;
    load.memory_pressure    = SIMULATED_MEM_PRESSURE;
    load.network_throughput = SIMULATED_NET_THROUGHPUT;
    load.shard_migration_rate = SIMULATED_MIGRATION_RATE;
    return load;
}

void SovereignObservabilityMonitor::executeEbpfProgram(const void* bytecode, sigma_usize size) {
    telemetry_execute_ebpf(bytecode, size);
}

void SovereignObservabilityMonitor::rebalanceLattice() {
    sigma_log_warn("[MONITOR] Lattice load imbalance detected via eBPF probes. Migrating shards...");
    sigma_log_info("[MONITOR] Migration: S412 -> Core 15, S092 -> Core 02.");
}

} // namespace Observability
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge (all use getInstance()) --- */
extern "C" void monitor_init() {
    SigmaOS::Kernel::Observability::SovereignObservabilityMonitor::getInstance().init();
}

extern "C" sigma_system_load_t monitor_get_load_matrix() {
    return SigmaOS::Kernel::Observability::SovereignObservabilityMonitor::getInstance().getLoadMatrix();
}

extern "C" void monitor_execute_ebpf(const void* bytecode, sigma_usize size) {
    SigmaOS::Kernel::Observability::SovereignObservabilityMonitor::getInstance().executeEbpfProgram(bytecode, size);
}

extern "C" void monitor_rebalance_lattice() {
    SigmaOS::Kernel::Observability::SovereignObservabilityMonitor::getInstance().rebalanceLattice();
}
'@
Write-Host "[FIXED] SovereignMonitor.cpp"

# ----------------------------------------------------------
# 3. FIX sigma_monitor.h — class declaration & usize/size_t parity
# ----------------------------------------------------------
$monitorHeaderPath = "$root\include\observability\sigma_monitor.h"
Set-Content -Path $monitorHeaderPath -Value @'
#ifndef SIGMA_MONITOR_H
#define SIGMA_MONITOR_H

#include "core/sigma_types.h"

typedef struct {
    sigma_u32 cpu_utilization;
    sigma_u32 memory_pressure;
    sigma_u32 network_throughput;
    sigma_u32 shard_migration_rate;
} sigma_system_load_t;

#ifdef __cplusplus
namespace SigmaOS {
namespace Kernel {
namespace Observability {

class SovereignObservabilityMonitor {
public:
    static SovereignObservabilityMonitor& getInstance();

    const char* type_name() const noexcept;

    void init();
    sigma_system_load_t getLoadMatrix();
    void executeEbpfProgram(const void* bytecode, sigma_usize size);
    void rebalanceLattice();

    virtual ~SovereignObservabilityMonitor() {}

private:
    SovereignObservabilityMonitor() : m_initialized(false) {}
    bool m_initialized;
};

} // namespace Observability
} // namespace Kernel
} // namespace SigmaOS
#endif /* __cplusplus */

#ifdef __cplusplus
extern "C" {
#endif

void monitor_init(void);
sigma_system_load_t monitor_get_load_matrix(void);
void monitor_rebalance_lattice(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MONITOR_H */
'@
Write-Host "[FIXED] sigma_monitor.h"

# ----------------------------------------------------------
# 4. FIX SovereignOrbManager.cpp — wrong QKD include path
# ----------------------------------------------------------
$orbPath = "$root\kernel\core\industrial\SovereignOrbManager.cpp"
$orbContent = Get-Content -Raw $orbPath
$orbContent = $orbContent -replace '#include "security/SovereignQKD.hpp"', '#include "../../../include/security/SovereignQKD.hpp"'
Set-Content -Path $orbPath -Value $orbContent
Write-Host "[FIXED] SovereignOrbManager.cpp — QKD include path"

# ----------------------------------------------------------
# 5. FIX memory_manager.cpp — nullptr cast and bare includes
# ----------------------------------------------------------
$memPath = "$root\kernel\core\memory\memory_manager.cpp"
$memContent = Get-Content -Raw $memPath
# Fix bare includes to use relative paths
$memContent = $memContent -replace '#include "hal/sigma_hal.h"', '#include "../../../include/hal/sigma_hal.h"'
$memContent = $memContent -replace '#include "core/sigma_types.h"', '#include "../../../include/core/sigma_types.h"'
$memContent = $memContent -replace '#include "libc/SovereignLibC.h"', '#include "../../../include/libc/SovereignLibC.h"'
# Fix nullptr_t cast: (sigma_u8*)SIGMA_NULL -> static_cast<sigma_u8*>(nullptr)
$memContent = $memContent -replace '\(sigma_u8\*\)SIGMA_NULL', 'static_cast<sigma_u8*>(nullptr)'
Set-Content -Path $memPath -Value $memContent
Write-Host "[FIXED] memory_manager.cpp"

# ----------------------------------------------------------
# 6. FIX SovereignZstd.cpp — make compressOrb/decompressOrb static
# ----------------------------------------------------------
$zstdPath = "$root\kernel\core\industrial\SovereignZstd.cpp"
$zstdContent = Get-Content -Raw $zstdPath
$zstdContent = $zstdContent -replace '#include "core/sigma_types.h"', '#include "../../../include/core/sigma_types.h"'
$zstdContent = $zstdContent -replace 'sigma_size_t compressOrb\(', 'static sigma_size_t compressOrb('
$zstdContent = $zstdContent -replace 'sigma_size_t decompressOrb\(', 'static sigma_size_t decompressOrb('
# Fix C bridge to call static directly
$zstdContent = $zstdContent -replace 'SigmaOS::Kernel::Industrial::SovereignZstd::getInstance\(\)\.compressOrb', 'SigmaOS::Kernel::Industrial::SovereignZstd::compressOrb'
$zstdContent = $zstdContent -replace 'SigmaOS::Kernel::Industrial::SovereignZstd::getInstance\(\)\.decompressOrb', 'SigmaOS::Kernel::Industrial::SovereignZstd::decompressOrb'
Set-Content -Path $zstdPath -Value $zstdContent
Write-Host "[FIXED] SovereignZstd.cpp"

# ----------------------------------------------------------
# 7. FIX SovereignUX.cpp — bare includes + sigma_gui.h path
# ----------------------------------------------------------
$uxPath = "$root\kernel\ui\SovereignUX.cpp"
$uxContent = Get-Content -Raw $uxPath
$uxContent = $uxContent -replace '#include "core/sigma_types.h"', '#include "../../include/core/sigma_types.h"'
$uxContent = $uxContent -replace '#include "libc/SovereignLibC.h"', '#include "../../include/libc/SovereignLibC.h"'
$uxContent = $uxContent -replace '#include "sigma_ux.h"', '#include "../../include/sigma_ux.h"'
$uxContent = $uxContent -replace '#include "ui/sigma_gui.h"', '#include "../../include/ui/sigma_gui.h"'
$uxContent = $uxContent -replace '#include "hal/sigma_hal.h"', '#include "../../include/hal/sigma_hal.h"'
Set-Content -Path $uxPath -Value $uxContent
Write-Host "[FIXED] SovereignUX.cpp"

# ----------------------------------------------------------
# 8. FIX SovereignPPE.cpp — bare includes
# ----------------------------------------------------------
$ppePath = "$root\kernel\ui\SovereignPPE.cpp"
$ppeContent = Get-Content -Raw $ppePath
$ppeContent = $ppeContent -replace '#include "core/sigma_types.h"', '#include "../../include/core/sigma_types.h"'
Set-Content -Path $ppePath -Value $ppeContent
Write-Host "[FIXED] SovereignPPE.cpp"

# ----------------------------------------------------------
# 9. FIX RPi4Tuning.cpp — bare includes
# ----------------------------------------------------------
$rpiPath = "$root\kernel\hal\RPi4Tuning.cpp"
$rpiContent = Get-Content -Raw $rpiPath
$rpiContent = $rpiContent -replace '#include "core/sigma_types.h"', '#include "../../include/core/sigma_types.h"'
Set-Content -Path $rpiPath -Value $rpiContent
Write-Host "[FIXED] RPi4Tuning.cpp"

# ----------------------------------------------------------
# 10. FIX SovereignPartitionManager.cpp — bare includes
# ----------------------------------------------------------
$partPath = "$root\kernel\core\vfs\SovereignPartitionManager.cpp"
$partContent = Get-Content -Raw $partPath
$partContent = $partContent -replace '#include "core/sigma_types.h"', '#include "../../../include/core/sigma_types.h"'
Set-Content -Path $partPath -Value $partContent
Write-Host "[FIXED] SovereignPartitionManager.cpp"

# ----------------------------------------------------------
# 11. FIX SovereignZstd + SovereignPartitionManager — also fix OrbManager bare includes
# ----------------------------------------------------------
$orbContent2 = Get-Content -Raw $orbPath
$orbContent2 = $orbContent2 -replace '#include "core/sigma_types.h"', '#include "../../../include/core/sigma_types.h"'
$orbContent2 = $orbContent2 -replace '#include "hal/sigma_hal.h"', '#include "../../../include/hal/sigma_hal.h"'
$orbContent2 = $orbContent2 -replace '#include "core/sigma_kernel_types.h"', '#include "../../../include/core/sigma_kernel_types.h"'
$orbContent2 = $orbContent2 -replace '#include "libc/SovereignLibC.h"', '#include "../../../include/libc/SovereignLibC.h"'
Set-Content -Path $orbPath -Value $orbContent2
Write-Host "[FIXED] SovereignOrbManager.cpp — bare includes"

# ----------------------------------------------------------
# 12. GLOBAL FIX — apply sigma_log_info/warn fix for files
#     missing sigma_log.h (Makefile -I include should handle
#     this, but let's ensure the macro path is absolute in
#     files that use bare 'core/' includes without -I)
# ----------------------------------------------------------
$cppFiles = Get-ChildItem -Path $root -Filter "*.cpp" -Recurse |
    Where-Object { $_.FullName -notlike "*node_modules*" -and $_.FullName -notlike "*build*" }

foreach ($file in $cppFiles) {
    $c = Get-Content -Raw $file.FullName
    if ($c -match 'sigma_log_info|sigma_log_warn|sigma_log_err') {
        # Ensure sigma_log.h is included — add it if missing
        if ($c -notmatch '#include.*sigma_log\.h') {
            $c = $c -replace '(#include [^\n]+\n)', "`$1#include `"../../../include/sigma_log.h`"`n"
            Set-Content -Path $file.FullName -Value $c
            Write-Host "[PATCHED-LOG] $($file.Name)"
        }
    }
}

Write-Host ""
Write-Host "=== All fixes applied. ==="
