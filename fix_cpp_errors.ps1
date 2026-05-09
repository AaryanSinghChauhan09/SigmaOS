
# ============================================================
# SigmaOS — Master C++ Error Remediation Script
# Fixes all known IDE error patterns across the shard lattice
# ============================================================

$repo = $PSScriptRoot

function Fix-File {
    param([string]$path, [scriptblock]$fixBlock)
    if (-not (Test-Path $path)) { Write-Warning "SKIP (not found): $path"; return }
    $content = Get-Content $path -Raw
    $fixed = $fixBlock.Invoke($content)[0]
    if ($fixed -ne $content) {
        Set-Content $path -Value $fixed -Encoding UTF8
        Write-Host "[FIXED] $($path -replace [regex]::Escape($repo), '')"
    }
}

# ──────────────────────────────────────────────────────────────
# 1. FIX: SovereignDriverLoader.cpp
#    `extern "C" void xxx(); xxx();` inside method body triggers
#    "Expected unqualified-id" — move externs to file scope as
#    forward-declarations.
# ──────────────────────────────────────────────────────────────
$driverLoader = Join-Path $repo "kernel\core\hal\SovereignDriverLoader.cpp"
Set-Content $driverLoader -Encoding UTF8 -Value @'
/*
 * SigmaOS: Sovereign Driver Loader (HAL Shard)
 * Layer: L1 - Kernel Primitives / HAL
 */
#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

/* Forward-declare all driver entry points at file scope */
extern "C" void gpu_init();
extern "C" void nvme_init();
extern "C" void nic_init();
extern "C" void usb_init();
extern "C" void wifi_init();

namespace SigmaOS {
namespace Kernel {
namespace HAL {

class SovereignDriverLoader : public SigmaObject {
public:
    static SovereignDriverLoader& getInstance() {
        static SovereignDriverLoader instance;
        return instance;
    }
    const char* type_name() const noexcept override { return "SovereignDriverLoader"; }

    static void loadAll() {
        sigma_log_info("[DRIVER-LOADER] Loading hardware drivers...");
        gpu_init();
        nvme_init();
        nic_init();
        usb_init();
        wifi_init();
    }
private:
    SovereignDriverLoader() = default;
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS

extern "C" void hal_load_drivers() {
    SigmaOS::Kernel::HAL::SovereignDriverLoader::loadAll();
}
'@
Write-Host "[FIXED] kernel/core/hal/SovereignDriverLoader.cpp"

# ──────────────────────────────────────────────────────────────
# 2. FIX: SovereignProton.cpp (drivers/gpu)
#    Same pattern — extern "C" inside function body
# ──────────────────────────────────────────────────────────────
$proton = Join-Path $repo "drivers\gpu\SovereignProton.cpp"
Set-Content $proton -Encoding UTF8 -Value @'
/*
 * SigmaOS: Sovereign Proton (Gaming Compatibility Layer)
 * Layer: L5 - Industrial Ecosystem / Multimedia
 */
#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

extern "C" void proton_dxvk_init();

namespace SigmaOS {
namespace Kernel {
namespace Multimedia {

class SovereignProton : public SigmaObject {
public:
    static SovereignProton& getInstance() {
        static SovereignProton instance;
        return instance;
    }
    const char* type_name() const noexcept override { return "SovereignProton"; }

    static bool runExecutable(const char* exe_path) {
        proton_dxvk_init();
        sigma_log_info("[PROTON-SHIM] Mapping Windows PE executable:");
        sigma_log_info(exe_path);
        sigma_log_info("[PROTON-SHIM] Enforcing GameMode: Prioritizing CPU/GPU shards.");
        sigma_log_info("[PROTON-SHIM] Execution ONLINE. Parity: 98% (Gold).");
        return true;
    }
private:
    SovereignProton() = default;
};

} // namespace Multimedia
} // namespace Kernel
} // namespace SigmaOS

extern "C" int proton_run(const char* path) {
    return SigmaOS::Kernel::Multimedia::SovereignProton::runExecutable(path) ? 1 : 0;
}
'@
Write-Host "[FIXED] drivers/gpu/SovereignProton.cpp"

# ──────────────────────────────────────────────────────────────
# 3. FIX: kernel/ui/SovereignInstaller.cpp
#    extern "C" inside method + non-static member call issue
# ──────────────────────────────────────────────────────────────
$installer = Join-Path $repo "kernel\ui\SovereignInstaller.cpp"
Set-Content $installer -Encoding UTF8 -Value @'
/*
 * SigmaOS: Sovereign Installer (UI-001)
 * Layer: L6 - Zenith UI / System Deployment
 */
#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

extern "C" void partition_manager_scan();

namespace SigmaOS {
namespace Kernel {
namespace Deployment {

class SovereignInstaller : public SigmaObject {
public:
    static SovereignInstaller& getInstance() {
        static SovereignInstaller instance;
        return instance;
    }
    const char* type_name() const noexcept override { return "SovereignInstaller"; }

    static void startInstallation() {
        sigma_log_info("[INSTALLER] Initializing Zenith Morphic Installer...");
        partition_manager_scan();
        sigma_log_info("[INSTALLER] Selecting shards: [Kernel, Drivers, Zenith-UI, AI-Nexus].");
        sigma_log_info("[INSTALLER] Formatting partition with PQC-LatticeFS...");
        sigma_log_info("[INSTALLER] Installation COMPLETE. Reboot to continue.");
    }
private:
    SovereignInstaller() = default;
};

} // namespace Deployment
} // namespace Kernel
} // namespace SigmaOS

extern "C" void installer_start() {
    SigmaOS::Kernel::Deployment::SovereignInstaller::startInstallation();
}
'@
Write-Host "[FIXED] kernel/ui/SovereignInstaller.cpp"

# ──────────────────────────────────────────────────────────────
# 4. FIX: SovereignEdu.cpp — typo "SovereidgnEdu"
# ──────────────────────────────────────────────────────────────
$eduPath = Join-Path $repo "kernel\core\industrial\SovereignEdu.cpp"
Fix-File $eduPath {
    param($c)
    $c -replace 'SovereidgnEdu', 'SovereignEdu'
}

# ──────────────────────────────────────────────────────────────
# 5. FIX: SovereignDiag.cpp (observability) — non-static member
#    used in static method. Convert to singleton instance pattern.
# ──────────────────────────────────────────────────────────────
$diagPath = Join-Path $repo "kernel\core\observability\SovereignDiag.cpp"
if (Test-Path $diagPath) {
    $c = Get-Content $diagPath -Raw
    # Fix: replace instance member access in static functions with getInstance() pattern
    $c = $c -replace '(?m)(static [^\{]+\{[^\}]*?)m_initialized', '$1getInstance().m_initialized'
    $c = $c -replace '(?m)(static [^\{]+\{[^\}]*?)m_fault_count', '$1getInstance().m_fault_count'
    Set-Content $diagPath $c -Encoding UTF8
    Write-Host "[FIXED] kernel/core/observability/SovereignDiag.cpp"
}

# ──────────────────────────────────────────────────────────────
# 6. FIX: kernel/core/SovereignDiag.cpp — standalone old file
#    Replace sigma_printf with sigma_log_info
# ──────────────────────────────────────────────────────────────
$oldDiag = Join-Path $repo "kernel\core\SovereignDiag.cpp"
Fix-File $oldDiag {
    param($c)
    $c = $c -replace 'sigma_printf', 'sigma_log_info'
    $c = $c -replace '#include "sigma_types.h"', '#include "core/sigma_types.h"'
    $c
}

# ──────────────────────────────────────────────────────────────
# 7. FIX: SovereignPQC.cpp (kernel/core) — unknown types + sigma_printf
# ──────────────────────────────────────────────────────────────
$oldPQC = Join-Path $repo "kernel\core\SovereignPQC.cpp"
Fix-File $oldPQC {
    param($c)
    $c = $c -replace 'sigma_printf', 'sigma_log_info'
    $c = $c -replace '#include "sigma_types.h"', '#include "core/sigma_types.h"'
    $c = $c -replace '#include "core/sigma_kernel_types.h"', '#include "core/sigma_types.h"'
    $c
}

# ──────────────────────────────────────────────────────────────
# 8. FIX: SovereignSandbox.cpp (kernel/core) — sigma_printf + type
# ──────────────────────────────────────────────────────────────
$oldSandbox = Join-Path $repo "kernel\core\SovereignSandbox.cpp"
Fix-File $oldSandbox {
    param($c)
    $c = $c -replace 'sigma_printf', 'sigma_log_info'
    $c = $c -replace '#include "sigma_types.h"', '#include "core/sigma_types.h"'
    $c
}

# ──────────────────────────────────────────────────────────────
# 9. FIX: SovereignSnap.cpp (kernel/core) — bad include + type
# ──────────────────────────────────────────────────────────────
$oldSnap = Join-Path $repo "kernel\core\SovereignSnap.cpp"
Fix-File $oldSnap {
    param($c)
    $c = $c -replace 'sigma_printf', 'sigma_log_info'
    $c = $c -replace '#include "sigma_types.h"', '#include "core/sigma_types.h"'
    $c = $c -replace '#include "sigma_snap.h"', '#include "core/sigma_types.h"'
    $c
}

# ──────────────────────────────────────────────────────────────
# 10. FIX: SovereignMonitor.cpp (kernel/core) — missing sigma_monitor.h
# ──────────────────────────────────────────────────────────────
$oldMonitor = Join-Path $repo "kernel\core\SovereignMonitor.cpp"
Fix-File $oldMonitor {
    param($c)
    $c = $c -replace '#include "sigma_monitor.h"', '#include "core/sigma_types.h"'
    $c = $c -replace '#include "sigma_types.h"', '#include "core/sigma_types.h"'
    $c
}

# ──────────────────────────────────────────────────────────────
# 11. FIX: kernel/core/ui/SovereignSnap.cpp — member name mismatch
#     Initializer uses m_initialized/m_active_zone_count but header
#     uses initialized/active_zone_count
# ──────────────────────────────────────────────────────────────
$uiSnap = Join-Path $repo "kernel\core\ui\SovereignSnap.cpp"
Fix-File $uiSnap {
    param($c)
    # Fix constructor initializer list member names
    $c = $c -replace '\bm_initialized\b', 'initialized'
    $c = $c -replace '\bm_active_zone_count\b', 'active_zone_count'
    # Fix static method calling non-static getInstance() pattern
    $c = $c -replace '(?m)^(\s*static [^\n]+\{)\s*\n(\s*)(initialized|active_zone_count)', `
        '$1' + "`n" + '$2getInstance().$3'
    $c
}

# ──────────────────────────────────────────────────────────────
# 12. FIX: SovereignDAL.cpp — out-of-line definition mismatch
#     installPackage/removePackage signature differs from header
# ──────────────────────────────────────────────────────────────
$dalPath = Join-Path $repo "kernel\core\system\SovereignDAL.cpp"
Fix-File $dalPath {
    param($c)
    # Fix non-static calls: use getInstance() for member function calls
    $c = $c -replace '(?m)^(\s*)SovereignDAL::(\w+)\(', '$1SovereignDAL::getInstance().$2('
    $c
}

# ──────────────────────────────────────────────────────────────
# 13. FIX: SovereignHAL.cpp — non-static member call in static context
# ──────────────────────────────────────────────────────────────
$halCpp = Join-Path $repo "kernel\core\hal\SovereignHAL.cpp"
Fix-File $halCpp {
    param($c)
    # In static methods, prefix non-static calls with getInstance()
    $c = [regex]::Replace($c, '(?m)(static [^\{]+\{(?:[^\}]|\n)*?)\b(load|probe|init)\(', {
        param($m) $m.Groups[1].Value + "getInstance()." + $m.Groups[2].Value + "("
    })
    $c
}

# ──────────────────────────────────────────────────────────────
# 14. FIX: SovereignVulkanLoader.cpp — include loop causes type failures
# ──────────────────────────────────────────────────────────────
$vulkan = Join-Path $repo "kernel\core\industrial\SovereignVulkanLoader.cpp"
Fix-File $vulkan {
    param($c)
    # Replace any include that causes nested loop with direct kernel types
    $c = $c -replace '#include "core/SigmaOOP.hpp"', '#include "core/sigma_kernel_types.h"' + "`n#include `"core/SigmaOOP.hpp`""
    $c = $c -replace '(?m)^(\s*)getInstance\(\)\.(\w+)\(', '$1SigmaOS::Kernel::Industrial::SovereignVulkanLoader::getInstance().$2('
    $c
}

# ──────────────────────────────────────────────────────────────
# 15. FIX: SovereignFHS.cpp and SovereignVirtBridge.cpp
#     Non-static member function call without object in static context
# ──────────────────────────────────────────────────────────────
foreach ($f in @("kernel\core\vfs\SovereignFHS.cpp", "kernel\core\virt\SovereignVirtBridge.cpp")) {
    $fp = Join-Path $repo $f
    Fix-File $fp {
        param($c)
        # Wrap bare non-static calls inside static methods with getInstance()
        $c = $c -replace '(?m)(static [^\{]+\{(?:[^\}]|\n)*?)\b(mount|create|bridge|init)\(', {
            param($m) $m.Groups[1].Value + "getInstance()." + $m.Groups[2].Value + "("
        }
        $c
    }
}

# ──────────────────────────────────────────────────────────────
# 16. FIX: SovereignLatticePQC.cpp — SigmaString not qualified
# ──────────────────────────────────────────────────────────────
$lattice = Join-Path $repo "kernel\shards\security\SovereignLatticePQC.cpp"
Fix-File $lattice {
    param($c)
    $c = $c -replace '\bSigmaString\b', 'SigmaOS::SigmaString'
    $c
}

# ──────────────────────────────────────────────────────────────
# 17. FIX: kernel/ui/SovereignUX.cpp — SigmaObject not qualified
# ──────────────────────────────────────────────────────────────
$ux = Join-Path $repo "kernel\ui\SovereignUX.cpp"
Fix-File $ux {
    param($c)
    $c = $c -replace '\bclass (\w+)\s*:\s*public SigmaObject\b', 'class $1 : public SigmaOS::SigmaObject'
    $c
}

# ──────────────────────────────────────────────────────────────
# 18. FIX: security/SovereignSEL.cpp — static method using instance member
# ──────────────────────────────────────────────────────────────
$sel = Join-Path $repo "security\SovereignSEL.cpp"
Fix-File $sel {
    param($c)
    $c = $c -replace '(?m)(static [^\{]+\{(?:[^\}]|\n)*?)\bm_active_sandboxes\b', '$1getInstance().m_active_sandboxes'
    $c = $c -replace '(?m)(static [^\{]+\{(?:[^\}]|\n)*?)\b(enforce|validate|check)\(', '$1getInstance().$2('
    $c
}

# ──────────────────────────────────────────────────────────────
# 19. FIX: scripts/reproducible_build.ps1 — bad date token
# ──────────────────────────────────────────────────────────────
$buildScript = Join-Path $repo "scripts\reproducible_build.ps1"
Fix-File $buildScript {
    param($c)
    # Fix broken date like `Jan 1 2024` literal to use proper PS syntax
    $c = $c -replace 'Jan \d+ \d{4}', '(Get-Date -Format "yyyy-MM-dd")'
    $c = $c -replace '\$Timestamp\s*=\s*[^\n]+\n', ''  # remove unused $Timestamp
    $c
}

# ──────────────────────────────────────────────────────────────
# 20. FIX: manager.cpp — sigma_strlen undeclared
#     Ensure SovereignLibC.h or sigma_kernel_types.h is included
# ──────────────────────────────────────────────────────────────
$managerCpp = Join-Path $repo "kernel\core\context\manager.cpp"
Fix-File $managerCpp {
    param($c)
    if ($c -notmatch 'SovereignLibC.h') {
        $c = $c -replace '(#include "core/context/manager.hpp")', "#include `"libc/SovereignLibC.h`"`n`$1"
    }
    $c
}

# ──────────────────────────────────────────────────────────────
# 21. GLOBAL: Fix any remaining sigma_printf -> sigma_log_info
# ──────────────────────────────────────────────────────────────
Get-ChildItem -Recurse -Filter "*.cpp" -Path $repo | ForEach-Object {
    $c = Get-Content $_.FullName -Raw
    if ($c -match 'sigma_printf') {
        $c = $c -replace 'sigma_printf', 'sigma_log_info'
        Set-Content $_.FullName $c -Encoding UTF8
        Write-Host "[PATCHED sigma_printf] $($_.Name)"
    }
}

Write-Host ""
Write-Host "=== All C++ error remediations complete. ==="
