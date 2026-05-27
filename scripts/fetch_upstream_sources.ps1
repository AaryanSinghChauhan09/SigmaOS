# fetch_upstream_sources.ps1
# Pulls upstream sources to replace SigmaOS stubs with real codebases

Write-Host "Fetching musl libc for POSIX compatibility layer..."
if (-not (Test-Path "lib\musl")) {
    git clone git://git.musl-libc.org/musl lib\musl
} else {
    Write-Host "musl libc already exists."
}

Write-Host "Fetching Mesa for graphics rendering..."
if (-not (Test-Path "lib\mesa")) {
    git clone https://gitlab.freedesktop.org/mesa/mesa.git lib\mesa
} else {
    Write-Host "Mesa already exists."
}

Write-Host "Fetching Wayland protocols..."
if (-not (Test-Path "lib\wayland")) {
    git clone https://gitlab.freedesktop.org/wayland/wayland.git lib\wayland
} else {
    Write-Host "Wayland already exists."
}

Write-Host "Fetching Linux kernel (drivers subset) for Ext4/Btrfs/GPU..."
if (-not (Test-Path "lib\linux-drivers")) {
    # Using a shallow clone of the linux kernel just for drivers
    git clone --depth 1 https://github.com/torvalds/linux.git lib\linux-drivers
} else {
    Write-Host "Linux driver tree already exists."
}

Write-Host "Successfully fetched upstream dependencies!"
