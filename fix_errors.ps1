$path = "kernel\core\SovereignThermalIQ.cpp"
$content = Get-Content $path
$content[2] = "/*"
$content = $content -replace "ENERGY_STATE_THROTTLED", "1"
Set-Content -Path $path -Value $content

$path = "kernel\core\SovereignOrchestrator.cpp"
$content = Get-Content $path
$classDef = "class SovereignOrchestratorEngine { public: static SovereignOrchestratorEngine& getInstance() { static SovereignOrchestratorEngine instance; return instance; } void init(); void applyPattern(const char* name); void selfHeal(); sigma_u64 getHealCount() { return heal_actions; } private: SovereignOrchestratorEngine() : initialized(0), patterns_applied(0), heal_actions(0) {} sigma_u32 initialized; sigma_u32 patterns_applied; sigma_u64 heal_actions; };"
$content = $content -replace '/\* --- Sovereign Orchestra Engine \(OOP Isolation\) ---\ \*/', $classDef
Set-Content -Path $path -Value $content

$path = "kernel\core\SovereignPQC.cpp"
$content = Get-Content $path
$content = $content -replace "sigma_memset", "sigma_secure_memset"
Set-Content -Path $path -Value $content

$path = "kernel\drivers\industrial_neural_engine.hpp"
$content = Get-Content $path
$content = $content -replace "sigma_bool", "bool"
Set-Content -Path $path -Value $content

$path = "userland\SovereignShell.cpp"
$content = Get-Content $path
$content = $content -replace "sigma_hardened_strcpy", "sigma_strcpy"
$content = $content -replace "sigma_strncmp", "sigma_hardened_strncmp"
Set-Content -Path $path -Value $content

$path = "kernel\core\SovereignNeuralNexus.cpp"
$content = Get-Content $path
$content = $content -replace "sigma_hardened_strcpy", "sigma_strcpy"
Set-Content -Path $path -Value $content

$path = "kernel\core\SovereignProcess.cpp"
$content = Get-Content $path
$content = $content -replace "sigma_hardened_strcpy", "sigma_strcpy"
Set-Content -Path $path -Value $content

$path = "kernel\core\SovereignSnap.cpp"
$content = Get-Content $path
$content = $content -replace "sigma_snap_zone_t", "void*"
Set-Content -Path $path -Value $content

$path = "zenith_desktop.css"
$content = Get-Content $path
$content = $content -replace "backdrop-filter", "-webkit-backdrop-filter"
Set-Content -Path $path -Value $content
