# Updated fix_errors.ps1 for modularized paths
$path = "kernel\core\SovereignThermalIQ.cpp"
if (Test-Path $path) {
    $content = Get-Content $path
    $content[2] = "/*"
    $content = $content -replace "ENERGY_STATE_THROTTLED", "1"
    Set-Content -Path $path -Value $content
}

$path = "kernel\core\SovereignOrchestrator.cpp"
if (Test-Path $path) {
    $content = Get-Content $path
    $classDef = "class SovereignOrchestratorEngine { public: static SovereignOrchestratorEngine& getInstance() { static SovereignOrchestratorEngine instance; return instance; } void init(); void applyPattern(const char* name); void selfHeal(); sigma_u64 getHealCount() { return heal_actions; } private: SovereignOrchestratorEngine() : initialized(0), patterns_applied(0), heal_actions(0) {} sigma_u32 initialized; sigma_u32 patterns_applied; sigma_u64 heal_actions; };"
    $content = $content -replace '/\* --- Sovereign Orchestra Engine \(OOP Isolation\) ---\ \*/', $classDef
    Set-Content -Path $path -Value $content
}

$path = "kernel\core\security\crypto\SovereignPQC.cpp"
if (Test-Path $path) {
    $content = Get-Content $path
    $content = $content -replace "sigma_memset", "sigma_secure_memset"
    Set-Content -Path $path -Value $content
}

$path = "ui/themes/zenith_desktop.css"
if (Test-Path $path) {
    $content = Get-Content $path
    $content = $content -replace "backdrop-filter", "-webkit-backdrop-filter"
    Set-Content -Path $path -Value $content
}

# Add more fixes as needed
