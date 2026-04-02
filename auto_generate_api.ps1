# Σ SIGMAOS: Sovereign Omni-API Auto-Generator
# Scans all kernel and userland shards, extracts global function prototypes, and builds a master unified header.

$ErrorActionPreference = "Stop"

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "       Σ SIGMAOS OMNI-API AUTO-GENERATOR                    " -ForegroundColor Cyan
Write-Host "       Improvising Native C11 Linking Metadata              " -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan

$kernel_files = Get-ChildItem -Path "kernel" -Filter "*.c" 
$output_header = "kernel\SovereignOmniAPI.h"

$header_content = @(
    "// ==============================================================================",
    "// SIGMAOS SOVEREIGN ARCHITECTURE",
    "// CORE API: Sovereign Omni-API (AUTO-GENERATED)",
    "// DEPENDENCIES: NONE (-nostdlib -ffreestanding)",
    "// ==============================================================================",
    "",
    "#ifndef SOVEREIGN_OMNI_API_H",
    "#define SOVEREIGN_OMNI_API_H",
    "",
    "#include `"sigma_kernel_types.h`"",
    ""
)

$functions_found = 0

foreach ($file in $kernel_files) {
    $content = Get-Content $file.FullName
    
    # Simple regex to catch void/int/uint32_t/etc function protocols that aren't static
    $matches = $content | Select-String -Pattern "^(?!(static|//|#|typedef)).+\s+([a-zA-Z0-9_]+)\([^)]*\)\s*\{"
    
    if ($matches.Count -gt 0) {
        $header_content += "// -> Source Shard: $($file.Name)"
        foreach ($m in $matches) {
            # Extract just the signature and add a semicolon
            $signature = $m.Line -replace "\s*\{\s*$", ";"
            $header_content += "extern $signature"
            $functions_found++
        }
        $header_content += ""
    }
}

$header_content += "#endif // SOVEREIGN_OMNI_API_H"
$header_content += ""

$header_content | Set-Content $output_header

Write-Host "[SUCCESS] Scanned $($kernel_files.Count) C sharts." -ForegroundColor Green
Write-Host "[SUCCESS] Extracted $functions_found global sovereign functions." -ForegroundColor Green
Write-Host "[SUCCESS] Generated Unified Header: $output_header" -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Cyan
