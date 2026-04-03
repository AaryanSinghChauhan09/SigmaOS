param (
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$CommandArgs
)

$Host.UI.RawUI.WindowTitle = "SigmaOS Omni-Agent"
Write-Host "=============================================" -ForegroundColor Cyan
Write-Host "Σ SIGMA-OS OMNI-AGENT SHELL " -ForegroundColor White -NoNewline
Write-Host "(Autonomous Engine)" -ForegroundColor DarkGray
Write-Host "=============================================" -ForegroundColor Cyan

# Create build dir if missing
if (-not (Test-Path -Path ".\build")) {
    New-Item -ItemType Directory -Path ".\build" | Out-Null
}

Write-Host "[SYS] Compiling Omni-CLI C11 shards native..." -ForegroundColor DarkGray
# Compile the userland CLI and the Kernel Agent Logic
gcc -std=c11 -Wall -O3 .\userland\OmniCLI.c .\kernel\SovereignOmniAgent.c .\kernel\SovereignNetData.c .\kernel\SovereignOrchestrator.c .\kernel\SovereignMCP.c -o .\build\sigma-cli.exe -I. -DSIGMA_WIN32=1

if ($LASTEXITCODE -eq 0) {
    Write-Host "[SYS] Compilation Success. Handoff to Native Binary." -ForegroundColor DarkGray
    Write-Host ""
    if ($CommandArgs) {
        $joinedArgs = $CommandArgs -join " "
        .\build\sigma-cli.exe $joinedArgs
    } else {
        .\build\sigma-cli.exe
    }
} else {
    Write-Host "[FATAL] Critical failure in Omni-Agent C11 compilation vector. Verify MinGW/GCC." -ForegroundColor Red
}
