$dirsToCheck = @("kernel", "include", "tests", "lib")
foreach ($dir in $dirsToCheck) {
    $files = Get-ChildItem -Path $dir -Include *.c, *.cpp, *.h, *.hpp -Recurse
    foreach ($file in $files) {
        $content = Get-Content -Raw $file.FullName
        $newContent = $content
        
        # specific replaces
        $newContent = $newContent -replace '#include\s+"(?:.*[/\\])?sigma_types\.h"', '#include "core/sigma_types.h"'
        $newContent = $newContent -replace '#include\s+<(?:.*[/\\])?sigma_types\.h>', '#include "core/sigma_types.h"'
        
        $newContent = $newContent -replace '#include\s+"(?:.*[/\\])?sigma_hal\.h"', '#include "hal/sigma_hal.h"'
        $newContent = $newContent -replace '#include\s+<(?:.*[/\\])?sigma_hal\.h>', '#include "hal/sigma_hal.h"'
        
        $newContent = $newContent -replace '#include\s+"(?:.*[/\\])?SovereignLibC\.h"', '#include "libc/SovereignLibC.h"'
        $newContent = $newContent -replace '#include\s+<(?:.*[/\\])?SovereignLibC\.h>', '#include "libc/SovereignLibC.h"'
        
        $newContent = $newContent -replace '#include\s+"(?:.*[/\\])?sigma_pqc\.h"', '#include "security/sigma_pqc.h"'
        $newContent = $newContent -replace '#include\s+<(?:.*[/\\])?sigma_pqc\.h>', '#include "security/sigma_pqc.h"'
        
        $newContent = $newContent -replace '#include\s+"(?:.*[/\\])?sigma_kernel_types\.h"', '#include "core/sigma_kernel_types.h"'
        
        # fix SovereignSnap.cpp malformed include if any
        $newContent = $newContent -replace '#include\s*$', '' # remove dangling empty include if present

        # ensure sigma_types.h is included if sigma_u32 is used but not included
        if ($newContent -match 'sigma_u32|sigma_u64|sigma_u8|sigma_u16|sigma_size_t') {
            if ($newContent -notmatch 'sigma_types\.h') {
                $newContent = "#include `"core/sigma_types.h`"`n" + $newContent
            }
        }
        
        # fix sigma_printf to sigma_log
        $newContent = $newContent -replace '\bsigma_printf\b', 'sigma_log'
        
        # fix namespace issues
        $newContent = [regex]::Replace($newContent, '(?<!SigmaOS::Kernel::Security::)SovereignSandboxEngine', 'SigmaOS::Kernel::Security::SovereignSandboxEngine')
        $newContent = [regex]::Replace($newContent, '(?<!SigmaOS::Kernel::Security::)SovereignPQCEngine', 'SigmaOS::Kernel::Security::SovereignPQCEngine')
        $newContent = [regex]::Replace($newContent, '(?<!SigmaOS::Kernel::Syscall::)SovereignSyscallEngine', 'SigmaOS::Kernel::Syscall::SovereignSyscallEngine')
        $newContent = [regex]::Replace($newContent, '(?<!SigmaOS::Kernel::HAL::)SovereignSMPEngine', 'SigmaOS::Kernel::HAL::SovereignSMPEngine')
        $newContent = [regex]::Replace($newContent, '(?<!SigmaOS::Kernel::AI::)SovereignAISchedEngine', 'SigmaOS::Kernel::AI::SovereignAISchedEngine')

        if ($content -ne $newContent) {
            Write-Host "Updating $($file.FullName)"
            $newContent | Set-Content $file.FullName -NoNewline
        }
    }
}
