$Dirs = @(
    "kernel\bootloader",
    "kernel\memory",
    "kernel\scheduler",
    "kernel\syscalls",
    "drivers\storage",
    "drivers\network",
    "drivers\input",
    "drivers\display",
    "fs\fat32",
    "fs\ext2",
    "tools\shell",
    "tools\utilities",
    "tools\compiler",
    "tests\unit",
    "tests\integration",
    "docs"
)

foreach ($dir in $Dirs) {
    if (-not (Test-Path $dir)) {
        New-Item -ItemType Directory -Path $dir -Force | Out-Null
    }
}

# Move files safely
$Moves = @(
    @{ Src = "kernel\drivers\sigma_e1000.cpp"; Dest = "drivers\network\" },
    @{ Src = "kernel\drivers\sigma_rtl8139.cpp"; Dest = "drivers\network\" },
    @{ Src = "kernel\drivers\sigma_vga.cpp"; Dest = "drivers\display\" },
    @{ Src = "kernel\drivers\sigma_keyboard.cpp"; Dest = "drivers\input\" },
    @{ Src = "kernel\drivers\sigma_mouse.cpp"; Dest = "drivers\input\" },
    @{ Src = "kernel\drivers\sigma_nvme.cpp"; Dest = "drivers\storage\" },
    @{ Src = "kernel\drivers\sigma_ata_driver.cpp"; Dest = "drivers\storage\" },
    @{ Src = "kernel\fs\sigma_fat32.cpp"; Dest = "fs\fat32\" },
    @{ Src = "usr\sigma_sh.cpp"; Dest = "tools\shell\" }
)

foreach ($move in $Moves) {
    if (Test-Path $move.Src) {
        Move-Item -Path $move.Src -Destination $move.Dest -Force
    }
}

if (Test-Path "README.md") { Move-Item -Path "README.md" -Destination "docs\" -Force }
if (Test-Path "CONTRIBUTING.md") { Move-Item -Path "CONTRIBUTING.md" -Destination "docs\" -Force }

Write-Host "Restructuring complete."
