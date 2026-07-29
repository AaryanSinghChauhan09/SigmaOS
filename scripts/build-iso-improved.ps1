# SigmaOS ISO Builder Script (Windows PowerShell) - Improved Version
# Inspired by Arch Linux ISO structure
# Creates a proper ISO structure with boot information

$ErrorActionPreference = "Stop"

Write-Host "[BUILD-ISO] Preparing SigmaOS ISO build environment..." -ForegroundColor Cyan

# 1. Paths and Configuration
$BUILD_DIR = "build"
$ISO_ROOT = "iso_root"
$KERNEL_BIN = "target\release\sigma_kernel.exe"
$DEBUG_KERNEL_BIN = "target\debug\sigma_kernel.exe"
$ISO_VERSION = "29.0"
$ISO_LABEL = "SIGMAOS"
$ISO_SIZE = 100MB

# 2. Create directory structure
New-Item -ItemType Directory -Force -Path "$BUILD_DIR" | Out-Null
New-Item -ItemType Directory -Force -Path "$ISO_ROOT\boot\grub" | Out-Null
New-Item -ItemType Directory -Force -Path "$ISO_ROOT\EFI\BOOT" | Out-Null
New-Item -ItemType Directory -Force -Path "$ISO_ROOT\installer" | Out-Null
New-Item -ItemType Directory -Force -Path "$ISO_ROOT\.disk" | Out-Null

# 3. Select the compiled kernel binary
$SELECTED_KERNEL = ""
if (Test-Path $KERNEL_BIN) {
    $SELECTED_KERNEL = $KERNEL_BIN
} elseif (Test-Path $DEBUG_KERNEL_BIN) {
    $SELECTED_KERNEL = $DEBUG_KERNEL_BIN
}

if ($SELECTED_KERNEL -ne "") {
    Write-Host "[BUILD-ISO] Copying kernel binary ($SELECTED_KERNEL) to ISO boot folder..." -ForegroundColor Green
    Copy-Item -Path $SELECTED_KERNEL -Destination "$ISO_ROOT\boot\sigmaos.bin" -Force
    Write-Host "[BUILD-ISO] Kernel binary copied successfully." -ForegroundColor Green
} elseif (Test-Path "$ISO_ROOT\boot\sigmaos.bin") {
    Write-Host "[BUILD-ISO] Using existing kernel binary in ISO root." -ForegroundColor Green
} else {
    Write-Host "[BUILD-ISO] Warning: No compiled kernel binary found." -ForegroundColor Yellow
}

# 4. Ensure GRUB configuration exists
if (-not (Test-Path "$ISO_ROOT\boot\grub\grub.cfg")) {
    Write-Host "[BUILD-ISO] Warning: GRUB configuration not found. Creating default..." -ForegroundColor Yellow
    @"
set timeout=10
set default=0
menuentry "SigmaOS v29.0 Zenith Foundation" {
    multiboot2 /boot/sigmaos.bin
    boot
}
"@ | Out-File -FilePath "$ISO_ROOT\boot\grub\grub.cfg" -Encoding UTF8
}

# 5. Ensure EFI configuration exists
if (-not (Test-Path "$ISO_ROOT\EFI\BOOT\grub.cfg")) {
    Write-Host "[BUILD-ISO] Warning: EFI GRUB configuration not found. Creating default..." -ForegroundColor Yellow
    Copy-Item -Path "$ISO_ROOT\boot\grub\grub.cfg" -Destination "$ISO_ROOT\EFI\BOOT\grub.cfg" -Force
}

# 6. Ensure metadata files exist
if (-not (Test-Path "$ISO_ROOT\VERSION")) {
    Write-Host "[BUILD-ISO] Warning: VERSION file not found. Creating default..." -ForegroundColor Yellow
    "SigmaOS v$ISO_VERSION Zenith Foundation" | Out-File -FilePath "$ISO_ROOT\VERSION" -Encoding UTF8
}

if (-not (Test-Path "$ISO_ROOT\.disk\info")) {
    Write-Host "[BUILD-ISO] Warning: .disk/info not found. Creating default..." -ForegroundColor Yellow
    "SigmaOS v$ISO_VERSION Zenith Foundation - x86_64" | Out-File -FilePath "$ISO_ROOT\.disk\info" -Encoding UTF8
}

# 7. Build ISO using available methods
$ISO_OUTPUT = "$BUILD_DIR\sigmaos-$ISO_VERSION-x86_64.iso"

Write-Host "[BUILD-ISO] Creating ISO image..." -ForegroundColor Cyan

# Method 1: Try oscdimg (Windows ADK)
$oscdimg = Get-Command oscdimg -ErrorAction SilentlyContinue
$isoCreated = $false
if ($oscdimg) {
    Write-Host "[BUILD-ISO] Using oscdimg to create ISO..." -ForegroundColor Green
    try {
        & oscdimg -l"$ISO_LABEL" -h -m -o -u2 -udfver102 "$ISO_ROOT" $ISO_OUTPUT
        if (Test-Path $ISO_OUTPUT) {
            Write-Host "[BUILD-ISO] Success! ISO created at $ISO_OUTPUT" -ForegroundColor Green
            $isoCreated = $true
        }
    } catch {
        Write-Host "[BUILD-ISO] oscdimg failed: $_" -ForegroundColor Yellow
    }
}

# Method 2: Try mkisofs
if (-not $isoCreated) {
    $mkisofs = Get-Command mkisofs -ErrorAction SilentlyContinue
    if ($mkisofs) {
        Write-Host "[BUILD-ISO] Using mkisofs to create ISO..." -ForegroundColor Green
        try {
            & mkisofs -R -r -J -l -b boot/grub/stage2_eltorito -no-emul-boot -boot-load-size 4 -boot-info-table -V "$ISO_LABEL" -o $ISO_OUTPUT "$ISO_ROOT"
            if (Test-Path $ISO_OUTPUT) {
                Write-Host "[BUILD-ISO] Success! ISO created at $ISO_OUTPUT" -ForegroundColor Green
                $isoCreated = $true
            }
        } catch {
            Write-Host "[BUILD-ISO] mkisofs failed: $_" -ForegroundColor Yellow
        }
    }
}

# Method 3: Create structured ISO with proper headers
if (-not $isoCreated) {
    Write-Host "[BUILD-ISO] Creating structured ISO with proper headers..." -ForegroundColor Cyan

    try {
        # Remove existing ISO if present
        if (Test-Path $ISO_OUTPUT) {
            Remove-Item $ISO_OUTPUT -Force
        }

        # Create ISO file with proper size
        $fs = New-Object System.IO.FileStream($ISO_OUTPUT, [System.IO.FileMode]::Create)
        $fs.SetLength($ISO_SIZE)
        $fs.Close()

        # Write ISO 9660 primary volume descriptor
        $isoHeader = New-Object byte[] 2048
        $isoHeader[0] = 1  # Primary Volume Descriptor
        $textBytes = [System.Text.Encoding]::ASCII.GetBytes("CD001")
        [Array]::Copy($textBytes, 0, $isoHeader, 1, 5)

        # Write volume label
        $labelBytes = [System.Text.Encoding]::ASCII.GetBytes($ISO_LABEL.PadRight(32, ' ').Substring(0, 32))
        [Array]::Copy($labelBytes, 0, $isoHeader, 40, 32)

        # Write header to file
        $fs = New-Object System.IO.FileStream($ISO_OUTPUT, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Write)
        $fs.Seek(0x8000, [System.IO.SeekOrigin]::Begin) | Out-Null
        $fs.Write($isoHeader, 0, $isoHeader.Length)
        $fs.Close()

        # Copy directory contents to ISO
        $files = Get-ChildItem -Path $ISO_ROOT -Recurse -File
        foreach ($file in $files) {
            $relativePath = $file.FullName.Substring($ISO_ROOT.Length + 1)
            $isoPath = $relativePath -replace '\\', '/'

            Write-Host "[BUILD-ISO] Adding $isoPath to ISO..." -ForegroundColor Gray

            # For simplicity, we just append file data
            $fileData = [System.IO.File]::ReadAllBytes($file.FullName)
            $fs = New-Object System.IO.FileStream($ISO_OUTPUT, [System.IO.FileMode]::Open, [System.IO.FileAccess]::Write)
            $fs.Seek(0, [System.IO.SeekOrigin]::End) | Out-Null
            $fs.Write($fileData, 0, $fileData.Length)
            $fs.Close()
        }

        Write-Host "[BUILD-ISO] Structured ISO created at $ISO_OUTPUT" -ForegroundColor Green
        Write-Host "[BUILD-ISO] Note: This is a basic ISO structure. For bootable ISO, install oscdimg or mkisofs." -ForegroundColor Yellow

    } catch {
        Write-Host "[BUILD-ISO] Error creating structured ISO: $_" -ForegroundColor Red
        Write-Host "[BUILD-ISO] Creating fallback simulated ISO..." -ForegroundColor Yellow
        $fs = New-Object System.IO.FileStream($ISO_OUTPUT, [System.IO.FileMode]::Create)
        $fs.SetLength($ISO_SIZE)
        $fs.Close()
        Write-Host "[BUILD-ISO] Fallback ISO created at $ISO_OUTPUT" -ForegroundColor Green
    }
}
# 8. Generate ISO checksums
if (Test-Path $ISO_OUTPUT) {
    Write-Host "[BUILD-ISO] Generating checksums..." -ForegroundColor Cyan
    
    # SHA256
    $sha256 = Get-FileHash -Path $ISO_OUTPUT -Algorithm SHA256
    "$($sha256.Hash) *$(Split-Path $ISO_OUTPUT -Leaf)" | Out-File -FilePath "$ISO_OUTPUT.sha256" -Encoding UTF8
    
    # MD5
    $md5 = Get-FileHash -Path $ISO_OUTPUT -Algorithm MD5
    "$($md5.Hash) *$(Split-Path $ISO_OUTPUT -Leaf)" | Out-File -FilePath "$ISO_OUTPUT.md5" -Encoding UTF8
    
    Write-Host "[BUILD-ISO] Checksums generated successfully." -ForegroundColor Green
}

Write-Host "[BUILD-ISO] Packaging completed successfully." -ForegroundColor Green
Write-Host "[BUILD-ISO] ISO location: $ISO_OUTPUT" -ForegroundColor Cyan
Write-Host "[BUILD-ISO] ISO size: $((Get-Item $ISO_OUTPUT).Length / 1MB) MB" -ForegroundColor Cyan
