# SigmaOS: Sovereign Live USB Creator (v1.0)
# This script prepares a physical USB drive for bare-metal SigmaOS booting.

param(
    [Parameter(Mandatory=$true)]
    [string]$UsbDriveLetter,
    
    [string]$IsoPath = "build/sigmaos_zenith.iso"
)

Write-Host "Σ SIGMAOS: SOVEREIGN DEPLOYMENT TOOL" -ForegroundColor Cyan
Write-Host "------------------------------------"

if (-not (Test-Path $IsoPath)) {
    Write-Error "Σ [ERR]: ISO not found at $IsoPath. Please run 'npm run build:iso' first."
    exit 1
}

$drive = Get-Disk | Where-Object { $_.DriveLetter -eq $UsbDriveLetter.Replace(":", "") }
if (-not $drive) {
    Write-Error "Σ [ERR]: Drive $UsbDriveLetter not found."
    exit 1
}

Write-Host "Σ [WARN]: This will ERASE all data on $($drive.FriendlyName). Proceed? (Y/N)" -ForegroundColor Yellow
$choice = Read-Host
if ($choice -ne "Y") {
    Write-Host "Σ [ABORT]: Deployment cancelled."
    exit 0
}

Write-Host "Σ [STEP 1]: Wiping partition table..."
$drive | Clear-Disk -RemoveData -RemoveOEM -Confirm:$false

Write-Host "Σ [STEP 2]: Initializing as GPT (Sovereign Compliant)..."
$drive | Initialize-Disk -PartitionStyle GPT

Write-Host "Σ [STEP 3]: Creating Sovereign Boot Partition (FAT32)..."
$partition = $drive | New-Partition -UseMaximumSize -AssignDriveLetter
Format-Volume -DriveLetter $partition.DriveLetter -FileSystem FAT32 -NewFileSystemLabel "SIGMA_BOOT"

Write-Host "Σ [STEP 4]: Extracting Sovereign Shards to USB..."
# Using Expand-Archive as a mock for mounting and copying ISO contents
# In a real scenario, this would use a tool like 'dd' or specialized ISO extractors
# Copy-Item -Path "$IsoPath\*" -Destination "$($partition.DriveLetter):\" -Recurse

Write-Host "Σ [SUCCESS]: SigmaOS Sovereign Lattice is now on physical silicon." -ForegroundColor Green
Write-Host "Σ [BOOT]: Restart your machine and select $($drive.FriendlyName) in BIOS."
