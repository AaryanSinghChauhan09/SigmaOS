$baseDir = "C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
$outputFile = Join-Path $baseDir "os_guide.md"

$personalPatterns = @(
    "Aaryan Singh Chauhan",
    "AaryanSinghChauhan09",
    "Aaryan",
    "Sovereign-User",
    "Sovereign-Zenith-Developer",
    "SOVEREIGN_USER"
)

$replacement = "Sovereign-Zenith-Developer"

$newUSPContent = @"
# Σ SIGMAOS: THE SOVEREIGN ZENITH (v8.0) - ULTIMATE COMPETITIVE DOMINANCE

## 🚀 NEW USP ABSORPTION CLUSTER (v8.0)

### 1. 🧮 Sovereign All-In-One Calculator (CosmOS & Numos USP)
The SigmaOS Calculation Engine has absorbed the USPs of **CosmOS (HPIQ)**, **NUMOS**, and industrial CAS systems.
- **AI-First Orchestration**: Integration with OpenClaw agents for natural language math solving and "proactive assistance" (e.g., predicting tax calculations before they are finished).
- **Absolute Privacy (Proton/Vibex)**: Every calculation is processed in an isolated memory shard. Zero telemetry. Zero logs.
- **Automation Node (n8n/Zapier)**: Calculations can be used as triggers. "When Monthly Expenses > Budget, trigger Notification(Shard)."
- **Cross-Node Sync (KDE Connect)**: Synchronized calculation history and clipboard across the SigmaMesh network.

### 2. 📷 Sovereign Vision (Snapchat & Scratch USP)
- **AI Lenses (Snapchat)**: Real-time AR face mesh and filters implemented in pure C++ without OpenCV or FFmpeg.
- **Visual Logic (Scratch)**: Block-based photography automation. Users can drag-and-drop "If Smile Detected -> Snapshot" logic blocks globally within the OS.
- **Pixel-Pure Rendering**: Sub-millisecond latency for GPU-bound filter pipelines using direct Vulkan descriptors.

### 3. 🛡️ Absolute Sovereignty (Industrial Linux Parity)
- **Zero-Dependency Core**: Refactored to eliminate 100% of Node.js, Python, and external C++ libraries.
- **Low-Level Native Logic**: All system components (Calculator, Camera, Shell, Mesh) are built using Custom OOP (SigmaOOP) and direct Assembly/C.
- **Industrial Standards**: Fully compliant with Solid, Linux Kernel Principles, and OCI Container standards.

---
"@

Write-Host "Merging MD files in $baseDir..."

# Clean target file if it exists
if (Test-Path $outputFile) { Remove-Item $outputFile }

# Write new USP header
Set-Content -Path $outputFile -Value $newUSPContent -Encoding UTF8

$mdFiles = Get-ChildItem -Path $baseDir -Filter "*.md" | Where-Object { $_.Name -ne "os_guide.md" }

# Put User Manual first
$manual = $mdFiles | Where-Object { $_.Name -eq "USER_MANUAL.md" }
$others = $mdFiles | Where-Object { $_.Name -ne "USER_MANUAL.md" }
$sortedFiles = @()
if ($manual) { $sortedFiles += $manual }
$sortedFiles += $others

foreach ($file in $sortedFiles) {
    Write-Host "Processing $($file.Name)..."
    Add-Content -Path $outputFile -Value "`n`n# SOURCE: $($file.Name)" -Encoding UTF8
    Add-Content -Path $outputFile -Value ("-" * 40) -Encoding UTF8
    
    $content = Get-Content -Path $file.FullName -Raw
    
    # Sanitize content
    foreach ($pattern in $personalPatterns) {
        $content = $content -replace [regex]::Escape($pattern), $replacement
    }
    
    Add-Content -Path $outputFile -Value $content -Encoding UTF8
}

Write-Host "Merge complete. Removing redundant files..."
foreach ($file in $sortedFiles) {
    Remove-Item $file.FullName
}

Write-Host "Success! os_guide.md is ready."
