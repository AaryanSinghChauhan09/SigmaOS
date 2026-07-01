$files = Get-ChildItem -Path drivers, kernel -Recurse -Include *.cpp, *.h
foreach ($file in $files) {
    $content = Get-Content -Path $file.FullName -Raw
    $original = $content
    
    # 1. Static member accessed through instance
    $content = $content -replace '([A-Za-z0-9_:]+)::getInstance\(\)\.([A-Za-z0-9_]+)\(', '$1::$2('
    
    # 2. Method can be made static
    $methods = @('initDevice', 'runExecutable', 'solvePhysicsProblem', 'setSleepState', 'optimizeForBattery', 'runContainer', 'beginTransaction', 'commitTransaction', 'rollbackTransaction', 'initializeDXVK', 'secureControlPlane', 'secureBootAMI', 'loadAll', 'initializePersistence', 'monitorCompliance', 'setQuotas', 'enforceQuotas', 'bypassAbstraction', 'verifyDriverCompliance')
    foreach ($method in $methods) {
        $content = $content -replace "(?m)^(\s*)(void|bool)\s+$method\(", "`$1static `$2 $method("
    }
    
    if ($content -ne $original) {
        Set-Content -Path $file.FullName -Value $content -NoNewline
    }
}
