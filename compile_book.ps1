$OutputFile = "SIGMAPEDIA_THE_SOVEREIGN_BOOK.md"
$IntroText = @"
# SIGMAPEDIA: THE SOVEREIGN BOOK OF SIGMA OS
*The Ultimate Professional Reference Manual for the Zero-Dependency Era*

## FOREWORD: CRUSHING THE DISTROS
SigmaOS completely fundamentally replaces and dominates Linux architectures (Ubuntu, Arch, NixOS). Where they rely on inflated `libc` wrappers, systemd bloat, and POSIX threads, SigmaOS runs on extremely pure, low-level Assembly (MMU page-walkers), Native Rust (`#[no_std]` APIC/Parallel Processing engines, Zero-trust Security and Networking layers), and Bare-Metal C (Custom UEFI sequences). There is ZERO 3rd-party integration. Every feature is written autonomously.

## 📥 HOW TO DOWNLOAD
1. Locate the latest stable `.iso` from the SigmaOS GitHub Registry: `https://github.com/AaryanSinghChauhan09/SigmaOS`.
2. Ensure you bypass legacy boot loaders; Sigma uses its own Pure C UEFI bootstraps.

## 🚀 HOW TO USE (NATIVE / BROWSER / VM / LIVE)
- **Browser Executable:** Native WASM bindings securely map SigmaOS processes to any Chromium browser.
- **Bare-Metal Integration:** Flash the ISO using standard tools to run via `efi_main.c` directly on local physical hardware.
- **Virtualization:** Sigma Container Broker dynamically emulates environment mappings allowing Live Boot and Portable OS routing.

## 🔗 HOW TO SHARE
- SigmaOS utilizes `Sigma Cloud` and `Sigma Store` (via `.spkg` containers). You can instantly bundle and share an exact atomic state of your OS via the proprietary Omni-Share network.

---

"@

# Initialize the book
Set-Content -Path $OutputFile -Value $IntroText

# Files to merge
$MarkdownFiles = @(
    "README.md",
    "SIGMAOS_USER_MANUAL.md",
    "SIGMAOS_SUBSTITUTIONS.md",
    "suggestions.md",
    "DOCS\MISSING_COMPONENTS.md",
    "DOCS\OS_PRINCIPLES_EVOLUTION.md",
    "DOCS\SIGMA_LIBC.md",
    "DOCS\LAUNCH_READINESS_REPORT.md",
    "workflows\linux-parity.md"
)

foreach ($File in $MarkdownFiles) {
    if (Test-Path $File) {
        $Content = Get-Content -Path $File -Raw
        Add-Content -Path $OutputFile -Value "`n`n<br/>`n<div style='page-break-after: always;'></div>`n`n"
        Add-Content -Path $OutputFile -Value "## --- MODULE: $($File.ToUpper()) ---`n"
        Add-Content -Path $OutputFile -Value $Content
        Write-Host "Appended $File"
    } else {
        Write-Host "Skipping $File (Not Found)"
    }
}

Write-Host "SIGMAPEDIA book compiled successfully."
