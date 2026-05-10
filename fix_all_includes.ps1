$headers = Get-Content -Raw "headers_map.json" | ConvertFrom-Json

$dirsToCheck = @("kernel", "include", "tests", "lib")
foreach ($dir in $dirsToCheck) {
    $files = Get-ChildItem -Path $dir -Include *.c, *.cpp, *.h, *.hpp -Recurse
    foreach ($file in $files) {
        $content = Get-Content -Raw $file.FullName
        $newContent = $content
        
        # Replace includes
        foreach ($h in $headers) {
            $name = [regex]::Escape($h.Name)
            $rel = $h.RelPath
            $newContent = [regex]::Replace($newContent, "#include\s+[`"<](?:.*?[\\/])?$name[`">]", "#include `"$rel`"")
        }
        
        # Replace other things
        $newContent = $newContent.Replace("sigma_printf", "sigma_log")
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
